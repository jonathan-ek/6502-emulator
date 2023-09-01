use crate::cpu::CPU;

impl CPU {
    pub const TAX: u8 = 0xAA;

    pub fn run_tax(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0xFFFF], inst: u8) -> bool {
        if inst == CPU::TAX {
            self.x = self.a;
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
    fn test_tax() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::TAX;
        cpu.x = 0x42;
        cpu.a = 5;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.x, 0x05);
    }
}