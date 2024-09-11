use std::time::Instant;

use color_eyre::eyre::{self, eyre};
use rbca_core::{Cpu, DISPLAY_HEIGHT, DISPLAY_WIDTH};
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window,
    EventPump,
};

use super::INSTR_DEBUG;

const SCALE: u32 = 5;

const WINDOW_WIDTH: u32 = (DISPLAY_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (DISPLAY_HEIGHT as u32) * SCALE;

// 0
const WHITE: (u8, u8, u8) = (0xD7, 0xEC, 0xA1);
// 1
const LIGHT_GREY: (u8, u8, u8) = (0xA6, 0xBB, 0x72);
// 2
const DARK_GREY: (u8, u8, u8) = (0x6C, 0x7D, 0x41);
// 3
const BLACK: (u8, u8, u8) = (0x3B, 0x46, 0x20);

pub struct Display {
    cpu: Cpu,
    canvas: Canvas<Window>,
    event_pump: EventPump,
}
impl Display {
    pub fn new(cpu: Cpu) -> eyre::Result<Self> {
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
        })
    }

    pub fn run(&mut self) -> eyre::Result<()> {
        let mut cycles: u128 = 0;
        'main_loop: loop {
            let start = Instant::now();

            // Read key events
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        break 'main_loop;
                    }
                    _ => {}
                }
            }

            // Cycle CPU
            cycles += self.cpu.cycle(INSTR_DEBUG) as u128;

            // Approximately one frame
            if cycles >= 70224 {
                cycles %= 70224;
                self.draw_screen()?;
                // Wait until can start next frame
                if start.elapsed().as_nanos() < 16_750_000 {}
            }
        }
        Ok(())
    }

    fn draw_screen(&mut self) -> eyre::Result<()> {
        // Clear canvas
        self.canvas
            .set_draw_color(Color::RGB(WHITE.0, WHITE.1, WHITE.2));
        self.canvas.clear();

        for (i, pixel) in self.cpu.get_pixels().iter().enumerate() {
            let colour = match pixel {
                0 => continue,
                1 => LIGHT_GREY,
                2 => DARK_GREY,
                3 => BLACK,
                _ => unreachable!("Illegal colour value."),
            };
            self.canvas
                .set_draw_color(Color::RGB(colour.0, colour.1, colour.2));
            let x = (i % DISPLAY_WIDTH) as u32;
            let y = (i / DISPLAY_WIDTH) as u32;
            let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
            self.canvas.fill_rect(rect).unwrap();
        }

        self.canvas.present();
        Ok(())
    }
}
