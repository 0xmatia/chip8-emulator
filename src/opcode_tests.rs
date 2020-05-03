use super::Chip8;
use assert_hex;

#[test]
fn op_1nnn() {
    let mut instance = Chip8::new();
    instance.memory[0x200] = 0x15;
    instance.memory[0x201] = 0x1F;
    instance.cycle().unwrap();
    assert_hex::assert_eq_hex!(instance.pc, 0x051F);
}

#[test]
fn op_2nnn() {
    let mut instance = Chip8::new();
    instance.memory[0x200] = 0x21;
    instance.memory[0x201] = 0x3F;

    instance.cycle().unwrap();
    assert_hex::assert_eq_hex!(instance.pc, 0x013F);
    assert_hex::assert_eq_hex!(instance.stack[0], 0x202);
    assert_hex::assert_eq_hex!(instance.sp, 0x1);
}

#[test]
fn op_00ee() {
    let mut instance = Chip8::new();
    instance.memory[0x200] = 0x21;
    instance.memory[0x201] = 0x3F;
    instance.cycle().unwrap();

    instance.memory[0x013F] = 0x00;
    instance.memory[0x0140] = 0xEE;

    instance.cycle().unwrap();
    assert_hex::assert_eq_hex!(instance.pc, 0x0202);
    assert_hex::assert_eq_hex!(instance.sp, 0x0);
}

#[test]
fn op_3xkk() {
    let mut instance = Chip8::new();
    instance.v[2] = 0xA;
    instance.memory[0x200] = 0x32;
    instance.memory[0x201] = 0x0A;
    instance.cycle().unwrap();

    assert_hex::assert_eq_hex!(instance.pc, 0x0204);
}

#[test]
fn op_4xkk() {
    let mut instance = Chip8::new();
    instance.v[2] = 0xB;
    instance.memory[0x200] = 0x42;
    instance.memory[0x201] = 0x0A;
    instance.cycle().unwrap();

    assert_hex::assert_eq_hex!(instance.pc, 0x0204);
}

#[test]
fn op_5xy0() {
    let mut instance = Chip8::new();
    instance.v[2] = 0xB;
    instance.v[1] = 0xB;
    instance.memory[0x200] = 0x52;
    instance.memory[0x201] = 0x10;
    instance.cycle().unwrap();

    assert_hex::assert_eq_hex!(instance.pc, 0x0204);
}

#[test]
fn op_6xkk() {
    let mut instance = Chip8::new();
    instance.memory[0x200] = 0x6A;
    instance.memory[0x201] = 0xCA;
    instance.cycle().unwrap();

    assert_hex::assert_eq_hex!(instance.v[0xA], 0x00CA);
}

#[test]
fn op_7xkk() {
    let mut instance = Chip8::new();
    instance.memory[0x200] = 0x6A;
    instance.memory[0x201] = 0xCA;
    instance.cycle().unwrap();

    instance.memory[0x202] = 0x7A;
    instance.memory[0x203] = 0x01;
    instance.cycle().unwrap();

    assert_hex::assert_eq_hex!(instance.v[0xA], 0xCB);
}

#[test]
fn op_8xy0() {
    let mut instance = Chip8::new();
    instance.memory[0x200] = 0x6A;
    instance.memory[0x201] = 0xCA;
    instance.cycle().unwrap();

    instance.memory[0x202] = 0x8A;
    instance.memory[0x203] = 0xE0;
    instance.cycle().unwrap();

    assert_hex::assert_eq_hex!(instance.v[0xA], instance.v[0xE]);
}

#[test]
fn op_8xy1() {
    let mut instance = Chip8::new();
    instance.memory[0x200] = 0x6A;
    instance.memory[0x201] = 0xCA;
    instance.cycle().unwrap();

    instance.memory[0x202] = 0x6B;
    instance.memory[0x203] = 0xCA;
    instance.cycle().unwrap();

    instance.memory[0x204] = 0x8A;
    instance.memory[0x205] = 0xB1;
    instance.cycle().unwrap();

    assert_hex::assert_eq_hex!(instance.v[0xA], 0xCA);
}

#[test]
fn op_8xy2() {
    let mut instance = Chip8::new();
    instance.memory[0x200] = 0x6A;
    instance.memory[0x201] = 0xCA;
    instance.cycle().unwrap();

    instance.memory[0x202] = 0x6B;
    instance.memory[0x203] = 0xCA;
    instance.cycle().unwrap();

    instance.memory[0x204] = 0x8A;
    instance.memory[0x205] = 0xB2;
    instance.cycle().unwrap();

    assert_hex::assert_eq_hex!(instance.v[0xA], 0xCA);
}

