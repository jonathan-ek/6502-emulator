use crate::cpu::CPU;

impl CPU {
    pub const CMP_IM: u8 = 0xC9;
    pub const CMP_ZP: u8 = 0xC5;
    pub const CMP_ZPX: u8 = 0xD5;
    pub const CMP_ABS: u8 = 0xCD;
    pub const CMP_ABSX: u8 = 0xDD;
    pub const CMP_ABSY: u8 = 0xD9;
    pub const CMP_INDX: u8 = 0xC1;
    pub const CMP_INDY: u8 = 0xD1;

    fn cmp(&mut self, tmp: u8) {
        let val = self.a.wrapping_sub(tmp);
        self.n = val & 0x80 > 0;
        self.c = self.a >= tmp;
        self.z = self.a == tmp;
    }

    pub fn run_cmp(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0x10000], inst: u8) -> bool {
        if inst == CPU::CMP_IM {
            let tmp = self.read_next_byte(&mut cycles, *mem);
            self.cmp(tmp);
        } else if inst == CPU::CMP_ZP {
            let tmp = self.read_zero_page(&mut cycles, *mem);
            self.cmp(tmp);
        } else if inst == CPU::CMP_ZPX {
            let tmp = self.read_zero_page_x(&mut cycles, *mem);
            self.cmp(tmp);
        } else if inst == CPU::CMP_ABS {
            let tmp = self.read_abs(&mut cycles, *mem);
            self.cmp(tmp);
        } else if inst == CPU::CMP_ABSX {
            let tmp = self.read_abs_x(&mut cycles, *mem);
            self.cmp(tmp);
        } else if inst == CPU::CMP_ABSY {
            let tmp = self.read_abs_y(&mut cycles, *mem);
            self.cmp(tmp);
        } else if inst == CPU::CMP_INDX {
            let addr = self.read_indexed_indirect_addr(&mut cycles, *mem);
            let tmp = self.read_byte(&mut cycles, *mem, addr);
            self.cmp(tmp);
        } else if inst == CPU::CMP_INDY {
            let addr = self.read_indirect_indexed_addr(&mut cycles, *mem, true);
            let tmp = self.read_byte(&mut cycles, *mem, addr);
            self.cmp(tmp);
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
    fn test_cmp_1() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::CMP_IM;
        mem[0xFFFD] = 42;
        cpu.a = 42;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.z, true);
        assert_eq!(cpu.n, false);
        assert_eq!(cpu.c, true);
    }
    #[test]
    fn test_cmp_2() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::CMP_IM;
        mem[0xFFFD] = 12;
        cpu.a = 42;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.z, false);
        assert_eq!(cpu.n, false);
        assert_eq!(cpu.c, true);
    }
    #[test]
    fn test_cmp_3() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::CMP_IM;
        mem[0xFFFD] = 42;
        cpu.a = 132;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.z, false);
        assert_eq!(cpu.n, false);
        assert_eq!(cpu.c, true);
    }
    #[test]
    fn test_cmp_4() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::CMP_IM;
        mem[0xFFFD] = 132;
        cpu.a = 42;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.z, false);
        assert_eq!(cpu.n, true);
        assert_eq!(cpu.c, false);
    }
}
