use rand::Rng;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::{thread, time};

const RAM_SIZE: usize = 4096;
const NUM_REGISTERS: usize = 16;
const STACK_SIZE: usize = 16;
const WIDTH: usize = 64;
const HEIGHT: usize = 32;

const CHIP8_FONTS: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

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
    // Display
    display: [u8; WIDTH * HEIGHT],

    keyboard: [bool; 16],

    // STACK
    stack: [u16; STACK_SIZE],

    //STACK pointer
    sp: u8,

    // random number handler
    rng: rand::rngs::ThreadRng,
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
        let mut ram = [0u8; RAM_SIZE];
        for i in 0..CHIP8_FONTS.len() {
            ram[i] = CHIP8_FONTS[i];
        }

        Chip8 {
            // FIll memory with zeros
            memory: ram,
            // execution starts at 0x200
            pc: 0x200,
            // FILL registers with 0
            v: [0; NUM_REGISTERS],
            i: 0x0,
            display: [0; WIDTH * HEIGHT],
            keyboard: [false; 16],
            sound_timer: 0,
            delay_timer: 0,
            stack: [0; STACK_SIZE],
            sp: 0,
            rng: rand::thread_rng(),
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
        // io::stdin()
        //     .read_line(&mut input)
        //     .ok()
        //     .expect("Couldn't read line");

        match nibbles {
            // Clear dispaly
            (0x0, 0x0, 0xE, 0x0) => self.op_00e0(),
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
            // subbstructs vy from vx
            (0x8, _, _, 0x5) => self.op_8xy5(x, y),
            // shift right
            (0x8, _, _, 0x6) => self.op_8xy6(x, y),
            // substrcut vy from vx store in vx
            (0x8, _, _, 0x7) => self.op_8xy7(x, y),
            // left shift (multiply by two)
            (0x8, _, _, 0xE) => self.op_8xye(x, y),
            // skip next instruction if vx!=vy
            (0x9, _, _, 0x0) => self.op_9xy0(x, y),
            // set I to nnn
            (0xA, _, _, _) => self.op_annn(nnn),
            // set B to nnn + v0
            (0xB, _, _, _) => self.op_bnnn(nnn),
            // generate random number
            (0xC, _, _, _) => self.op_cxkk(x, kk),
            // draw to screen
            (0xD, _, _, _) => self.op_dxyn(x, y, n),
            _ => return Err(format!("Unknown intruction: {:#06X}", opcode)),
        }
        Ok(())
    }

    // clear display
    fn op_00e0(&mut self) {
        for elem in self.display.iter_mut() { *elem = 0; }
        self.pc += 2;
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
        self.stack[self.sp as usize] = self.pc + 2; // push the next instruction
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
            self.pc += 0x4; //skip additional 2bytes!
        } else {
            // increment the program counter
            self.pc += 0x2;
        }
    }

    // compare vx to xx and increment pc by two if they are not equal (+=2);
    fn op_4xkk(&mut self, x: u8, kk: u8) {
        if self.v[x as usize] != kk {
            self.pc += 4; //skip additional 2bytes!
        } else {
            // increment the program counter
            self.pc += 0x2;
        }
    }

    // compare vx and vy and increment pc by two if they are equal (+=2);
    fn op_5xy0(&mut self, x: u8, y: u8) {
        if self.v[x as usize] == self.v[y as usize] {
            self.pc += 4; //skip additional 2bytes!
        } else {
            // increment the program counter
            self.pc += 0x2;
        }
    }

    // set v[x] to kk
    fn op_6xkk(&mut self, x: u8, kk: u8) {
        self.v[x as usize] = kk;
        // increment the program counter
        self.pc += 0x2;
    }

    // add kk to v[x]
    fn op_7xkk(&mut self, x: u8, kk: u8) {
        self.v[x as usize] = self.v[x as usize] + kk;
        // increment the program counter
        self.pc += 0x2;
    }

    // store vy in vx
    fn op_8xy0(&mut self, x: u8, y: u8) {
        self.v[x as usize] = self.v[y as usize];
        // increment the program counter
        self.pc += 0x2;
    }

    // set vx = vx | vy
    fn op_8xy1(&mut self, x: u8, y: u8) {
        self.v[x as usize] = self.v[x as usize] | self.v[y as usize];
        // increment the program counter
        self.pc += 0x2;
    }

    // set vx = vx & vy
    fn op_8xy2(&mut self, x: u8, y: u8) {
        self.v[x as usize] = self.v[x as usize] & self.v[y as usize];
        // increment the program counter
        self.pc += 0x2;
    }

    // set vx = vx & vy
    fn op_8xy3(&mut self, x: u8, y: u8) {
        self.v[x as usize] = self.v[x as usize] ^ self.v[y as usize];
        // increment the program counter
        self.pc += 0x2;
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
        // increment the program counter
        self.pc += 0x2;
    }

    // substructs vy from vx. if vx > vy vf is set to one.
    fn op_8xy5(&mut self, x: u8, y: u8) {
        if self.v[x as usize] > self.v[y as usize] {
            self.v[0xF] = 0x1;
        } else {
            self.v[0xF] = 0x0;
        }
        self.v[x as usize] -= self.v[y as usize];
        // increment the program counter
        self.pc += 0x2;
    }

    // shift right. if lsb of vx is 1, carry flag is turned on
    fn op_8xy6(&mut self, x: u8, _y: u8) {
        let lsb = self.v[x as usize] & 0x1; // extract the lsb
        self.v[0xf] = lsb;
        self.v[x as usize] >>= 1;
        // increment the program counter
        self.pc += 0x2;
    }

    // substract. if vy > vx vf is set to one.
    fn op_8xy7(&mut self, x: u8, y: u8) {
        if self.v[y as usize] > self.v[x as usize] {
            self.v[0xF] = 0x1;
        } else {
            self.v[0xF] = 0x0;
        }
        self.v[x as usize] = self.v[y as usize] - self.v[x as usize];
        // increment the program counter
        self.pc += 0x2;
    }

    // shift right. if msb of vx is 1, carry flag is turned on
    fn op_8xye(&mut self, x: u8, _y: u8) {
        let msb = (self.v[x as usize] >> 7) & 0x1; // extract the msb
        self.v[0xf] = msb;
        self.v[x as usize] <<= 1;
        // increment the program counter
        self.pc += 0x2;
    }

    // if the two registers aren't equal, skip the next instruction
    fn op_9xy0(&mut self, x: u8, y: u8) {
        if self.v[x as usize] != self.v[y as usize] {
            self.pc += 4; //skip additional 2bytes!
        } else {
            // increment the program counter
            self.pc += 0x2;
        }
    }

    // set i = nnn
    fn op_annn(&mut self, nnn: u16) {
        self.i = nnn;
        self.pc += 2;
    }

    // set pc = nnn + v0
    fn op_bnnn(&mut self, nnn: u16) {
        self.pc = nnn + (self.v[0x0] as u16);
    }

    // set vx = random number & kk
    fn op_cxkk(&mut self, x: u8, kk: u8) {
        let random_number: u16 = self.rng.gen_range(0, 256);
        self.v[x as usize] = random_number as u8 & kk;
        self.pc += 2;
    }

    fn op_dxyn(&mut self, x: u8, y: u8, n: u8) {
        let mut pixel: u8;
        self.v[0xF] = 0;
        for yline in 0..n {
            pixel = self.memory[self.i as usize + yline as usize];
            for xline in 0..8 {
                if (pixel & (0x80 >> xline)) != 0 {
                    print!("*");
                    if self.display[(x + xline + ((y + yline) * 64)) as usize] == 1 {
                        self.v[0xF] = 1;
                    }
                    self.display[(x + xline + ((y + yline) * 64)) as usize] ^= 1;
                }
                print!("_");
            }
            println!("\n");
        }
        println!("\n");
        self.pc += 2;
    }
}

#[cfg(test)]
#[path = "./opcode_tests.rs"]
mod opcode_tests;
