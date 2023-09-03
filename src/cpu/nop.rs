use crate::cpu::CPU;

impl CPU {
    pub const NOP: u8 = 0xEA;

    pub fn run_nop(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0xFFFF], inst: u8) -> bool {
        if inst == CPU::NOP {
            *cycles += 1;
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