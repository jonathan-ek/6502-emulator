use crate::cpu::CPU;

impl CPU {
    pub const STY_ZP: u8 = 0x84;
    pub const STY_ZPX: u8 = 0x94;
    pub const STY_ABS: u8 = 0x8C;

    pub fn run_sty(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0xFFFF], inst: u8) -> bool {
        if inst == CPU::STY_ZP {
        } else if inst == CPU::STY_ZPX {
        } else if inst == CPU::STY_ABS {
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