#[test]
fn op_8xy3() {
    let mut instance = Chip8::new();
    instance.memory[0x200] = 0x6A;
    instance.memory[0x201] = 0xCA;
    instance.cycle().unwrap();

    instance.memory[0x202] = 0x6B;
    instance.memory[0x203] = 0xCA;
    instance.cycle().unwrap();

    instance.memory[0x204] = 0x8A;
    instance.memory[0x205] = 0xB3;
    instance.cycle().unwrap();

    assert_hex::assert_eq_hex!(instance.v[0xA], 0x0);
}

#[test]
fn op_8xy4() {
    let mut instance = Chip8::new();
    instance.memory[0x200] = 0x6A;
    instance.memory[0x201] = 0x5;
    instance.cycle().unwrap();

    instance.memory[0x202] = 0x6B;
    instance.memory[0x203] = 0x02;
    instance.cycle().unwrap();

    instance.memory[0x204] = 0x8A;
    instance.memory[0x205] = 0xB4;
    instance.cycle().unwrap();
    assert_hex::assert_eq_hex!(instance.v[0xA], 0x07);
}

#[test]
fn op_8xy4_overflow() {
    let mut instance = Chip8::new();
    instance.memory[0x200] = 0x6A;
    instance.memory[0x201] = 0xFF;
    instance.cycle().unwrap();

    instance.memory[0x202] = 0x6B;
    instance.memory[0x203] = 0xCA;
    instance.cycle().unwrap();

    instance.memory[0x204] = 0x8A;
    instance.memory[0x205] = 0xB4;
    instance.cycle().unwrap();
    assert_hex::assert_eq_hex!(instance.v[0xF], 0x1);
    assert_hex::assert_eq_hex!(instance.v[0xA], 0xC9);
}

#[test]
fn op_8xy5() {
    let mut instance = Chip8::new();
    instance.memory[0x200] = 0x6A;
    instance.memory[0x201] = 0xFF;
    instance.cycle().unwrap();

    instance.memory[0x202] = 0x6B;
    instance.memory[0x203] = 0xCA;
    instance.cycle().unwrap();

    instance.memory[0x204] = 0x8A;
    instance.memory[0x205] = 0xB5;
    instance.cycle().unwrap();
    assert_hex::assert_eq_hex!(instance.v[0xF], 0x1);
    assert_hex::assert_eq_hex!(instance.v[0xA], 0x35);
}

#[test]
fn op_8xy6() {
    let mut instance = Chip8::new();
    instance.memory[0x200] = 0x6A;
    instance.memory[0x201] = 0x30;
    instance.cycle().unwrap();

    instance.memory[0x202] = 0x6B;
    instance.memory[0x203] = 0xCA;
    instance.cycle().unwrap();

    instance.memory[0x204] = 0x8A;
    instance.memory[0x205] = 0xB6;
    instance.cycle().unwrap();
    assert_hex::assert_eq_hex!(instance.v[0xF], 0x0);
    assert_hex::assert_eq_hex!(instance.v[0xA], 0x18);
}

#[test]
fn op_8xy7() {
    let mut instance = Chip8::new();
    instance.memory[0x200] = 0x6A;
    instance.memory[0x201] = 0xCA;
    instance.cycle().unwrap();

    instance.memory[0x202] = 0x6B;
    instance.memory[0x203] = 0xFA;
    instance.cycle().unwrap();

    instance.memory[0x204] = 0x8A;
    instance.memory[0x205] = 0xB7;
    instance.cycle().unwrap();
    assert_hex::assert_eq_hex!(instance.v[0xF], 0x1);
    assert_hex::assert_eq_hex!(instance.v[0xA], 0x30);
}

#[test]
fn op_8xye() {
    let mut instance = Chip8::new();
    instance.memory[0x200] = 0x6A;
    instance.memory[0x201] = 0xFF;
    instance.cycle().unwrap();

    instance.memory[0x202] = 0x6B;
    instance.memory[0x203] = 0xCA;
    instance.cycle().unwrap();

    instance.memory[0x204] = 0x8A;
    instance.memory[0x205] = 0xBE;
    instance.cycle().unwrap();
    assert_hex::assert_eq_hex!(instance.v[0xF], 0x1);
    assert_hex::assert_eq_hex!(instance.v[0xA], 0xFE);
}

#[test]
fn op_9xy0() {
    let mut instance = Chip8::new();
    instance.memory[0x200] = 0x6A;
    instance.memory[0x201] = 0xFF;
    instance.cycle().unwrap();

    instance.memory[0x202] = 0x6B;
    instance.memory[0x203] = 0xCA;
    instance.cycle().unwrap();

    instance.memory[0x204] = 0x9A;
    instance.memory[0x205] = 0xB0;
    instance.cycle().unwrap();
    assert_hex::assert_eq_hex!(instance.pc, 0x208);
}

