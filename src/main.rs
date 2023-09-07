use std::fs;
use crate::cpu::CPU;
pub mod cpu;

fn main() {
    let mut cpu = CPU::new();
    // let mut mem: [u8; 0x10000] = [0; 0x10000];
    let file_path = "inputs/program.bin";
    let contents: Vec<u8> = fs::read(file_path)
        .expect("Should have been able to read the file");
    let boxed_slice: Box<[u8]> = contents.into_boxed_slice();
    let m: Box<[u8; 0x10000]> = boxed_slice.try_into().unwrap();
    let mut mem: [u8; 0x10000] = *m;
    cpu.pc = 0x0400;
    let cycles = 0;
    cpu.run(cycles, &mut mem);
}
