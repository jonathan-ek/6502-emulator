use crate::cpu::CPU;

impl CPU {
    pub const INX: u8 = 0xE8;

    pub fn run_inx(&mut self, mut cycles: &mut u32, _mem: &mut [u8; 0x10000], inst: u8) -> bool {
        if inst == CPU::INX {
            self.x = self.x.wrapping_add(1);
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
    fn test_inx() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::INX;
        cpu.x = 0x42;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.x, 0x43);
    }
}