#[test]
fn op_annn() {
    let mut instance = Chip8::new();
    instance.memory[0x200] = 0xAA;
    instance.memory[0x201] = 0xFF;
    instance.cycle().unwrap();

    assert_hex::assert_eq_hex!(instance.i, 0xAFF);
    assert_hex::assert_eq_hex!(instance.pc, 0x202);
}

#[test]
fn op_bnnn() {
    let mut instance = Chip8::new();
    instance.memory[0x200] = 0x60;
    instance.memory[0x201] = 0x3A;
    instance.cycle().unwrap();

    instance.memory[0x202] = 0xBA;
    instance.memory[0x203] = 0x37;
    instance.cycle().unwrap();

    assert_hex::assert_eq_hex!(instance.pc, 0x3A + 0xA37);
}

#[test]
fn op_ex9e() {
    let mut instance = Chip8::new();
    instance.keyboard[0] = true;
    instance.memory[0x200] = 0xE0;
    instance.memory[0x201] = 0x9E;
    instance.cycle().unwrap();

    assert_hex::assert_eq_hex!(instance.pc, 0x204);
}

// #[test]
// fn op_exa1() {
//     let mut instance = Chip8::new();
//     instance.keyboard[0] = true;
//     instance.memory[0x200] = 0xE0;
//     instance.memory[0x201] = 0x9E;
//     instance.cycle().unwrap();

//     assert_hex::assert_eq_hex!(instance.pc, 0x202);
// }

#[test]
fn op_fx07() {
    let mut instance = Chip8::new();
    instance.delay_timer = 0x55;
    instance.memory[0x200] = 0xF1;
    instance.memory[0x201] = 0x07;
    instance.cycle().unwrap();

    assert_hex::assert_eq_hex!(instance.v[0x1], 0x55);
}

#[test]
fn op_fx15() {
    let mut instance = Chip8::new();
    instance.v[0x7] = 0x55;
    instance.memory[0x200] = 0xF7;
    instance.memory[0x201] = 0x15;
    instance.cycle().unwrap();

    assert_hex::assert_eq_hex!(instance.v[0x7], 0x55);
}

#[test]
fn op_fx18() {
    let mut instance = Chip8::new();
    instance.v[0x7] = 0x55;
    instance.memory[0x200] = 0xF7;
    instance.memory[0x201] = 0x18;
    instance.cycle().unwrap();

    assert_hex::assert_eq_hex!(instance.v[0x7], 0x55);
}

#[test]
fn op_fx1e() {
    let mut instance = Chip8::new();
    instance.v[0x7] = 0x55;
    instance.i = 0x1;
    instance.memory[0x200] = 0xF7;
    instance.memory[0x201] = 0x1E;
    instance.cycle().unwrap();

    assert_hex::assert_eq_hex!(instance.i, 0x56);
}

#[test]
fn op_fx29() {
    let mut instance = Chip8::new();
    instance.v[0x7] = 0x1;
    instance.memory[0x200] = 0xF7;
    instance.memory[0x201] = 0x29;
    instance.cycle().unwrap();

    assert_hex::assert_eq_hex!(instance.i, 0x5);
}

#[test]
fn op_fx33() {
    let mut instance = Chip8::new();
    instance.v[0x7] = 0x91;
    instance.i = 0x300;
    instance.memory[0x200] = 0xF7;
    instance.memory[0x201] = 0x33;
    instance.cycle().unwrap();

    assert_hex::assert_eq_hex!(instance.memory[instance.i as usize], 0x1);
    assert_hex::assert_eq_hex!(instance.memory[(instance.i + 1) as usize], 0x4);
    assert_hex::assert_eq_hex!(instance.memory[(instance.i + 2) as usize], 0x5);
}

#[test]
fn op_fx55() {
    let mut instance = Chip8::new();
    instance.v[0x1] = 0x91;
    instance.i = 0x300;
    instance.memory[0x200] = 0xF1;
    instance.memory[0x201] = 0x55;
    instance.cycle().unwrap();

    assert_hex::assert_eq_hex!(instance.memory[(instance.i + 1) as usize], 0x91);
}

#[test]
fn op_fx65() {
    let mut instance = Chip8::new();
    instance.i = 0x300;
    instance.memory[instance.i as usize] = 0x35;
    instance.memory[0x200] = 0xF0;
    instance.memory[0x201] = 0x65;
    instance.cycle().unwrap();

    assert_hex::assert_eq_hex!(instance.v[0x0], 0x35);
}