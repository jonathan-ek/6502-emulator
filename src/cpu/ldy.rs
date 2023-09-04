use crate::cpu::CPU;

impl CPU {
    pub const LDY_IM: u8 = 0xA0;
    pub const LDY_ZP: u8 = 0xA4;
    pub const LDY_ZPX: u8 = 0xB4;
    pub const LDY_ABS: u8 = 0xAC;
    pub const LDY_ABSX: u8 = 0xBC;

    pub fn run_ldy(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0x10000], inst: u8) -> bool {
        if inst == CPU::LDY_IM {
            self.y = self.read_next_byte(&mut cycles, *mem);
            self.set_flags(self.y, CPU::FLAG_N | CPU::FLAG_Z);
        } else if inst == CPU::LDY_ZP {
            self.y = self.read_zero_page(&mut cycles, *mem);
            self.set_flags(self.y, CPU::FLAG_N | CPU::FLAG_Z);
        } else if inst == CPU::LDY_ZPX {
            self.y = self.read_zero_page_x(&mut cycles, *mem);
            self.set_flags(self.y, CPU::FLAG_N | CPU::FLAG_Z);
        } else if inst == CPU::LDY_ABS {
            self.y = self.read_abs(&mut cycles, *mem);
            self.set_flags(self.y, CPU::FLAG_N | CPU::FLAG_Z);
        } else if inst == CPU::LDY_ABSX {
            self.y = self.read_abs_x(&mut cycles, *mem);
            self.set_flags(self.y, CPU::FLAG_N | CPU::FLAG_Z);
        } else {
            return false;
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::CPU;

    //     LDY
    #[test]
    fn test_ldy_im_1() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::LDY_IM;
        mem[0xFFFD] = 0x48;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.y, 0x48, "Y reg");
        assert_eq!(cpu.n, false, "N flag");
        assert_eq!(cpu.z, false, "Z flag");
    }

    #[test]
    fn test_ldy_im_2() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::LDY_IM;
        mem[0xFFFD] = 0b10000001;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.y, 0b10000001, "Y reg");
        assert_eq!(cpu.n, true, "N flag");
        assert_eq!(cpu.z, false, "Z flag");
    }

    #[test]
    fn test_ldy_im_3() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::LDY_IM;
        mem[0xFFFD] = 0b00000000;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.y, 0, "Y reg");
        assert_eq!(cpu.n, false, "N flag");
        assert_eq!(cpu.z, true, "Z flag");
    }

    #[test]
    fn test_ldy_zp_1() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::LDY_ZP;
        mem[0xFFFD] = 0x48;
        mem[0x0048] = 0x42;
        let cycles = 3;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.y, 0x42, "Y reg");
    }

    #[test]
    fn test_ldy_zpx_1() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::LDY_ZPX;
        mem[0xFFFD] = 0x80;
        mem[0x008F] = 0x41;
        let cycles = 4;
        cpu.x = 0x0f;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.y, 0x41, "Y reg");
    }

    #[test]
    fn test_ldy_zpx_2() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::LDY_ZPX;
        mem[0xFFFD] = 0x80;
        mem[0x007F] = 0x40;
        let cycles = 4;
        cpu.x = 0xff;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.y, 0x40, "Y reg");
    }

    #[test]
    fn test_ldy_abs() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::LDY_ABS;
        mem[0xFFFD] = 0x80;
        mem[0xFFFE] = 0x54;
        mem[0x5480] = 0x39;
        let cycles = 4;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.y, 0x39, "Y reg");
    }

    #[test]
    fn test_ldy_absx_1() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::LDY_ABSX;
        mem[0xFFFD] = 0x80;
        mem[0xFFFE] = 0x54;
        mem[0x5483] = 0x39;
        cpu.x = 0x03;
        let cycles = 4;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.y, 0x39, "Y reg");
    }

    #[test]
    fn test_ldy_absx_2() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::LDY_ABSX;
        mem[0xFFFD] = 0x80;
        mem[0xFFFE] = 0x54;
        mem[0x5523] = 0x38;
        cpu.x = 0xA3;
        let cycles = 5;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.y, 0x38, "Y reg");
    }
}