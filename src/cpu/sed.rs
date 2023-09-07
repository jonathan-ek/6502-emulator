use crate::cpu::CPU;

impl CPU {
    pub const SED: u8 = 0xF8;

    pub fn run_sed(&mut self, cycles: &mut u32, _mem: &mut [u8; 0x10000], inst: u8) -> bool {
        if inst == CPU::SED {
            self.d = true;
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
    fn test_sed() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::SED;
        cpu.d = false;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.d, true);
    }
}