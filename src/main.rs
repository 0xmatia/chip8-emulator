mod chip8;
mod display;
mod keyboard;
use chip8::{Chip8};
use std::process;
use std::time::Duration;


fn main() {
    println!("CHIP-8 emulator starting...");
    let mut display = display::Display::initialize();
    let mut keyboard = keyboard::InputDevice::new(&display.context);
    
    // create chip8 instance
    let mut chip8 = Chip8::new();
    if let Err(e) = chip8.load_rom("Roms\\TICTAC") {
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

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 120));
    }
}  