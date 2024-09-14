use std::{collections::VecDeque, time::Instant};

use color_eyre::eyre::{self, eyre};
use rbca_core::{
    Button::{self, Down, Left, Right, Select, Start, Up, A, B},
    Cpu, EmuState, DISPLAY_HEIGHT, DISPLAY_WIDTH,
};
use sdl2::{
    event::Event, keyboard::Scancode, rect::Rect, render::Canvas, video::Window, EventPump,
};
use text_io::read;

use super::{config::UserConfig, palette::hex_to_sdl};

const SCALE: u32 = 5;

const WINDOW_WIDTH: u32 = (DISPLAY_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (DISPLAY_HEIGHT as u32) * SCALE;

pub struct Emulator<'a> {
    cpu: Cpu,
    canvas: Canvas<Window>,
    event_pump: EventPump,
    config: &'a UserConfig,
}
impl<'a> Emulator<'a> {
    pub fn new(cpu: Cpu, config: &'a UserConfig) -> eyre::Result<Self> {
        let sdl_context = match sdl2::init() {
            Ok(sdlc) => sdlc,
            Err(e) => return Err(eyre!(e)),
        };
        let video_subsystem = match sdl_context.video() {
            Ok(vs) => vs,
            Err(e) => return Err(eyre!(e)),
        };
        let window = video_subsystem
            .window("rgba", WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .opengl()
            .build()?;
        let mut canvas = window.into_canvas().present_vsync().build()?;

        canvas.clear();
        canvas.present();

        let event_pump = match sdl_context.event_pump() {
            Ok(ep) => ep,
            Err(e) => return Err(eyre!(e)),
        };

        Ok(Self {
            cpu,
            canvas,
            event_pump,
            config,
        })
    }

    pub fn run(&mut self) -> eyre::Result<()> {
        let mut cycles: u128 = 0;
        let mut frame_count: u128 = 0;
        let mut last_frame_time = Instant::now();
        let mut history: VecDeque<EmuState> = VecDeque::with_capacity(self.config.history());
        let mut step_forward: bool = false;

        'main_loop: loop {
            let start = Instant::now();

            // Read key events
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        scancode: Some(Scancode::Escape),
                        ..
                    } => {
                        break 'main_loop;
                    }
                    Event::KeyDown {
                        scancode: Some(sc), ..
                    } => {
                        if let Some(btn) = match_scancode_button(self.config, &sc) {
                            self.cpu.button_down(btn, self.config.btn_debug());
                        }
                    }
                    Event::KeyUp {
                        scancode: Some(sc), ..
                    } => {
                        if let Some(btn) = match_scancode_button(self.config, &sc) {
                            self.cpu.button_up(btn, self.config.btn_debug());
                        }
                    }
                    _ => {}
                }
            }

            // Cycle CPU
            let cycles_and_state = self
                .cpu
                .cycle(self.config.instr_debug(), self.config.breakpoints_enabled());
            cycles += cycles_and_state.0 as u128;

            // Match with breakpoint & add to history
            if let Some(emu_state) = cycles_and_state.1 {
                step_forward = self.match_breakpoint(&emu_state, &history, step_forward);
                if history.len() >= self.config.history() {
                    history.pop_front();
                }
                history.push_back(emu_state);
            }

            // Approximately one frame
            // TODO make this less hack-y
            if cycles >= 70224 {
                frame_count += 1;
                if self.config.fps_debug() > 0
                    && frame_count % (self.config.fps_debug() as u128) == 0
                {
                    println!("{:.0}", 1_f64 / last_frame_time.elapsed().as_secs_f64());
                }

                cycles %= 70224;
                self.draw_screen()?;
                last_frame_time = Instant::now();
                // Wait until can start next frame
                if self.config.general_debug() && start.elapsed().as_secs_f64() >= (1.0 / 59.0) {
                    eprintln!("Warning: emulator is running slower than it should!");
                }
                while start.elapsed().as_nanos() < 16_750_000 {}
            }
        }
        Ok(())
    }

    fn draw_screen(&mut self) -> eyre::Result<()> {
        // Clear canvas
        self.canvas
            .set_draw_color(hex_to_sdl(self.config.palette().lightest()));
        self.canvas.clear();

        for (i, pixel) in self.cpu.get_pixels().iter().enumerate() {
            self.canvas
                .set_draw_color(hex_to_sdl(self.config.palette().num_to_hex(*pixel)));
            let x = (i % DISPLAY_WIDTH) as u32;
            let y = (i / DISPLAY_WIDTH) as u32;
            let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
            self.canvas.fill_rect(rect).unwrap();
        }

        self.canvas.present();
        Ok(())
    }

    fn match_breakpoint(
        &self,
        emu_state: &EmuState,
        history: &VecDeque<EmuState>,
        step_forward: bool,
    ) -> bool {
        let byte_2_compare = ((emu_state.byte_1 as u16) << 8) | (emu_state.byte_0 as u16);
        let byte_3_compare = ((emu_state.byte_2 as u32) << 8) | (byte_2_compare as u32);

        if step_forward
            || self
                .config
                .breakpoints()
                .program_counter
                .contains(&emu_state.pc)
            || self
                .config
                .breakpoints()
                .opcode_1_byte
                .contains(&emu_state.byte_0)
            || self
                .config
                .breakpoints()
                .opcode_2_byte
                .contains(&byte_2_compare)
            || self
                .config
                .breakpoints()
                .opcode_3_byte
                .contains(&byte_3_compare)
            || self.config.breakpoints().a_reg.contains(&emu_state.a_reg)
            || self.config.breakpoints().b_reg.contains(&emu_state.b_reg)
            || self.config.breakpoints().c_reg.contains(&emu_state.c_reg)
            || self.config.breakpoints().d_reg.contains(&emu_state.d_reg)
            || self.config.breakpoints().e_reg.contains(&emu_state.e_reg)
            || self.config.breakpoints().h_reg.contains(&emu_state.h_reg)
            || self.config.breakpoints().l_reg.contains(&emu_state.l_reg)
        {
            if !step_forward {
                for hist_item in history.iter().rev() {
                    println!("{hist_item}");
                }
            }
            println!("{emu_state}");
            println!("-------");
            if step_forward {
                println!(" - STEP FORWARD - ");
            } else {
                println!(" - BREAK - ");
            }
            loop {
                println!(
                    "Enter '{}' to step forward, '{}' to continue...",
                    self.config.step_forward_key(),
                    self.config.continue_key()
                );
                let input: String = read!();
                if !input.is_empty() {
                    if input.chars().nth(0).unwrap().to_lowercase().next()
                        == self.config.step_forward_key().to_lowercase().next()
                    {
                        return true;
                    }
                    if input.chars().nth(0).unwrap().to_lowercase().next()
                        == self.config.continue_key().to_lowercase().next()
                    {
                        return false;
                    }
                }
            }
        }
        false
    }
}

fn match_scancode_button(config: &UserConfig, sc: &Scancode) -> Option<Button> {
    if sc == config.up_code() {
        Some(Up)
    } else if sc == config.down_code() {
        Some(Down)
    } else if sc == config.left_code() {
        Some(Left)
    } else if sc == config.right_code() {
        Some(Right)
    } else if sc == config.a_code() {
        Some(A)
    } else if sc == config.b_code() {
        Some(B)
    } else if sc == config.start_code() {
        Some(Start)
    } else if sc == config.select_code() {
        Some(Select)
    } else {
        None
    }
}
