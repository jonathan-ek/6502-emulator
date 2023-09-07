use crate::cpu::CPU;

impl CPU {
    pub const ADC_IM: u8 = 0x69;
    pub const ADC_ZP: u8 = 0x65;
    pub const ADC_ZPX: u8 = 0x75;
    pub const ADC_ABS: u8 = 0x6D;
    pub const ADC_ABSX: u8 = 0x7D;
    pub const ADC_ABSY: u8 = 0x79;
    pub const ADC_INDX: u8 = 0x61;
    pub const ADC_INDY: u8 = 0x71;

    fn adc(&mut self, val: u8) {
        let val_neg = val & 0b1000_0000 > 0;
        let a_neg = self.a & 0b1000_0000 > 0;
        let bin_ans = self.a as u16 + val as u16 + self.c as u16;
        if self.d {
            let lo_nib_val = val & 0x0f;
            let hi_nib_val = (val & 0xf0) >> 4;
            let lo_nib_a = self.a & 0x0f;
            let hi_nib_a = (self.a & 0xf0) >> 4;
            let mut lo_nib_ans = lo_nib_a + lo_nib_val + self.c as u8;
            self.c = false;
            if lo_nib_ans > 9 {
                self.c = true;
                lo_nib_ans = (lo_nib_ans + 6) & 0x0f;
            }
            let mut hi_nib_ans = hi_nib_a + hi_nib_val + self.c as u8;
            self.c = false;
            if hi_nib_ans > 9 {
                self.c = true;
                hi_nib_ans = (hi_nib_ans + 6) & 0x0f;
            }
            self.a = (hi_nib_ans << 4) + lo_nib_ans;
            self.set_flags((0x00FF & bin_ans) as u8, CPU::FLAG_Z);
            self.set_flags(self.a, CPU::FLAG_N);
        } else {
            self.c = 0x0100 & bin_ans > 0;
            self.a = (0x00FF & bin_ans) as u8;
            let res_neg = self.a & 0b1000_0000 > 0;
            self.v = (val_neg && a_neg && !res_neg) || (!val_neg && !a_neg && res_neg);
            self.set_flags(self.a, CPU::FLAG_N | CPU::FLAG_Z);
        }
    }

