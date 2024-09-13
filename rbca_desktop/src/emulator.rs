use std::time::Instant;

use color_eyre::eyre::{self, eyre};
use rbca_core::{
    Button::{self, Down, Left, Right, Select, Start, Up, A, B},
    Cpu, DISPLAY_HEIGHT, DISPLAY_WIDTH,
};
use sdl2::{
    event::Event, keyboard::Scancode, rect::Rect, render::Canvas, video::Window, EventPump,
};

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
            cycles += self.cpu.cycle(self.config.instr_debug()) as u128;

            // Approximately one frame
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
                if start.elapsed().as_nanos() < 16_750_000 {}
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
