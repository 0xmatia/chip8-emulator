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