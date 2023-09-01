use crate::cpu::CPU;

impl CPU {
    pub const TYA: u8 = 0x98;

    pub fn run_tya(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0xFFFF], inst: u8) -> bool {
        if inst == CPU::TYA {
            self.a = self.y;
            *cycles += 1;
            self.set_flags(self.a, CPU::FLAG_N | CPU::FLAG_Z);
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
    fn test_tya() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::TYA;
        cpu.y = 0x42;
        cpu.a = 5;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 0x42);
    }
}