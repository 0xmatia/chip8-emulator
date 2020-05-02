mod chip8;
use chip8::{Chip8, WIDTH, HEIGHT};
use std::process;
use std::{thread, time};
use std::io;


fn main() {
    println!("CHIP-8 emulator starting...");
    // create chip8 instance
    let mut chip8 = Chip8::new();

    if let Err(e) = chip8.load_rom("Roms\\BRIX") {
        eprintln!("Couldn't load ROM: {}", e);

        process::exit(1);
    }
    //chip8.print_memory(0x200); // print memory from 0x200
    loop {
        // execute a cpu cycle (one instruction)
        if let Err(e) = chip8.cycle() {
            eprintln!("Execution error: {}", e)
        }
        if chip8.sound_timer > 0 {
            println!("BEEP!");
        }
        if chip8.draw {
            for yline in 0..HEIGHT {
                for xline in 0..WIDTH {
                    let pixel = chip8.display[yline][xline];
                    if pixel == 1 {
                        print!("*");
                    }
                    else {
                        print!("-");
                    }
                }
                println!();
            }
            chip8.draw = false;
            //Wait for input to proceed
            let mut input = String::from("");
            io::stdin()
                .read_line(&mut input)
                .ok()
                .expect("Couldn't read line");
            // let ten_millis = time::Duration::from_millis(100);
            // thread::sleep(ten_millis);
        }
    }
}  