use color_eyre::eyre::{self, eyre};
use rbca_core::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window,
    EventPump, Sdl, VideoSubsystem,
};

const BG_RGB: (u8, u8, u8) = (0xBE, 0xCE, 0x9A);
const FG_RGB: (u8, u8, u8) = (0x31, 0x3D, 0x19);

const SCALE: u32 = 5;

const WINDOW_WIDTH: u32 = (DISPLAY_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (DISPLAY_HEIGHT as u32) * SCALE;

pub struct Display {
    sdl_context: Sdl,
    video_subsystem: VideoSubsystem,
    canvas: Canvas<Window>,
    event_pump: EventPump,
    display_arr: [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT],
    // TODO test
    px_coords: (u32, u32),
}
impl Display {
    pub fn new() -> eyre::Result<Self> {
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
            sdl_context,
            video_subsystem,
            canvas,
            event_pump,
            display_arr: [false; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            // TODO test
            px_coords: (DISPLAY_WIDTH as u32 / 2, DISPLAY_HEIGHT as u32 / 2),
        })
    }

    pub fn run(&mut self) -> eyre::Result<()> {
        'main_loop: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        break 'main_loop;
                    }
                    // TODO test
                    Event::KeyDown {
                        keycode: Some(Keycode::W),
                        ..
                    } => {
                        self.px_coords = (self.px_coords.0, self.px_coords.1 - 1);
                    }
                    // TODO test
                    Event::KeyDown {
                        keycode: Some(Keycode::S),
                        ..
                    } => {
                        self.px_coords = (self.px_coords.0, self.px_coords.1 + 1);
                    }
                    // TODO test
                    Event::KeyDown {
                        keycode: Some(Keycode::A),
                        ..
                    } => {
                        self.px_coords = (self.px_coords.0 - 1, self.px_coords.1);
                    }
                    // TODO test
                    Event::KeyDown {
                        keycode: Some(Keycode::D),
                        ..
                    } => {
                        self.px_coords = (self.px_coords.0 + 1, self.px_coords.1);
                    }
                    _ => {}
                }
            }
            self.draw_screen()?;
        }
        Ok(())
    }

    fn draw_screen(&mut self) -> eyre::Result<()> {
        // Clear canvas
        self.canvas
            .set_draw_color(Color::RGB(BG_RGB.0, BG_RGB.1, BG_RGB.2));
        self.canvas.clear();

        // TODO test
        for i in 0..(DISPLAY_HEIGHT * DISPLAY_WIDTH) {
            let x = (i % DISPLAY_WIDTH) as u32;
            let y = (i / DISPLAY_HEIGHT) as u32;
            self.display_arr[i] = (x == self.px_coords.0) && (y == self.px_coords.1);
        }

        // TODO get display here
        self.canvas
            .set_draw_color(Color::RGB(FG_RGB.0, FG_RGB.1, FG_RGB.2));
        for (i, pixel) in self.display_arr.iter().enumerate() {
            if *pixel {
                // Convert index to 2D [x,y] position
                let x = (i % DISPLAY_WIDTH) as u32;
                let y = (i / DISPLAY_HEIGHT) as u32;

                // Draw scaled-up rectangle @ [x,y]
                let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
                self.canvas.fill_rect(rect).unwrap();
            }
        }
        self.canvas.present();

        Ok(())
    }
}
