use crate::cpu::CPU;

impl CPU {
    pub const DEC_ZP: u8 = 0xC6;
    pub const DEC_ZPX: u8 = 0xD6;
    pub const DEC_ABS: u8 = 0xCE;
    pub const DEC_ABSX: u8 = 0xDE;

    fn dec(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0x10000], addr: u16) {
        let mut val = self.read_byte(&mut cycles, *mem, addr);
        val = val.wrapping_sub(1);
        *cycles += 1;
        self.write_byte(&mut cycles, mem, addr, val);
        self.set_flags(val, CPU::FLAG_N | CPU::FLAG_Z);
        return;
    }

    pub fn run_dec(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0x10000], inst: u8) -> bool {
        if inst == CPU::DEC_ZP {
            let addr = self.read_zero_page_addr(&mut cycles, *mem);
            self.dec(&mut cycles, mem, addr);
        } else if inst == CPU::DEC_ZPX {
            let addr = self.read_zero_page_x_addr(&mut cycles, *mem);
            self.dec(&mut cycles, mem, addr);
        } else if inst == CPU::DEC_ABS {
            let addr = self.read_abs_addr(&mut cycles, *mem);
            self.dec(&mut cycles, mem, addr);
        } else if inst == CPU::DEC_ABSX {
            let addr = self.read_abs_x_addr(&mut cycles, *mem, false);
            *cycles += 1;
            self.dec(&mut cycles, mem, addr);
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
    fn test_dec_1() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::DEC_ZP;
        mem[0xFFFD] = 0x54;
        mem[0x0054] = 54;
        let cycles = 5;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(mem[0x0054], 53);
        assert_eq!(cpu.n, false);
        assert_eq!(cpu.z, false);
    }
    #[test]
    fn test_dec_2() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::DEC_ZP;
        mem[0xFFFD] = 0x54;
        mem[0x0054] = 0;
        let cycles = 5;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(mem[0x0054], 0xff);
        assert_eq!(cpu.n, true);
        assert_eq!(cpu.z, false);
    }
    #[test]
    fn test_dec_3() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::DEC_ZP;
        mem[0xFFFD] = 0x54;
        mem[0x0054] = 1;
        let cycles = 5;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(mem[0x0054], 0);
        assert_eq!(cpu.n, false);
        assert_eq!(cpu.z, true);
    }
    #[test]
    fn test_dec_4() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::DEC_ZPX;
        mem[0xFFFD] = 0x50;
        mem[0x0054] = 8;
        cpu.x = 4;
        let cycles = 6;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(mem[0x0054], 7);
    }
    #[test]
    fn test_dec_5() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::DEC_ABS;
        mem[0xFFFD] = 0x50;
        mem[0xFFFE] = 0x40;
        mem[0x4050] = 8;
        let cycles = 6;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(mem[0x4050], 7);
    }
    #[test]
    fn test_dec_6() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::DEC_ABSX;
        mem[0xFFFD] = 0x50;
        mem[0xFFFE] = 0x40;
        mem[0x4054] = 8;
        cpu.x = 4;
        let cycles = 7;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(mem[0x4054], 7);
    }
}
