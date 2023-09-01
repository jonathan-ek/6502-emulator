use crate::cpu::CPU;

impl CPU {
    pub const TSX: u8 = 0xBA;

    pub fn run_tsx(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0xFFFF], inst: u8) -> bool {
        if inst == CPU::TSX {
            self.x = self.sp;
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
    fn test_tsx() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::TSX;
        cpu.sp = 5;
        cpu.x = 0x45;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.x, 5);
    }
}