use crate::cpu::CPU;

impl CPU {
    pub const BPL: u8 = 0x10;


    pub fn run_bpl(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0x10000], inst: u8) -> bool {
        if inst == CPU::BPL {
            let val = self.read_next_byte(&mut cycles, *mem);
            if !self.n {
                *cycles += 1;
                let old_pc = self.pc;
                if val >= 0x80 {
                    let neg = (val ^ 0xff) + 1;
                    self.pc -= neg as u16;
                } else {
                    self.pc += val as u16;
                }
                if 0x0100 & self.pc != 0x0100 & old_pc {
                    *cycles += 1;
                }
            }
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
    fn test_bpl_1() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::BPL;
        mem[0xFFFD] = 0x01;
        cpu.n = true;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.pc, 0xFFFE);
    }
    #[test]
    fn test_bpl_2() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::BPL;
        mem[0xFFFD] = 0x01;
        cpu.n = false;
        let cycles = 3;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.pc, 0xFFFF);
    }
    #[test]
    fn test_bpl_3() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::BPL;
        mem[0xFFFD] = 0xFF;
        cpu.n = false;
        let cycles = 3;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.pc, 0xFFFD);
    }
    #[test]
    fn test_bpl_4() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFF0C] = CPU::BPL;
        mem[0xFF0D] = 0x80;
        cpu.pc = 0xFF0C;
        cpu.n = false;
        let cycles = 4;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.pc, 0xFE8E);
    }
}