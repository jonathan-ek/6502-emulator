use crate::cpu::CPU;

impl CPU {
    pub const SBC_IM: u8 = 0xE9;
    pub const SBC_ZP: u8 = 0xE5;
    pub const SBC_ZPX: u8 = 0xF5;
    pub const SBC_ABS: u8 = 0xED;
    pub const SBC_ABSX: u8 = 0xFD;
    pub const SBC_ABSY: u8 = 0xF9;
    pub const SBC_INDX: u8 = 0xE1;
    pub const SBC_INDY: u8 = 0xF1;

    fn sbc(&mut self, val: u8) {
        let v = val ^ 0xff;
        let val_neg = v & 0b1000_0000 > 0;
        let a_neg = self.a & 0b1000_0000 > 0;
        let tmp = self.a as u16 + v as u16 + self.c as u16;
        self.c = 0x0100 & tmp > 0;
        self.a = (0x00FF & tmp) as u8;
        let res_neg = self.a & 0b1000_0000 > 0;
        self.v = (val_neg && a_neg && !res_neg) || (!val_neg && !a_neg && res_neg);
        self.set_flags(self.a, CPU::FLAG_N | CPU::FLAG_Z);
    }

    pub fn run_sbc(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0x10000], inst: u8) -> bool {
        if inst == CPU::SBC_IM {
            let val = self.read_next_byte(&mut cycles, *mem);
            self.sbc(val);
        } else if inst == CPU::SBC_ZP {
            let addr = self.read_zero_page_addr(&mut cycles, *mem);
            let val = self.read_byte(&mut cycles, *mem, addr);
            self.sbc(val);
        } else if inst == CPU::SBC_ZPX {
            let addr = self.read_zero_page_x_addr(&mut cycles, *mem);
            let val = self.read_byte(&mut cycles, *mem, addr);
            self.sbc(val);
        } else if inst == CPU::SBC_ABS {
            let addr = self.read_abs_addr(&mut cycles, *mem);
            let val = self.read_byte(&mut cycles, *mem, addr);
            self.sbc(val);
        } else if inst == CPU::SBC_ABSX {
            let addr = self.read_abs_x_addr(&mut cycles, *mem, true);
            let val = self.read_byte(&mut cycles, *mem, addr);
            self.sbc(val);
        } else if inst == CPU::SBC_ABSY {
            let addr = self.read_abs_y_addr(&mut cycles, *mem, true);
            let val = self.read_byte(&mut cycles, *mem, addr);
            self.sbc(val);
        } else if inst == CPU::SBC_INDX {
            let addr = self.read_indexed_indirect_addr(&mut cycles, *mem);
            let val = self.read_byte(&mut cycles, *mem, addr);
            self.sbc(val);
        } else if inst == CPU::SBC_INDY {
            let addr = self.read_indirect_indexed_addr(&mut cycles, *mem, true);
            let val = self.read_byte(&mut cycles, *mem, addr);
            self.sbc(val);
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
    fn test_sbc_1() {
        // 0 - 0 = 0
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::SBC_IM;
        mem[0xFFFD] = 0x00;
        cpu.c = true;
        cpu.a = 0;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 0, "A reg");
        assert_eq!(cpu.c, true, "c reg");
        assert_eq!(cpu.n, false, "n reg");
        assert_eq!(cpu.v, false, "v reg");
        assert_eq!(cpu.z, true, "z reg");
    }
    #[test]
    fn test_sbc_2() {
        // 0 - 0 - !c = -11
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::SBC_IM;
        mem[0xFFFD] = 0x00;
        cpu.c = false;
        cpu.a = 0;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a as i8, -1, "A reg");
        assert_eq!(cpu.c, false, "c reg");
        assert_eq!(cpu.n, true, "n reg");
        assert_eq!(cpu.v, false, "v reg");
        assert_eq!(cpu.z, false, "z reg");
    }
    #[test]
    fn test_sbc_2b() {
        // 0 - 1
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::SBC_IM;
        mem[0xFFFD] = 1;
        cpu.c = true;
        cpu.a = 0;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, -1i8 as u8, "A reg");
        assert_eq!(cpu.c, false, "c reg");
        assert_eq!(cpu.n, true, "n reg");
        assert_eq!(cpu.v, false, "v reg");
        assert_eq!(cpu.z, false, "z reg");
    }
    #[test]
    fn test_sbc_3() {
        // 100 - 37
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::SBC_IM;
        mem[0xFFFD] = 37;
        cpu.c = true;
        cpu.a = 100;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 63, "A reg");
        assert_eq!(cpu.c, true, "c reg");
        assert_eq!(cpu.n, false, "n reg");
        assert_eq!(cpu.v, false, "v reg");
        assert_eq!(cpu.z, false, "z reg");
    }
    #[test]
    fn test_sbc_3b() {
        // 37 - 100
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::SBC_IM;
        mem[0xFFFD] = 100;
        cpu.c = true;
        cpu.a = 37;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, -63i8 as u8, "A reg");
        assert_eq!(cpu.c, false, "c reg");
        assert_eq!(cpu.n, true, "n reg");
        assert_eq!(cpu.v, false, "v reg");
        assert_eq!(cpu.z, false, "z reg");
    }
    #[test]
    fn test_sbc_4() {
        // pos - neg
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::SBC_IM;
        mem[0xFFFD] = 0b1110_1111;  // -17
        cpu.c = true;
        cpu.a = 20;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 37, "A reg");
        assert_eq!(cpu.c, false, "c reg");
        assert_eq!(cpu.n, false, "n reg");
        assert_eq!(cpu.v, false, "v reg");
        assert_eq!(cpu.z, false, "z reg");
    }
    #[test]
    fn test_sbc_5() {
        // test carry
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::SBC_IM;
        mem[0xFFFD] = 1;
        cpu.c = true;
        cpu.a = 0x80;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 127, "A reg");
        assert_eq!(cpu.c, true, "c reg");
        assert_eq!(cpu.n, false, "n reg");
        assert_eq!(cpu.v, true, "v reg");
        assert_eq!(cpu.z, false, "z reg");
    }

    #[test]
    fn test_sbc_6() {
        // test overflow
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::SBC_IM;
        mem[0xFFFD] = 0b1111_1111; // -1
        cpu.c = true;
        cpu.a = 127;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 0x80, "A reg");  // 128 / -128
        assert_eq!(cpu.c, false, "c reg");
        assert_eq!(cpu.n, true, "n reg");
        assert_eq!(cpu.v, true, "v reg");
        assert_eq!(cpu.z, false, "z reg");
    }
    #[test]
    fn test_sbc_7() {
        // test overflow 2
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::SBC_IM;
        mem[0xFFFD] = 1;
        cpu.c = true;
        cpu.a = 0x80; // -128
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 127, "A reg"); // -128
        assert_eq!(cpu.c, true, "c reg");
        assert_eq!(cpu.n, false, "n reg");
        assert_eq!(cpu.v, true, "v reg");
        assert_eq!(cpu.z, false, "z reg");
    }
    #[test]
    fn test_sbc_8() {
        // neg - neg
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::SBC_IM;
        mem[0xFFFD] = -20i8 as u8;
        cpu.c = false;
        cpu.a = -32i8 as u8;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, -13i8 as u8, "A reg"); // -128
        assert_eq!(cpu.c, false, "c reg");
        assert_eq!(cpu.n, true, "n reg");
        assert_eq!(cpu.v, false, "v reg");
        assert_eq!(cpu.z, false, "z reg");
    }
}
