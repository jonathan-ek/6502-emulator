use crate::cpu::CPU;

impl CPU {
    pub const ASL_A: u8 = 0x0a;
    pub const ASL_ZP: u8 = 0x06;
    pub const ASL_ZPX: u8 = 0x16;
    pub const ASL_ABS: u8 = 0x0e;
    pub const ASL_ABSX: u8 = 0x1e;

    pub fn run_asl(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0xFFFF], inst: u8) -> bool {
        if inst == CPU::ASL_A {
        } else if inst == CPU::ASL_ZP {
        } else if inst == CPU::ASL_ZPX {
        } else if inst == CPU::ASL_ABS {
        } else if inst == CPU::ASL_ABSX {
        } else {
            return false;
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::CPU;

    #[test]
    fn test_nop() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::NOP;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
    }
}
