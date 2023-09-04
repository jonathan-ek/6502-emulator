use crate::cpu::CPU;

impl CPU {
    pub const LSR_A: u8 = 0x4A;
    pub const LSR_ZP: u8 = 0x46;
    pub const LSR_ZPX: u8 = 0x56;
    pub const LSR_ABS: u8 = 0x4e;
    pub const LSR_ABSX: u8 = 0x5e;

    pub fn run_lsr(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0x10000], inst: u8) -> bool {
        if inst == CPU::LSR_A {
            *cycles += 1;
            self.c = self.a & 0b0000_0001 == 1;
            self.a = self.a >> 1;
            self.set_flags(self.a, CPU::FLAG_N | CPU::FLAG_Z);
        } else if inst == CPU::LSR_ZP {
            let addr = self.read_zero_page_addr(&mut cycles, *mem);
            let mut value = self.read_byte(&mut cycles, *mem, addr);
            *cycles += 1;
            self.c = value & 0b0000_0001 == 1;
            value = value >> 1;
            self.write_byte(&mut cycles, mem, addr, value);
            self.set_flags(self.a, CPU::FLAG_N | CPU::FLAG_Z);
        } else if inst == CPU::LSR_ZPX {
            let addr = self.read_zero_page_x_addr(&mut cycles, *mem);
            let mut value = self.read_byte(&mut cycles, *mem, addr);
            *cycles += 1;
            self.c = value & 0b0000_0001 == 1;
            value = value >> 1;
            self.write_byte(&mut cycles, mem, addr, value);
            self.set_flags(self.a, CPU::FLAG_N | CPU::FLAG_Z);
        } else if inst == CPU::LSR_ABS {
            let addr = self.read_abs_addr(&mut cycles, *mem);
            let mut value = self.read_byte(&mut cycles, *mem, addr);
            *cycles += 1;
            self.c = value & 0b0000_0001 == 1;
            value = value >> 1;
            self.write_byte(&mut cycles, mem, addr, value);
            self.set_flags(self.a, CPU::FLAG_N | CPU::FLAG_Z);
        } else if inst == CPU::LSR_ABSX {
            let addr = self.read_abs_x_addr(&mut cycles, *mem, false);
            let mut value = self.read_byte(&mut cycles, *mem, addr);
            *cycles += 2;
            self.c = value & 0b0000_0001 == 1;
            value = value >> 1;
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
    fn test_lsr_a() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::LSR_A;
        cpu.a = 0b1010_1100;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 0b0101_0110, "a reg");
        assert_eq!(cpu.c, false, "c reg");
    }

    #[test]
    fn test_lsr_a_2() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::LSR_A;
        cpu.a = 0b1010_1101;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 0b0101_0110, "a reg");
        assert_eq!(cpu.c, true, "c reg");
    }

    #[test]
    fn test_lsr_zp() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::LSR_ZP;
        mem[0xFFFD] = 0x12;
        mem[0x0012] = 0b1010_1101;
        let cycles = 5;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(mem[0x0012], 0b0101_0110, "a reg");
        assert_eq!(cpu.c, true, "c reg");
    }

    #[test]
    fn test_lsr_zpx() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::LSR_ZPX;
        mem[0xFFFD] = 0x12;
        mem[0x0036] = 0b1010_1101;
        cpu.x = 0x24;
        let cycles = 6;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(mem[0x0036], 0b0101_0110, "a reg");
        assert_eq!(cpu.c, true, "c reg");
    }

    #[test]
    fn test_lsr_abs() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::LSR_ABS;
        mem[0xFFFD] = 0x12;
        mem[0xFFFE] = 0x34;
        mem[0x3412] = 0b0000_0001;
        let cycles = 6;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(mem[0x3412], 0, "a reg");
        assert_eq!(cpu.c, true, "c reg");
    }

    #[test]
    fn test_lsr_absx() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::LSR_ABSX;
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