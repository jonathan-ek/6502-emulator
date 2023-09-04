use crate::cpu::CPU;

impl CPU {
    pub const PHA: u8 = 0x48;
    pub fn run_pha(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0x10000], inst: u8) -> bool {
        if inst == CPU::PHA {
            self.push_to_stack(&mut cycles, mem, self.a)
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
    fn test_pha() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::PHA;
        cpu.a = 0xB4;
        let cycles = 3;
        assert_eq!(cpu.sp, 0, "sp reg");
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(mem[0x0100], 0xB4, "a reg");
        assert_eq!(cpu.sp, 1, "sp reg");
    }
}