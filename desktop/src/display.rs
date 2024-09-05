use color_eyre::eyre::{self, eyre};
use rbca_core::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window,
    EventPump, Sdl, VideoSubsystem,
};

const SCALE: u32 = 5;

const WINDOW_WIDTH: u32 = (DISPLAY_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (DISPLAY_HEIGHT as u32) * SCALE;

pub struct Display {
    sdl_context: Sdl,
    video_subsystem: VideoSubsystem,
    canvas: Canvas<Window>,
    event_pump: EventPump,
    // 3 bytes per pixel; RGB24 encoding
    framebuffer: [u8; 3 * DISPLAY_WIDTH * DISPLAY_HEIGHT],
    // 4 bytes per pixel; RGBA24 encoding
    framebuffer_a: [u8; 4 * DISPLAY_WIDTH * DISPLAY_HEIGHT],
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
            framebuffer: [0x00; 3 * DISPLAY_WIDTH * DISPLAY_HEIGHT],
            framebuffer_a: [0x00; 4 * DISPLAY_WIDTH * DISPLAY_HEIGHT],
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
                    _ => {}
                }
            }
            self.draw_screen()?;
        }
        Ok(())
    }

    fn draw_screen(&mut self) -> eyre::Result<()> {
        // Clear canvas
        self.canvas.set_draw_color(Color::RGB(0xB8, 0xD7, 0x81));
        self.canvas.clear();

        // TODO get display here
        // for (i, pixel) in self.display_arr.iter().enumerate() {
        //     if *pixel {
        //         // Convert index to 2D [x,y] position
        //         let x = (i % DISPLAY_WIDTH) as u32;
        //         let y = (i / DISPLAY_HEIGHT) as u32;
        //
        //         // Draw scaled-up rectangle @ [x,y]
        //         let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
        //         self.canvas.fill_rect(rect).unwrap();
        //     }
        // }
        self.canvas.present();

        Ok(())
    }
}
