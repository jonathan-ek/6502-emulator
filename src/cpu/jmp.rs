use crate::cpu::CPU;

impl CPU {
    pub const JMP_ABS: u8 = 0x4C;
    pub const JMP_IND: u8 = 0x6C;

    pub fn run_jmp(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0x10000], inst: u8) -> bool {
        if inst == CPU::JMP_ABS {
            self.pc = self.read_abs_addr(&mut cycles, *mem);
        } else if inst == CPU::JMP_IND {
            let addr = self.read_abs_addr(&mut cycles, *mem);
            let mut lsb = self.read_byte(&mut cycles, *mem, addr);
            let mut msb = self.read_byte(&mut cycles, *mem, addr+1);
            self.pc = ((msb as u16) << 8) + (lsb as u16);
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
    fn test_jmp_abs() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::JMP_ABS;
        mem[0xFFFD] = 0x40;
        mem[0xFFFE] = 0x65;
        let cycles = 3;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.pc, 0x6540);
    }
    #[test]
    fn test_jmp_ind() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::JMP_IND;
        mem[0xFFFD] = 0x40;
        mem[0xFFFE] = 0x65;
        mem[0x6540] = 0x32;
        mem[0x6541] = 0x54;
        let cycles = 5;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.pc, 0x5432);
    }
}