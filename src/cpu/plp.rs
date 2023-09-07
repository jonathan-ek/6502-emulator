use crate::cpu::CPU;

impl CPU {
    pub const PLP: u8 = 0x28;

    pub fn run_plp(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0x10000], inst: u8) -> bool {
        if inst == CPU::PLP {
            let res = self.pop_from_stack(&mut cycles, mem);
            self.c = res & CPU::FLAG_C > 0;
            self.z = res & CPU::FLAG_Z > 0;
            self.i = res & CPU::FLAG_I > 0;
            self.d = res & CPU::FLAG_D > 0;
            // self.b = res & CPU::FLAG_B > 0;
            self.v = res & CPU::FLAG_V > 0;
            self.n = res & CPU::FLAG_N > 0;
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
    fn test_plp() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::PLP;
        mem[0x01FF] = 0b1000_0011;
        cpu.sp = 0xFE;
        let cycles = 4;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.c, true, "c reg");
        assert_eq!(cpu.n, true, "n reg");
        assert_eq!(cpu.z, true, "z reg");
        assert_eq!(cpu.b, false, "b reg");
        assert_eq!(cpu.d, false, "d reg");
        assert_eq!(cpu.i, false, "i reg");
        assert_eq!(cpu.v, false, "v reg");
        assert_eq!(cpu.sp, 0xFF, "sp reg");
    }
}