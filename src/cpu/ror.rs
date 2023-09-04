use crate::cpu::CPU;

impl CPU {
    pub const ROR_A: u8 = 0x6A;
    pub const ROR_ZP: u8 = 0x66;
    pub const ROR_ZPX: u8 = 0x76;
    pub const ROR_ABS: u8 = 0x6E;
    pub const ROR_ABSX: u8 = 0x7E;

    pub fn run_ror(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0x10000], inst: u8) -> bool {
        if inst == CPU::ROR_A {
            *cycles += 1;
            let bit_7: u8 = if self.c { 0x80 } else { 0 };
            self.c = self.a & 0b0000_0001 == 1;
            self.a = (self.a >> 1) + bit_7;
            self.set_flags(self.a, CPU::FLAG_N | CPU::FLAG_Z);
        } else if inst == CPU::ROR_ZP {
            let addr = self.read_zero_page_addr(&mut cycles, *mem);
            let mut value = self.read_byte(&mut cycles, *mem, addr);
            let bit_7: u8 = if self.c { 0x80 } else { 0 };
            *cycles += 1;
            self.c = value & 0b0000_0001 == 1;
            value = (value >> 1) + bit_7;
            self.write_byte(&mut cycles, mem, addr, value);
            self.set_flags(self.a, CPU::FLAG_N | CPU::FLAG_Z);
        } else if inst == CPU::ROR_ZPX {
            let addr = self.read_zero_page_x_addr(&mut cycles, *mem);
            let mut value = self.read_byte(&mut cycles, *mem, addr);
            let bit_7: u8 = if self.c { 0x80 } else { 0 };
            *cycles += 1;
            self.c = value & 0b0000_0001 == 1;
            value = (value >> 1) + bit_7;
            self.write_byte(&mut cycles, mem, addr, value);
            self.set_flags(self.a, CPU::FLAG_N | CPU::FLAG_Z);
        } else if inst == CPU::ROR_ABS {
            let addr = self.read_abs_addr(&mut cycles, *mem);
            let mut value = self.read_byte(&mut cycles, *mem, addr);
            let bit_7: u8 = if self.c { 0x80 } else { 0 };
            *cycles += 1;
            self.c = value & 0b0000_0001 == 1;
            value = (value >> 1) + bit_7;
            self.write_byte(&mut cycles, mem, addr, value);
            self.set_flags(self.a, CPU::FLAG_N | CPU::FLAG_Z);
        } else if inst == CPU::ROR_ABSX {
            let addr = self.read_abs_x_addr(&mut cycles, *mem, false);
            let mut value = self.read_byte(&mut cycles, *mem, addr);
            let bit_7: u8 = if self.c { 0x80 } else { 0 };
            *cycles += 2;
            self.c = value & 0b0000_0001 == 1;
            value = (value >> 1) + bit_7;
            self.write_byte(&mut cycles, mem, addr, value);
            self.set_flags(self.a, CPU::FLAG_N | CPU::FLAG_Z);
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
    fn test_ror_a() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::ROR_A;
        cpu.a = 0b1010_1100;
        cpu.c = false;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 0b0101_0110, "a reg");
        assert_eq!(cpu.c, false, "c reg");
        assert_eq!(cpu.z, false, "z reg");
        assert_eq!(cpu.n, false, "n reg");
    }
    #[test]
    fn test_ror_a_c() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::ROR_A;
        cpu.a = 0b1010_1100;
        cpu.c = true;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 0b1101_0110, "a reg");
        assert_eq!(cpu.c, false, "c reg");
        assert_eq!(cpu.z, false, "z reg");
        assert_eq!(cpu.n, true, "n reg");
    }
    #[test]
    fn test_ror_a_c_2() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::ROR_A;
        cpu.a = 0;
        cpu.c = true;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 0b1000_0000, "a reg");
        assert_eq!(cpu.c, false, "c reg");
        assert_eq!(cpu.z, false, "z reg");
        assert_eq!(cpu.n, true, "n reg");
    }
    #[test]
    fn test_ror_a_c_3() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::ROR_A;
        cpu.a = 1;
        cpu.c = false;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 0, "a reg");
        assert_eq!(cpu.c, true, "c reg");
        assert_eq!(cpu.z, true, "z reg");
        assert_eq!(cpu.n, false, "n reg");
    }

    #[test]
    fn test_ror_a_2() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::ROR_A;
        cpu.a = 0b1010_1101;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 0b0101_0110, "a reg");
        assert_eq!(cpu.c, true, "c reg");
    }

    #[test]
    fn test_ror_zp() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::ROR_ZP;
        mem[0xFFFD] = 0x12;
        mem[0x0012] = 0b1010_1101;
        let cycles = 5;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(mem[0x0012], 0b0101_0110, "a reg");
        assert_eq!(cpu.c, true, "c reg");
    }

    #[test]
    fn test_ror_zpx() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::ROR_ZPX;
        mem[0xFFFD] = 0x12;
        mem[0x0036] = 0b1010_1101;
        cpu.x = 0x24;
        let cycles = 6;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(mem[0x0036], 0b0101_0110, "a reg");
        assert_eq!(cpu.c, true, "c reg");
    }

    #[test]
    fn test_ror_abs() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::ROR_ABS;
        mem[0xFFFD] = 0x12;
        mem[0xFFFE] = 0x34;
        mem[0x3412] = 0b0000_0001;
        let cycles = 6;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(mem[0x3412], 0, "a reg");
        assert_eq!(cpu.c, true, "c reg");
    }

    #[test]
    fn test_ror_absx() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::ROR_ABSX;
        mem[0xFFFD] = 0x12;
        mem[0xFFFE] = 0x55;
        mem[0x5536] = 0b1010_1101;
        cpu.x = 0x24;
        let cycles = 7;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(mem[0x5536], 0b0101_0110, "a reg");
        assert_eq!(cpu.c, true, "c reg");
    }
}