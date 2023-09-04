use crate::cpu::CPU;

impl CPU {
    pub const STX_ZP: u8 = 0x86;
    pub const STX_ZPY: u8 = 0x96;
    pub const STX_ABS: u8 = 0x8E;

    pub fn run_stx(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0x10000], inst: u8) -> bool {
        if inst == CPU::STX_ZP {
        } else if inst == CPU::STX_ZPY {
        } else if inst == CPU::STX_ABS {
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
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::NOP;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
    }
}
