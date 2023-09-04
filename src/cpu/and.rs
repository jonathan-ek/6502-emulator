use crate::cpu::CPU;

impl CPU {
    pub const AND_IM: u8 = 0x29;
    pub const AND_ZP: u8 = 0x25;
    pub const AND_ZPX: u8 = 0x35;
    pub const AND_ABS: u8 = 0x2D;
    pub const AND_ABSX: u8 = 0x3D;
    pub const AND_ABSY: u8 = 0x39;
    pub const AND_INDX: u8 = 0x21;
    pub const AND_INDY: u8 = 0x31;

    fn and(&mut self, val: u8) {
        self.a = self.a & val;
        self.set_flags(self.a, CPU::FLAG_N | CPU::FLAG_Z);
    }
    pub fn run_and(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0x10000], inst: u8) -> bool {
        if inst == CPU::AND_IM {
            let val = self.read_next_byte(&mut cycles, *mem);
            self.and(val);
        } else if inst == CPU::AND_ZP {
            let addr = self.read_zero_page_addr(&mut cycles, *mem);
            let val = self.read_byte(&mut cycles, *mem, addr);
            self.and(val);
        } else if inst == CPU::AND_ZPX {
            let addr = self.read_zero_page_x_addr(&mut cycles, *mem);
            let val = self.read_byte(&mut cycles, *mem, addr);
            self.and(val);
        } else if inst == CPU::AND_ABS {
            let addr = self.read_abs_addr(&mut cycles, *mem);
            let val = self.read_byte(&mut cycles, *mem, addr);
            self.and(val);
        } else if inst == CPU::AND_ABSX {
            let addr = self.read_abs_x_addr(&mut cycles, *mem, true);
            let val = self.read_byte(&mut cycles, *mem, addr);
            self.and(val);
        } else if inst == CPU::AND_ABSY {
            let addr = self.read_abs_y_addr(&mut cycles, *mem, true);
            let val = self.read_byte(&mut cycles, *mem, addr);
            self.and(val);
        } else if inst == CPU::AND_INDX {
            let addr = self.read_indexed_indirect_addr(&mut cycles, *mem);
            let val = self.read_byte(&mut cycles, *mem, addr);
            self.and(val);
        } else if inst == CPU::AND_INDY {
            let addr = self.read_indirect_indexed_addr(&mut cycles, *mem, true);
            let val = self.read_byte(&mut cycles, *mem, addr);
            self.and(val);
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
    fn test_and_1() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::AND_IM;
        mem[0xFFFD] = 0b1010_0101;
        cpu.a = 0b0011_0011;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 0b0010_0001);
        assert_eq!(cpu.n, false);
        assert_eq!(cpu.z, false);
    }
    #[test]
    fn test_and_2() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::AND_IM;
        mem[0xFFFD] = 0b1010_0101;
        cpu.a = 0b1100_0011;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 0b1000_0001);
        assert_eq!(cpu.n, true);
        assert_eq!(cpu.z, false);
    }
    #[test]
    fn test_and_3() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::AND_IM;
        mem[0xFFFD] = 0x00;
        cpu.a = 0b1100_0011;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.n, false);
        assert_eq!(cpu.z, true);
    }
}
