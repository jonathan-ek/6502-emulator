use crate::cpu::CPU;

impl CPU {
    pub const CLD: u8 = 0xD8;

    pub fn run_cld(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0xFFFF], inst: u8) -> bool {
        if inst == CPU::CLD {
            self.d = false;
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
    fn test_cld() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::CLD;
        cpu.d = true;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.d, false);
    }
}