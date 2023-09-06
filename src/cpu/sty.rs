use crate::cpu::CPU;

impl CPU {
    pub const STY_ZP: u8 = 0x84;
    pub const STY_ZPX: u8 = 0x94;
    pub const STY_ABS: u8 = 0x8C;

    pub fn run_sty(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0x10000], inst: u8) -> bool {
        if inst == CPU::STY_ZP {
            let addr = self.read_zero_page_addr(&mut cycles, *mem);
            self.write_byte(&mut cycles, mem, addr, self.y);
        } else if inst == CPU::STY_ZPX {
            let addr = self.read_zero_page_x_addr(&mut cycles, *mem);
            self.write_byte(&mut cycles, mem, addr, self.y);
        } else if inst == CPU::STY_ABS {
            let addr = self.read_abs_addr(&mut cycles, *mem);
            self.write_byte(&mut cycles, mem, addr, self.y);
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
    fn test_sty_zp() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::STY_ZP;
        mem[0xFFFD] = 0x25;
        cpu.y = 62;
        let cycles = 3;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(mem[0x0025], 62);
    }
    #[test]
    fn test_sty_zpx() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::STY_ZPX;
        mem[0xFFFD] = 0x25;
        cpu.y = 62;
        cpu.x = 2;
        let cycles = 4;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(mem[0x0027], 62);
    }
    #[test]
    fn test_sty_abs() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::STY_ABS;
        mem[0xFFFD] = 0x25;
        mem[0xFFFE] = 0x48;
        cpu.y = 62;
        let cycles = 4;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(mem[0x4825], 62);
    }
}
