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
        let tmp = self.a as u16 + val as u16 + self.c as u16;
        self.c = 0x0100 & tmp > 0;
        self.a = (0x00FF & tmp) as u8;
        let res_neg = self.a & 0b1000_0000 > 0;
        self.v = (val_neg && a_neg && !res_neg) || (!val_neg && !a_neg && res_neg);
        self.set_flags(self.a, CPU::FLAG_N | CPU::FLAG_Z);
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
