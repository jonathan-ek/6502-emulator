use crate::cpu::CPU;

impl CPU {
    pub const STX_ZP: u8 = 0x86;
    pub const STX_ZPY: u8 = 0x96;
    pub const STX_ABS: u8 = 0x8E;

    pub fn run_stx(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0x10000], inst: u8) -> bool {
        if inst == CPU::STX_ZP {
            let addr = self.read_zero_page_addr(&mut cycles, *mem);
            self.write_byte(&mut cycles, mem, addr, self.x);
        } else if inst == CPU::STX_ZPY {
            let addr = self.read_zero_page_y_addr(&mut cycles, *mem);
            self.write_byte(&mut cycles, mem, addr, self.x);
        } else if inst == CPU::STX_ABS {
            let addr = self.read_abs_addr(&mut cycles, *mem);
            self.write_byte(&mut cycles, mem, addr, self.x);
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
    fn test_stx_zp() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::STX_ZP;
        mem[0xFFFD] = 0x25;
        cpu.x = 62;
        let cycles = 3;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(mem[0x0025], 62);
    }
    #[test]
    fn test_stx_zpy() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::STX_ZPY;
        mem[0xFFFD] = 0x25;
        cpu.x = 62;
        cpu.y = 2;
        let cycles = 4;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(mem[0x0027], 62);
    }
    #[test]
    fn test_stx_abs() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::STX_ABS;
        mem[0xFFFD] = 0x25;
        mem[0xFFFE] = 0x48;
        cpu.x = 62;
        let cycles = 4;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(mem[0x4825], 62);
    }
}