    pub fn run_adc(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0x10000], inst: u8) -> bool {
        if inst == CPU::ADC_IM {
            let val = self.read_next_byte(&mut cycles, *mem);
            self.adc(val);
        } else if inst == CPU::ADC_ZP {
            let addr = self.read_zero_page_addr(&mut cycles, *mem);
            let val = self.read_byte(&mut cycles, *mem, addr);
            self.adc(val);
        } else if inst == CPU::ADC_ZPX {
            let addr = self.read_zero_page_x_addr(&mut cycles, *mem);
            let val = self.read_byte(&mut cycles, *mem, addr);
            self.adc(val);
        } else if inst == CPU::ADC_ABS {
            let addr = self.read_abs_addr(&mut cycles, *mem);
            let val = self.read_byte(&mut cycles, *mem, addr);
            self.adc(val);
        } else if inst == CPU::ADC_ABSX {
            let addr = self.read_abs_x_addr(&mut cycles, *mem, true);
            let val = self.read_byte(&mut cycles, *mem, addr);
            self.adc(val);
        } else if inst == CPU::ADC_ABSY {
            let addr = self.read_abs_y_addr(&mut cycles, *mem, true);
            let val = self.read_byte(&mut cycles, *mem, addr);
            self.adc(val);
        } else if inst == CPU::ADC_INDX {
            let addr = self.read_indexed_indirect_addr(&mut cycles, *mem);
            let val = self.read_byte(&mut cycles, *mem, addr);
            self.adc(val);
        } else if inst == CPU::ADC_INDY {
            let addr = self.read_indirect_indexed_addr(&mut cycles, *mem, true);
            let val = self.read_byte(&mut cycles, *mem, addr);
            self.adc(val);
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
    fn test_adc_bcd_1() {
        // SED      ; Decimal mode (BCD addition: 58 + 46 + 1 = 105)
        // SEC      ; Note: carry is set, not clear!
        // LDA #$58
        // ADC #$46 ; After this instruction, C = 1, A = $05
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        cpu.d = true;
        mem[0xFFFC] = CPU::ADC_IM;
        mem[0xFFFD] = 0x46;
        cpu.a = 0x58;
        cpu.c = true;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 0x05, "A reg");
        assert_eq!(cpu.c, true, "c reg");
    }
    #[test]
    fn test_adc_bcd_2() {
        // SED      ; Decimal mode (BCD addition: 12 + 34 = 46)
        // CLC
        // LDA #$12
        // ADC #$34 ; After this instruction, C = 0, A = $46
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        cpu.d = true;
        mem[0xFFFC] = CPU::ADC_IM;
        mem[0xFFFD] = 0x34;
        cpu.a = 0x12;
        cpu.c = false;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 0x46, "A reg");
        assert_eq!(cpu.c, false, "c reg");
    }
    #[test]
    fn test_adc_bcd_3() {
        // SED      ; Decimal mode (BCD addition: 15 + 26 = 41)
        // CLC
        // LDA #$15
        // ADC #$26 ; After this instruction, C = 0, A = $41
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        cpu.d = true;
        mem[0xFFFC] = CPU::ADC_IM;
        mem[0xFFFD] = 0x26;
        cpu.a = 0x15;
        cpu.c = false;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 0x41, "A reg");
        assert_eq!(cpu.c, false, "c reg");
    }
    #[test]
    fn test_adc_bcd_4() {
        // SED      ; Decimal mode (BCD addition: 81 + 92 = 173)
        // CLC
        // LDA #$81
        // ADC #$92 ; After this instruction, C = 1, A = $73
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        cpu.d = true;
        mem[0xFFFC] = CPU::ADC_IM;
        mem[0xFFFD] = 0x92;
        cpu.a = 0x81;
        cpu.c = false;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 0x73, "A reg");
        assert_eq!(cpu.c, true, "c reg");
    }
    #[test]
    fn test_adc_1() {
        // 0 + 0 = 0
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::ADC_IM;
        mem[0xFFFD] = 0x00;
        cpu.c = false;
        cpu.a = 0;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 0, "A reg");
        assert_eq!(cpu.c, false, "c reg");
        assert_eq!(cpu.n, false, "n reg");
        assert_eq!(cpu.v, false, "v reg");
        assert_eq!(cpu.z, true, "z reg");
    }
    #[test]
    fn test_adc_2() {
        // 0 + 0 + c = 1
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::ADC_IM;
        mem[0xFFFD] = 0x00;
        cpu.c = true;
        cpu.a = 0;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 1, "A reg");
        assert_eq!(cpu.c, false, "c reg");
        assert_eq!(cpu.n, false, "n reg");
        assert_eq!(cpu.v, false, "v reg");
        assert_eq!(cpu.z, false, "z reg");
    }
    #[test]
    fn test_adc_3() {
        // pos + pos + c
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::ADC_IM;
        mem[0xFFFD] = 20;
        cpu.c = true;
        cpu.a = 17;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 38, "A reg");
        assert_eq!(cpu.c, false, "c reg");
        assert_eq!(cpu.n, false, "n reg");
        assert_eq!(cpu.v, false, "v reg");
        assert_eq!(cpu.z, false, "z reg");
    }
    #[test]
    fn test_adc_4() {
        // pos + neg
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::ADC_IM;
        mem[0xFFFD] = 0b1110_1111;  // -17
        cpu.c = true;
        cpu.a = 20;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 4, "A reg");
        assert_eq!(cpu.c, true, "c reg");
        assert_eq!(cpu.n, false, "n reg");
        assert_eq!(cpu.v, false, "v reg");
        assert_eq!(cpu.z, false, "z reg");
    }
    #[test]
    fn test_adc_5() {
        // test carry
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::ADC_IM;
        mem[0xFFFD] = 0xff;
        cpu.c = false;
        cpu.a = 1;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 0, "A reg");
        assert_eq!(cpu.c, true, "c reg");
        assert_eq!(cpu.n, false, "n reg");
        assert_eq!(cpu.v, false, "v reg");
        assert_eq!(cpu.z, true, "z reg");
    }

    #[test]
    fn test_adc_6() {
        // test overflow
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::ADC_IM;
        mem[0xFFFD] = 0b1111_1111; // -1
        cpu.c = false;
        cpu.a = 0b1000_0000; // -128
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 0b01111111, "A reg");  // 127
        assert_eq!(cpu.c, true, "c reg");
        assert_eq!(cpu.n, false, "n reg");
        assert_eq!(cpu.v, true, "v reg");
        assert_eq!(cpu.z, false, "z reg");
    }
    #[test]
    fn test_adc_7() {
        // test overflow 2
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::ADC_IM;
        mem[0xFFFD] = 0b1111_1111; // -1
        cpu.c = true;
        cpu.a = 0b1000_0000; // -128
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 0b1000_0000, "A reg"); // -128
        assert_eq!(cpu.c, true, "c reg");
        assert_eq!(cpu.n, true, "n reg");
        assert_eq!(cpu.v, false, "v reg");
        assert_eq!(cpu.z, false, "z reg");
    }
    #[test]
    fn test_adc_8() {
        // test overflow 2
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::ADC_IM;
        mem[0xFFFD] = 0x01; // 1
        cpu.c = false;
        cpu.a = 0b0111_1111; // 127
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 0b1000_0000, "A reg"); // -128
        assert_eq!(cpu.c, false, "c reg");
        assert_eq!(cpu.n, true, "n reg");
        assert_eq!(cpu.v, true, "v reg");
        assert_eq!(cpu.z, false, "z reg");
    }
    #[test]
    fn test_adc_9() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::ADC_IM;
        mem[0xFFFD] = 0b00000001; // 1
        cpu.c = false;
        cpu.a = 0b0111_1111; // 127
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 0b1000_0000, "A reg"); // 128
        assert_eq!(cpu.c, false, "c reg");
        assert_eq!(cpu.n, true, "n reg");
        assert_eq!(cpu.v, true, "v reg");
        assert_eq!(cpu.z, false, "z reg");
    }
}
