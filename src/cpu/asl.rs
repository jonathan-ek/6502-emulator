use crate::cpu::CPU;

impl CPU {
    pub const ASL_A: u8 = 0x0a;
    pub const ASL_ZP: u8 = 0x06;
    pub const ASL_ZPX: u8 = 0x16;
    pub const ASL_ABS: u8 = 0x0e;
    pub const ASL_ABSX: u8 = 0x1e;

    fn asl(&mut self, val: u8) -> u8 {
        self.c = val & 0b1000_0000 > 0;
        let res = (((val as u16) << 1) & 0x00FF) as u8;
        self.set_flags(res, CPU::FLAG_N | CPU::FLAG_Z);
        return res;
    }

    pub fn run_asl(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0xFFFF], inst: u8) -> bool {
        if inst == CPU::ASL_A {
            self.a = self.asl(self.a);
            *cycles += 1;
        } else if inst == CPU::ASL_ZP {
            let addr = self.read_zero_page_addr(&mut cycles, *mem);
            let value = self.read_byte(&mut cycles, *mem, addr);
            *cycles += 1;
            let val = self.asl(value);
            self.write_byte(&mut cycles, mem, addr, val);
        } else if inst == CPU::ASL_ZPX {
            let addr = self.read_zero_page_x_addr(&mut cycles, *mem);
            let value = self.read_byte(&mut cycles, *mem, addr);
            *cycles += 1;
            let val = self.asl(value);
            self.write_byte(&mut cycles, mem, addr, val);
        } else if inst == CPU::ASL_ABS {
            let addr = self.read_abs_addr(&mut cycles, *mem);
            let value = self.read_byte(&mut cycles, *mem, addr);
            *cycles += 1;
            let val = self.asl(value);
            self.write_byte(&mut cycles, mem, addr, val);
        } else if inst == CPU::ASL_ABSX {
            let addr = self.read_abs_x_addr(&mut cycles, *mem, false);
            let value = self.read_byte(&mut cycles, *mem, addr);
            *cycles += 2;
            let val = self.asl(value);
            self.write_byte(&mut cycles, mem, addr, val);
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
    fn test_asl_1() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::ASL_A;
        cpu.a = 0b1000_0000;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.c, true);
        assert_eq!(cpu.z, true);
        assert_eq!(cpu.n, false);
    }

    #[test]
    fn test_asl_2() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::ASL_A;
        cpu.a = 0b1100_0001;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 0b1000_0010);
        assert_eq!(cpu.c, true);
        assert_eq!(cpu.z, false);
        assert_eq!(cpu.n, true);
    }

    #[test]
    fn test_asl_3() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::ASL_A;
        cpu.a = 0b0100_0001;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 0b1000_0010);
        assert_eq!(cpu.c, false);
        assert_eq!(cpu.z, false);
        assert_eq!(cpu.n, true);
    }

    #[test]
    fn test_asl_4() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::ASL_ZP;
        mem[0xFFFD] = 0x01;
        mem[0x0001] = 0b0100_0001;
        let cycles = 5;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(mem[0x0001], 0b1000_0010);
        assert_eq!(cpu.c, false, "C");
        assert_eq!(cpu.z, false, "Z");
        assert_eq!(cpu.n, true, "N");
    }

    #[test]
    fn test_asl_zpx() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::ASL_ZPX;
        mem[0xFFFD] = 0x12;
        mem[0x0036] = 0b1010_1101;
        cpu.x = 0x24;
        let cycles = 6;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(mem[0x0036], 0b0101_1010, "a reg");
        assert_eq!(cpu.c, true, "c reg");
        assert_eq!(cpu.z, false, "Z");
        assert_eq!(cpu.n, false, "N");
    }

    #[test]
    fn test_asl_abs() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::ASL_ABS;
        mem[0xFFFD] = 0x12;
        mem[0xFFFE] = 0x34;
        mem[0x3412] = 0b0000_0001;
        let cycles = 6;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(mem[0x3412], 0b0000_0010, "a reg");
        assert_eq!(cpu.c, false, "c reg");
    }

    #[test]
    fn test_asl_absx() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::ASL_ABSX;
        mem[0xFFFD] = 0x12;
        mem[0xFFFE] = 0x55;
        mem[0x5536] = 0;
        cpu.x = 0x24;
        let cycles = 7;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(mem[0x5536], 0, "a reg");
    }
}
