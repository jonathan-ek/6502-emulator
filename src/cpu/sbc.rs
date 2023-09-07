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
        let bin_ans = self.a as u16 + v as u16 + self.c as u16;
        if self.d {
            let lo_nib_val = val & 0x0f;
            let hi_nib_val = (val & 0xf0) >> 4;
            let lo_nib_a = self.a & 0x0f;
            let hi_nib_a = (self.a & 0xf0) >> 4;
            let mut lo_nib_ans = lo_nib_a.wrapping_sub(lo_nib_val).wrapping_sub((!self.c) as u8);
            let mut rest: u8 = 0x00;
            if lo_nib_ans > 9 {
                lo_nib_ans = lo_nib_ans.wrapping_sub(6) & 0x0f;
                rest = 0x01;
            }
            let mut hi_nib_ans = hi_nib_a.wrapping_sub(hi_nib_val).wrapping_sub(rest);
            if hi_nib_ans > 9 {
                hi_nib_ans = hi_nib_ans.wrapping_sub(6) & 0x0f;
            }
            self.a = (hi_nib_ans << 4) + lo_nib_ans;
            self.c = 0x0100 & bin_ans > 0;
            let bin_a = (0x00FF & bin_ans) as u8;
            let res_neg = bin_a & 0b1000_0000 > 0;
            self.v = (val_neg && a_neg && !res_neg) || (!val_neg && !a_neg && res_neg);
            self.set_flags(bin_a, CPU::FLAG_N | CPU::FLAG_Z);
        } else {
            self.c = 0x0100 & bin_ans > 0;
            self.a = (0x00FF & bin_ans) as u8;
            let res_neg = self.a & 0b1000_0000 > 0;
            self.v = (val_neg && a_neg && !res_neg) || (!val_neg && !a_neg && res_neg);
            self.set_flags(self.a, CPU::FLAG_N | CPU::FLAG_Z);
        }
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
    fn test_sbc_bcd_1() {
        // SED      ; Decimal mode (BCD subtraction: 46 - 12 = 34)
        // SEC
        // LDA #$46
        // SBC #$12 ; After this instruction, C = 1, A = $34)
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        cpu.d = true;
        mem[0xFFFC] = CPU::SBC_IM;
        mem[0xFFFD] = 0x12;
        cpu.c = true;
        cpu.a = 0x46;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 0x34, "A reg");
        assert_eq!(cpu.c, true, "c reg");
    }
    #[test]
    fn test_sbc_bcd_2() {
        // SED      ; Decimal mode (BCD subtraction: 40 - 13 = 27)
        // SEC
        // LDA #$40
        // SBC #$13 ; After this instruction, C = 1, A = $27)
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        cpu.d = true;
        mem[0xFFFC] = CPU::SBC_IM;
        mem[0xFFFD] = 0x13;
        cpu.c = true;
        cpu.a = 0x40;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 0x27, "A reg");
        assert_eq!(cpu.c, true, "c reg");
    }
    #[test]
    fn test_sbc_bcd_3() {
        // SED      ; Decimal mode (BCD subtraction: 32 - 2 - 1 = 29)
        // CLC      ; Note: carry is clear, not set!
        // LDA #$32
        // SBC #$02 ; After this instruction, C = 1, A = $29)
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        cpu.d = true;
        mem[0xFFFC] = CPU::SBC_IM;
        mem[0xFFFD] = 0x02;
        cpu.c = false;
        cpu.a = 0x32;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 0x29, "A reg");
        assert_eq!(cpu.c, true, "c reg");
    }
    #[test]
    fn test_sbc_bcd_4() {
        // SED      ; Decimal mode (BCD subtraction: 12 - 21)
        // SEC
        // LDA #$12
        // SBC #$21 ; After this instruction, C = 0, A = $91)
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        cpu.d = true;
        mem[0xFFFC] = CPU::SBC_IM;
        mem[0xFFFD] = 0x21;
        cpu.c = true;
        cpu.a = 0x12;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 0x91, "A reg");
        assert_eq!(cpu.c, false, "c reg");
    }
    #[test]
    fn test_sbc_bcd_5() {
        // SED      ; Decimal mode (BCD subtraction: 21 - 34)
        // SEC
        // LDA #$21
        // SBC #$34 ; After this instruction, C = 0, A = $87)
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        cpu.d = true;
        mem[0xFFFC] = CPU::SBC_IM;
        mem[0xFFFD] = 0x34;
        cpu.c = true;
        cpu.a = 0x21;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 0x87, "A reg");
        assert_eq!(cpu.c, false, "c reg");
    }
    #[test]
    fn test_sbc_bcd_6() {
        // SED      ; Decimal mode (BCD subtraction: 46 - 12 = 34)
        // SEC
        // LDA #$00
        // SBC #$60 ; After this instruction, C = 1, A = $34)
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        cpu.d = true;
        mem[0xFFFC] = CPU::SBC_IM;
        mem[0xFFFD] = 0x60;
        cpu.c = false;
        cpu.a = 0x00;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 0x39, "A reg");
        assert_eq!(cpu.c, false, "c reg");
    }
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
