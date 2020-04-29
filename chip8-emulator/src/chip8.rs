use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
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

impl fmt::Display for Chip8 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "(PC: {:#06X}, I: {:#06X}, Stack: {:X?}, Vs: {:X?})",
            self.pc, self.i, self.stack, self.v
        )
    }
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
            sp: 0,
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
        let nibbles = (
            (opcode & 0xF000) >> 12 as u8,
            (opcode & 0x0F00) >> 8 as u8,
            (opcode & 0x00F0) >> 4 as u8,
            (opcode & 0x000F) as u8,
        );

        // extract data from the opcode: kk, nnn, n, nn x, y
        let nnn = opcode & 0x0FFF;
        let kk: u8 = (opcode & 0x00FF) as u8; // or nn
        let n: u8 = (opcode & 0x000F) as u8;
        let x: u8 = ((opcode >> 8) & 0x000F) as u8;
        let y: u8 = ((opcode >> 4) & 0x000F) as u8;
        println!("\nOpcode: {:#06X}", opcode);
        println!(
            "nnn: {:#05X}; kk: {:#04X}; n: {:#03X}; x: {:#03X}; y: {:#03X}",
            nnn, kk, n, x, y
        );
        println!("State: {}", self);
        //Wait for input to proceed
        let mut input = String::from("");
        io::stdin()
            .read_line(&mut input)
            .ok()
            .expect("Couldn't read line");
        // increment the program counter
        self.pc += 0x2;

        match nibbles {
            // Clear dispaly
            (0x0, 0x0, 0xE, 0x0) => {
                return Err(format!("Opcode not implemented yet: {:#06X}", opcode))
            }
            // RET - return from subroutine
            (0x0, 0x0, 0xE, 0xE) => self.op_00ee()?,
            // 1nnn: sets pc to nnn
            (0x1, _, _, _) => self.op_1nnn(nnn),
            //call - push pc to stack and jump to nnn
            (0x2, _, _, _) => self.op_2nnn(nnn)?,
            // skip next intruction if vx == kk
            (0x3, _, _, _) => self.op_3xkk(x, kk),
            // skip next intruction if vx != kk
            (0x4, _, _, _) => self.op_4xkk(x, kk),
            // skip next intruction if vx == vy
            (0x5, _, _, 0x0) => self.op_5xy0(x, y),
            // sets vx to kk
            (0x6, _, _, _) => self.op_6xkk(x, kk),
            // adds kk v[x], store in v[x]
            (0x7, _, _, _) => self.op_7xkk(x, kk),
            // sets v[x] = v[y]
            (0x8, _, _, 0x0) => self.op_8xy0(x, y),
            // sets v[x] = v[x] | v[y]
            (0x8, _, _, 0x1) => self.op_8xy1(x, y),
            // sets v[x] = v[x] & v[y]
            (0x8, _, _, 0x2) => self.op_8xy2(x, y),
            // sets v[x] = v[x] ^ v[y]
            (0x8, _, _, 0x3) => self.op_8xy3(x, y),
            // adds vx and vy
            (0x8, _, _, 0x4) => self.op_8xy4(x, y),
            _ => return Err(format!("Unknown intruction: {:#06X}", opcode)),
        }
        Ok(())
    }

    // sets pc to whatever nnn is
    fn op_1nnn(&mut self, nnn: u16) {
        self.pc = nnn;
    }

    // pushes pc to the stack, and sets pc to nnn and increment sp
    fn op_2nnn(&mut self, nnn: u16) -> Result<(), String> {
        // check if we are not out of frames
        if self.sp as usize == STACK_SIZE {
            return Err(String::from("Stack is full"));
        }
        self.stack[self.sp as usize] = self.pc; // push
        self.sp += 1; // increment
        self.pc = nnn; // jump
        Ok(())
    }

    // return from subroutine - pop address on stack to pc
    fn op_00ee(&mut self) -> Result<(), String> {
        if self.sp == 0 {
            // it shouldn't happened, just in case
            return Err(String::from("Tried to return to none?"));
        }
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
        Ok(())
    }

    // compare vx to xx and increment pc by two if they are equal (+=2);
    fn op_3xkk(&mut self, x: u8, kk: u8) {
        if self.v[x as usize] == kk {
            self.pc += 2; //skip additional 2bytes!
        }
        // do nothing else!
    }

    // compare vx to xx and increment pc by two if they are not equal (+=2);
    fn op_4xkk(&mut self, x: u8, kk: u8) {
        if self.v[x as usize] != kk {
            self.pc += 2; //skip additional 2bytes!
        }
        // do nothing else!
    }

    // compare vx and vy and increment pc by two if they are equal (+=2);
    fn op_5xy0(&mut self, x: u8, y: u8) {
        if self.v[x as usize] == self.v[y as usize] {
            self.pc += 2; //skip additional 2bytes!
        }
        // do nothing else!
    }

    // set v[x] to kk
    fn op_6xkk(&mut self, x: u8, kk: u8) {
        self.v[x as usize] = kk;
    }

    // add kk to v[x]
    fn op_7xkk(&mut self, x: u8, kk: u8) {
        self.v[x as usize] = self.v[x as usize] + kk;
    }

    // store vy in vx
    fn op_8xy0(&mut self, x: u8, y: u8) {
        self.v[x as usize] = self.v[y as usize];
    }

    // set vx = vx | vy
    fn op_8xy1(&mut self, x: u8, y: u8) {
        self.v[x as usize] = self.v[x as usize] | self.v[y as usize];
    }

    // set vx = vx & vy
    fn op_8xy2(&mut self, x: u8, y: u8) {
        self.v[x as usize] = self.v[x as usize] & self.v[y as usize];
    }

    // set vx = vx & vy
    fn op_8xy3(&mut self, x: u8, y: u8) {
        self.v[x as usize] = self.v[x as usize] ^ self.v[y as usize];
    }
    // adds vx and vy; turns on carry flag if necessery;
    fn op_8xy4(&mut self, x: u8, y: u8) {
        let vx: u16 = self.v[x as usize] as u16;
        let vy: u16 = self.v[y as usize] as u16;
        let result: u16 = vx + vy;
        println!("{:X}", result as u8);
        self.v[x as usize] = result as u8;
        if result > 0x00FF {
            self.v[0xF] = 0x1;
        } else {
            self.v[0xF] = 0x0;
        }
    }
}

#[cfg(test)]
#[path = "./opcode_tests.rs"]
mod opcode_tests;
