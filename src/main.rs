use crate::cpu::CPU;
pub mod cpu;

fn main() {
    let mut cpu = CPU::new();
    let mut mem: [u8; 0x10000] = [0; 0x10000];
    mem[0xFFFC] = CPU::LDA_IM;
    mem[0xFFFD] = 0x48;
    let cycles = 2;
    assert_eq!(cpu.run(cycles, &mut mem), cycles);
    assert_eq!(cpu.a, 0x48, "A reg");
    assert_eq!(cpu.n, false, "N flag");
    assert_eq!(cpu.z, false, "Z flag");
}
