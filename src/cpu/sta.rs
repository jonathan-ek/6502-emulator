use crate::cpu::CPU;

impl CPU {
    pub const STA_ZP: u8 = 0x85;
    pub const STA_ZPX: u8 = 0x95;
    pub const STA_ABS: u8 = 0x8D;
    pub const STA_ABSX: u8 = 0x9D;
    pub const STA_ABSY: u8 = 0x99;
    pub const STA_INDX: u8 = 0x81;
    pub const STA_INDY: u8 = 0x91;

    pub fn run_sta(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0x10000], inst: u8) -> bool {
        if inst == CPU::STA_ZP {
            let addr = self.read_zero_page_addr(&mut cycles, *mem);
            self.write_byte(&mut cycles, mem, addr, self.a);
        } else if inst == CPU::STA_ZPX {
            let addr = self.read_zero_page_x_addr(&mut cycles, *mem);
            self.write_byte(&mut cycles, mem, addr, self.a);
        } else if inst == CPU::STA_ABS {
            let addr = self.read_abs_addr(&mut cycles, *mem);
            self.write_byte(&mut cycles, mem, addr, self.a);
        } else if inst == CPU::STA_ABSX {
            let addr = self.read_abs_x_addr(&mut cycles, *mem, false);
            *cycles += 1;
            self.write_byte(&mut cycles, mem, addr, self.a);
        } else if inst == CPU::STA_ABSY {
            let addr = self.read_abs_y_addr(&mut cycles, *mem, false);
            *cycles += 1;
            self.write_byte(&mut cycles, mem, addr, self.a);
        } else if inst == CPU::STA_INDX {
            let addr = self.read_indexed_indirect_addr(&mut cycles, *mem);
            self.write_byte(&mut cycles, mem, addr, self.a);
        } else if inst == CPU::STA_INDY {
            let addr = self.read_indirect_indexed_addr(&mut cycles, *mem, false);
            *cycles += 1;
            self.write_byte(&mut cycles, mem, addr, self.a);
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
    fn test_sta_zp() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::STA_ZP;
        mem[0xFFFD] = 0x25;
        cpu.a = 62;
        let cycles = 3;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(mem[0x0025], 62);
    }
    #[test]
    fn test_sta_zpx() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::STA_ZPX;
        mem[0xFFFD] = 0x25;
        cpu.a = 62;
        cpu.x = 2;
        let cycles = 4;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(mem[0x0027], 62);
    }
    #[test]
    fn test_sta_abs() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::STA_ABS;
        mem[0xFFFD] = 0x25;
        mem[0xFFFE] = 0x48;
        cpu.a = 62;
        let cycles = 4;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(mem[0x4825], 62);
    }
    #[test]
    fn test_sta_absx() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::STA_ABSX;
        mem[0xFFFD] = 0x25;
        mem[0xFFFE] = 0x48;
        cpu.a = 62;
        cpu.x = 2;
        let cycles = 5;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(mem[0x4827], 62);
    }
    #[test]
    fn test_sta_absy() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::STA_ABSY;
        mem[0xFFFD] = 0x25;
        mem[0xFFFE] = 0x48;
        cpu.a = 62;
        cpu.y = 2;
        let cycles = 5;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(mem[0x4827], 62);
    }
    #[test]
    fn test_sta_indx() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::STA_INDX;
        mem[0xFFFD] = 0x25;
        mem[0x0027] = 0x29;
        mem[0x0028] = 0x48;
        cpu.a = 62;
        cpu.x = 2;
        let cycles = 6;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(mem[0x4829], 62);
    }
    #[test]
    fn test_sta_indy() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::STA_INDY;
        mem[0xFFFD] = 0x25;
        mem[0x0025] = 0x29;
        mem[0x0026] = 0x48;
        cpu.a = 62;
        cpu.y = 2;
        let cycles = 6;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(mem[0x482B], 62);
    }
}
