use crate::cpu::CPU;

impl CPU {
    pub const PHP: u8 = 0x08;

    pub fn run_php(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0xFFFF], inst: u8) -> bool {
        if inst == CPU::PHP {
            let mut res: u8 = 0;
            if self.c { res += CPU::FLAG_C; }
            if self.z { res += CPU::FLAG_Z; }
            if self.i { res += CPU::FLAG_I; }
            if self.d { res += CPU::FLAG_D; }
            if self.b { res += CPU::FLAG_B; }
            if self.v { res += CPU::FLAG_V; }
            if self.n { res += CPU::FLAG_N; }
            self.push_to_stack(&mut cycles, mem, res)
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
    fn test_php() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::PHP;
        cpu.c = true;
        cpu.n = true;
        cpu.z = true;
        let cycles = 3;
        assert_eq!(cpu.sp, 0, "sp reg");
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(mem[0x0100], 0b1000_0011, "a reg");
        assert_eq!(cpu.sp, 1, "sp reg");
    }
}