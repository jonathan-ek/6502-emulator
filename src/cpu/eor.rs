use crate::cpu::CPU;

impl CPU {
    pub const EOR_IM: u8 = 0x49;
    pub const EOR_ZP: u8 = 0x45;
    pub const EOR_ZPX: u8 = 0x55;
    pub const EOR_ABS: u8 = 0x4D;
    pub const EOR_ABSX: u8 = 0x5D;
    pub const EOR_ABSY: u8 = 0x59;
    pub const EOR_INDX: u8 = 0x41;
    pub const EOR_INDY: u8 = 0x51;

    fn eor(&mut self, val: u8) {
        self.a = self.a ^ val;
        self.set_flags(self.a, CPU::FLAG_N | CPU::FLAG_Z);
    }
    pub fn run_eor(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0x10000], inst: u8) -> bool {
        if inst == CPU::EOR_IM {
            let val = self.read_next_byte(&mut cycles, *mem);
            self.eor(val);
        } else if inst == CPU::EOR_ZP {
            let addr = self.read_zero_page_addr(&mut cycles, *mem);
            let val = self.read_byte(&mut cycles, *mem, addr);
            self.eor(val);
        } else if inst == CPU::EOR_ZPX {
            let addr = self.read_zero_page_x_addr(&mut cycles, *mem);
            let val = self.read_byte(&mut cycles, *mem, addr);
            self.eor(val);
        } else if inst == CPU::EOR_ABS {
            let addr = self.read_abs_addr(&mut cycles, *mem);
            let val = self.read_byte(&mut cycles, *mem, addr);
            self.eor(val);
        } else if inst == CPU::EOR_ABSX {
            let addr = self.read_abs_x_addr(&mut cycles, *mem, true);
            let val = self.read_byte(&mut cycles, *mem, addr);
            self.eor(val);
        } else if inst == CPU::EOR_ABSY {
            let addr = self.read_abs_y_addr(&mut cycles, *mem, true);
            let val = self.read_byte(&mut cycles, *mem, addr);
            self.eor(val);
        } else if inst == CPU::EOR_INDX {
            let addr = self.read_indexed_indirect_addr(&mut cycles, *mem);
            let val = self.read_byte(&mut cycles, *mem, addr);
            self.eor(val);
        } else if inst == CPU::EOR_INDY {
            let addr = self.read_indirect_indexed_addr(&mut cycles, *mem, true);
            let val = self.read_byte(&mut cycles, *mem, addr);
            self.eor(val);
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
    fn test_eor_1() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::EOR_IM;
        mem[0xFFFD] = 0b1010_0101;
        cpu.a = 0b0011_0011;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 0b1001_0110);
        assert_eq!(cpu.n, true);
        assert_eq!(cpu.z, false);
    }
    #[test]
    fn test_eor_2() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::EOR_IM;
        mem[0xFFFD] = 0b1010_0101;
        cpu.a = 0b1100_0011;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 0b0110_0110);
        assert_eq!(cpu.n, false);
        assert_eq!(cpu.z, false);
    }
    #[test]
    fn test_eor_3() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::EOR_IM;
        mem[0xFFFD] = 0b1100_0011;
        cpu.a = 0b1100_0011;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.n, false);
        assert_eq!(cpu.z, true);
    }
}
