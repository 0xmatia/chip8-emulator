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
    // this means if v2 is equals to 0xA skip to next instruciton 0x204
    instance.v[2] = 0xA;
    instance.memory[0x200] = 0x32;
    instance.memory[0x201] = 0x0A;
    instance.cycle().unwrap();

    assert_hex::assert_eq_hex!(instance.pc, 0x0204);
}


#[test]
fn op_4xkk() {
    let mut instance = Chip8::new();
    // this means if v2 is equals to 0xA skip to next instruciton 0x204
    instance.v[2] = 0xB;
    instance.memory[0x200] = 0x42;
    instance.memory[0x201] = 0x0A;
    instance.cycle().unwrap();

    assert_hex::assert_eq_hex!(instance.pc, 0x0204);
}
