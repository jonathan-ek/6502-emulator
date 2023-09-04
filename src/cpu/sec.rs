use crate::cpu::CPU;

impl CPU {
    pub const SEC: u8 = 0x38;

    pub fn run_sec(&mut self, mut cycles: &mut u32, _mem: &mut [u8; 0x10000], inst: u8) -> bool {
        if inst == CPU::SEC {
            self.c = true;
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
    fn test_sec() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::SEC;
        cpu.c = false;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.c, true);
    }
}