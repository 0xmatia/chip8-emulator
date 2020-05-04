use crate::chip8;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct InputDevice {
    events: sdl2::EventPump,
}

impl InputDevice {
    pub fn new(sdl_context: &sdl2::Sdl) -> InputDevice {
        InputDevice {
            events: sdl_context
                .event_pump()
                .expect("Something went wrong with the event pump"),
        }
    }
    // handles the input from the window. returns true of needs to exit
    pub fn handle_input(&mut self, chip8: &mut chip8::Chip8) -> bool{
        for i in &mut chip8.keyboard {
            *i = false;
        }

        for event in self.events.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return true
                },
                _ => {}
            }
        }
        let keys: Vec<Keycode> = self
            .events
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();
        // https://github.com/starrhorne/chip8-rust/blob/master/src/drivers/input_driver.rs
        for key in keys {
            let index = match key {
                Keycode::Num1 => Some(0x1),
                Keycode::Num2 => Some(0x2),
                Keycode::Num3 => Some(0x3),
                Keycode::Num4 => Some(0xc),
                Keycode::Q => Some(0x4),
                Keycode::W => Some(0x5),
                Keycode::E => Some(0x6),
                Keycode::R => Some(0xd),
                Keycode::A => Some(0x7),
                Keycode::S => Some(0x8),
                Keycode::D => Some(0x9),
                Keycode::F => Some(0xe),
                Keycode::Z => Some(0xa),
                Keycode::X => Some(0x0),
                Keycode::C => Some(0xb),
                Keycode::V => Some(0xf),
                _ => None,
            };
            if let Some(i) = index {
                chip8.keyboard[i] = true;
            }
        }
        false
    }
}
