use std::fs;
use std::error::Error;
use std::{thread, time};

const RAM_SIZE: usize = 4096;
const NUM_REGISTERS: usize = 16;
const STACK_SIZE: usize = 16;

pub struct Chip8 {
    // Memory: 4kb of 8 bits(byte)
    memory: [u8; RAM_SIZE],
    // Pc: the program counter, 16 bits but uses only 12 (max address is 0xfff)
    pc: u16,
    // General purpose registers
    v: [u8; NUM_REGISTERS],
    // I register: 16 bit memory holder
    i: u16,
    //Two timers, they are not implemented yet.
    sound_timer: u16,

    delay_timer: u16,
    // Display + keyboard: tbc

    // STACK
    stack: [u16; STACK_SIZE],

    //STACK pointer
    sp: u8,
}

impl Chip8 {
    // This function creates a new instance of the chip struct
    pub fn new() -> Chip8 {
        Chip8 {
            // FIll memory with zeros
            memory: [0; RAM_SIZE],
            // execution starts at 0x200
            pc: 0x200,
            // FILL registers with 0
            v: [0; NUM_REGISTERS],
            i: 0x0,
            sound_timer: 0,
            delay_timer: 0,
            stack: [0; STACK_SIZE],
            sp: 0
        }
    }
    // This function loads a rom to memory
    pub fn load_rom(&mut self, rom_path: &str) -> Result<(), Box<dyn Error>> {
        // Read ROM
        let rom_data = fs::read(rom_path)?;
        let mut memory_pointer = 0x200;
     
        for &byte in rom_data.iter() {
            self.memory[memory_pointer] = byte;
            memory_pointer += 1;
        }
        Ok(())
    }

    //For debuging, this function will print memory from the requested location
    pub fn print_memory(&mut self, start_index: u16) {
        for pointer in start_index..RAM_SIZE as u16 {
            println!("{:#X}: {:#04X}", pointer, self.memory[pointer as usize]);
            thread::sleep(time::Duration::from_millis(10));
            
        }
    }

    pub fn cycle(&mut self) -> Result<(), String> {
        // Decode the opcode:
        let hi = self.memory[self.pc as usize] as u16;
        let lo = self.memory[(self.pc + 1) as usize] as u16;
        let opcode = (hi << 8) | lo;
        println!("Opcode: {:#06X}", opcode);
        
        // increment the program counter
        self.pc += 2;

        // extract data from the opcode: kk, nnn, n, x, y
        

        Ok(())
    }
}

