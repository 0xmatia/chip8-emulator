use sdl2;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::Rect;

const SCREEN_WIDTH: usize = 1280;
const SCREEN_HEIGH: usize = 640;
const PIXEL_SIZE: usize = 20;

pub struct Display {
    // the canvas I will be drawing to
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub event_pump: sdl2::EventPump

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
            canvas: window.into_canvas().build().expect("Failed to build canvas"),
            event_pump: sdl.event_pump().unwrap(),
        }
    }
}

