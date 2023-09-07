use crate::cpu::CPU;

impl CPU {
    pub const TXA: u8 = 0x8A;

    pub fn run_txa(&mut self, cycles: &mut u32, _mem: &mut [u8; 0x10000], inst: u8) -> bool {
        if inst == CPU::TXA {
            self.a = self.x;
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
    fn test_txa() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::TXA;
        cpu.x = 0x42;
        cpu.a = 5;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 0x42);
    }
}