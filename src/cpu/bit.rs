use crate::cpu::CPU;

impl CPU {
    pub const BIT_ZP: u8 = 0x24;
    pub const BIT_ABS: u8 = 0x2C;

    pub fn run_bit(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0x10000], inst: u8) -> bool {
        if inst == CPU::BIT_ZP {
            let addr = self.read_zero_page_addr(&mut cycles, *mem);
            let val = self.read_byte(&mut cycles, *mem, addr);
            self.n = (val & 0b1000_0000) > 0;
            self.v = (val & 0b0100_0000) > 0;
            self.z = (val & self.a) == 0;
        } else if inst == CPU::BIT_ABS {
            let addr = self.read_abs_addr(&mut cycles, *mem);
            let val = self.read_byte(&mut cycles, *mem, addr);
            self.n = (val & 0b1000_0000) > 0;
            self.v = (val & 0b0100_0000) > 0;
            self.z = (val & self.a) == 0;
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
    fn test_bit_1() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::BIT_ZP;
        mem[0xFFFD] = 0x12;
        mem[0x0012] = 0b1010_1010;
        cpu.a = 0b1111_0000;
        let cycles = 3;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.n, true, "n");
        assert_eq!(cpu.z, false, "z");
        assert_eq!(cpu.v, false, "v");
    }
    #[test]
    fn test_bit_2() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::BIT_ZP;
        mem[0xFFFD] = 0x12;
        mem[0x0012] = 0b1110_1010;
        cpu.a = 0;
        let cycles = 3;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.n, true, "n");
        assert_eq!(cpu.z, true, "z");
        assert_eq!(cpu.v, true, "v");
    }
    #[test]
    fn test_bit_3() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::BIT_ABS;
        mem[0xFFFD] = 0x12;
        mem[0xFFFE] = 0xAB;
        mem[0xAB12] = 0b0110_1010;
        cpu.a = 0b1010_1010;
        let cycles = 4;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.n, false, "n");
        assert_eq!(cpu.z, false, "z");
        assert_eq!(cpu.v, true, "v");
    }
}