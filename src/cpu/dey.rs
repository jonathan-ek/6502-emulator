use crate::cpu::CPU;

impl CPU {
    pub const DEY: u8 = 0x88;

    pub fn run_dey(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0xFFFF], inst: u8) -> bool {
        if inst == CPU::DEY {
            self.y = self.y.wrapping_sub(1);
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
    fn test_dey() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::DEY;
        cpu.y = 0x42;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.y, 0x41);
    }
    #[test]
    fn test_dey_2() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::DEY;
        cpu.y = 0x01;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.z, true);
    }
    #[test]
    fn test_dey_3() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::DEY;
        cpu.y = 0x00;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.y, 0xFF);
        assert_eq!(cpu.z, false);
        assert_eq!(cpu.n, true);
    }
}