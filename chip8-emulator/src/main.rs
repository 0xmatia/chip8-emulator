mod chip8;
use chip8::Chip8;
use std::process;

fn main() {
    println!("CHIP-8 emulator starting...");
    // create chip8 instance
    let mut chip8 = Chip8::new();

    if let Err(e) = chip8.load_rom("Roms\\PONG") {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
    chip8.print_memory(0x200); // print memory from 0x200
}   
