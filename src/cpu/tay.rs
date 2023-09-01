use crate::cpu::CPU;

impl CPU {
    pub const TAY: u8 = 0xA8;

    pub fn run_tay(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0xFFFF], inst: u8) -> bool {
        if inst == CPU::TAY {
            self.y = self.a;
            *cycles += 1;
            self.set_flags(self.y, CPU::FLAG_N | CPU::FLAG_Z);
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
    fn test_tay() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::TAY;
        cpu.y = 0x42;
        cpu.a = 5;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.y, 0x05);
    }
}