use crate::cpu::CPU;

impl CPU {
    pub const PLA: u8 = 0x68;

    pub fn run_pla(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0x10000], inst: u8) -> bool {
        if inst == CPU::PLA {
            self.a = self.pop_from_stack(&mut cycles, mem);
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
    fn test_pla() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::PLA;
        mem[0x0100] = 0xB4;
        cpu.a = 0;
        cpu.sp = 1;
        let cycles = 4;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 0xB4, "a reg");
        assert_eq!(cpu.sp, 0, "sp reg");
    }
}