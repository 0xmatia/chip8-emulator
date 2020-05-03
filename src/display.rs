use sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use crate::chip8::{WIDTH, HEIGHT};

const SCREEN_WIDTH: usize = 1280;
const SCREEN_HEIGH: usize = 640;
const SCALE: usize = 20;

pub struct Display {
    // the canvas I will be drawing to
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub event_pump: sdl2::EventPump,
}

impl Display {
    pub fn initialize() -> Display {
        let sdl = sdl2::init().expect("Failed to initialize display!");
        let video_subsystem = sdl.video().expect("Failed to load video subsystem.");
        let window = video_subsystem
            .window("Chip-8 Emulator", SCREEN_WIDTH as u32, SCREEN_HEIGH as u32)
            .resizable()
            .build()
            .expect("Failed to build window");
        Display {
            canvas: window
                .into_canvas()
                .build()
                .expect("Failed to build canvas"),
            event_pump: sdl.event_pump().unwrap(),
        }
    }
    pub fn draw(&mut self, display: &[[u8; WIDTH]; HEIGHT]) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                //println!("x:{} y:{} state: {}", y, x, display[y][x]);
                if display[y][x] == 1 {
                    self.canvas.fill_rect(Rect::new(
                        x as i32 * SCALE as i32,
                        y as i32 * SCALE as i32,
                        20,
                        20,
                    ))?;
                }
            }
        }
        self.canvas.present();
        Ok(())
    }
}
