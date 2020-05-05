mod chip8;
mod display;
mod keyboard;
mod beep;
use chip8::{Chip8};
use std::process;
use std::time::Duration;
use std::env;

fn main() {
    println!("CHIP-8 emulator starting...");
    let args: Vec<String> = env::args().collect();
    let mut display = display::Display::initialize();
    let mut keyboard = keyboard::InputDevice::new(&display.context);
    let audio = beep::AudioDevice::new(&display.context);
    
    // create chip8 instance
    let mut chip8 = Chip8::new();
    let rom = &args[1];
    if let Err(e) = chip8.load_rom(rom) {
        eprintln!("Couldn't load ROM: {}", e);

        process::exit(1);
    }

    'main: loop {
        if keyboard.handle_input(&mut chip8) {
            break 'main;
        }
        // execute a cpu cycle (one instruction)
        if let Err(e) = chip8.cycle() {
            eprintln!("Execution error: {}", e)
        }

        if chip8.draw {
            if let Err(e) = display.draw(&chip8.display) {
                println!("Draw error: {}", e);
            }
            chip8.draw = false;
        }

        if chip8.sound_timer > 0 { 
            audio.beep();
        }
        else {
            audio.stop_beep();
        }

        ::std::thread::sleep(Duration::from_millis(2));
    }
}  