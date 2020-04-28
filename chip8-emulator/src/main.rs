mod chip8;
use chip8::Chip8;
use std::process;
use std::io;

fn main() {
    println!("CHIP-8 emulator starting...");
    // create chip8 instance
    let mut chip8 = Chip8::new();

    if let Err(e) = chip8.load_rom("Roms\\GUESS") {
        eprintln!("Couldn't load ROM: {}", e);

        process::exit(1);
    }
    //chip8.print_memory(0x200); // print memory from 0x200
    loop {
        // execute a cpu cycle (one instruction)
        if let Err(e) = chip8.cycle() {
            eprintln!("Execution error: {}", e)
        }
        //Wait for input to proceed to the next cycle
        let mut input = String::from("");
        io::stdin().read_line(&mut input)
        .ok()
        .expect("Couldn't read line");   
    }
}  
