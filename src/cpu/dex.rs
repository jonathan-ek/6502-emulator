use crate::cpu::CPU;

impl CPU {
    pub const DEX: u8 = 0xCA;

    pub fn run_dex(&mut self, cycles: &mut u32, _mem: &mut [u8; 0x10000], inst: u8) -> bool {
        if inst == CPU::DEX {
            self.x = self.x.wrapping_sub(1);
            *cycles += 1;
            self.set_flags(self.x, CPU::FLAG_N | CPU::FLAG_Z);
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
    fn test_dex() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::DEX;
        cpu.x = 0x42;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.x, 0x41);
    }
}