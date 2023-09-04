use crate::cpu::CPU;

impl CPU {
    pub const LDX_IM: u8 = 0xA2;
    pub const LDX_ZP: u8 = 0xA6;
    pub const LDX_ZPY: u8 = 0xB6;
    pub const LDX_ABS: u8 = 0xAE;
    pub const LDX_ABSY: u8 = 0xBE;

    pub fn run_ldx(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0x10000], inst: u8) -> bool {
        if inst == CPU::LDX_IM {
            self.x = self.read_next_byte(&mut cycles, *mem);
            self.set_flags(self.x, CPU::FLAG_N | CPU::FLAG_Z);
        } else if inst == CPU::LDX_ZP {
            self.x = self.read_zero_page(&mut cycles, *mem);
            self.set_flags(self.x, CPU::FLAG_N | CPU::FLAG_Z);
        } else if inst == CPU::LDX_ZPY {
            self.x = self.read_zero_page_y(&mut cycles, *mem);
            self.set_flags(self.x, CPU::FLAG_N | CPU::FLAG_Z);
        } else if inst == CPU::LDX_ABS {
            self.x = self.read_abs(&mut cycles, *mem);
            self.set_flags(self.x, CPU::FLAG_N | CPU::FLAG_Z);
        } else if inst == CPU::LDX_ABSY {
            self.x = self.read_abs_y(&mut cycles, *mem);
            self.set_flags(self.x, CPU::FLAG_N | CPU::FLAG_Z);
        } else {
            return false;
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::CPU;

    //     LDX
    #[test]
    fn test_ldx_im_1() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::LDX_IM;
        mem[0xFFFD] = 0x48;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.x, 0x48, "X reg");
        assert_eq!(cpu.n, false, "N flag");
        assert_eq!(cpu.z, false, "Z flag");
    }

    #[test]
    fn test_ldx_im_2() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::LDX_IM;
        mem[0xFFFD] = 0b10000001;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.x, 0b10000001, "X reg");
        assert_eq!(cpu.n, true, "N flag");
        assert_eq!(cpu.z, false, "Z flag");
    }

    #[test]
    fn test_ldx_im_3() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::LDX_IM;
        mem[0xFFFD] = 0b00000000;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.x, 0, "X reg");
        assert_eq!(cpu.n, false, "N flag");
        assert_eq!(cpu.z, true, "Z flag");
    }

    #[test]
    fn test_ldx_zp_1() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::LDX_ZP;
        mem[0xFFFD] = 0x48;
        mem[0x0048] = 0x42;
        let cycles = 3;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.x, 0x42, "X reg");
    }

    #[test]
    fn test_ldx_zpy_1() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::LDX_ZPY;
        mem[0xFFFD] = 0x80;
        mem[0x008F] = 0x41;
        let cycles = 4;
        cpu.y = 0x0f;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.x, 0x41, "X reg");
    }

    #[test]
    fn test_ldx_zpy_2() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::LDX_ZPY;
        mem[0xFFFD] = 0x80;
        mem[0x007F] = 0x40;
        let cycles = 4;
        cpu.y = 0xff;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.x, 0x40, "X reg");
    }

    #[test]
    fn test_ldx_abs() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::LDX_ABS;
        mem[0xFFFD] = 0x80;
        mem[0xFFFE] = 0x54;
        mem[0x5480] = 0x39;
        let cycles = 4;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.x, 0x39, "X reg");
    }

    #[test]
    fn test_ldx_absy_1() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::LDX_ABSY;
        mem[0xFFFD] = 0x80;
        mem[0xFFFE] = 0x54;
        mem[0x5483] = 0x39;
        cpu.y = 0x03;
        let cycles = 4;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.x, 0x39, "A reg");
    }

    #[test]
    fn test_ldx_absy_2() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::LDX_ABSY;
        mem[0xFFFD] = 0x80;
        mem[0xFFFE] = 0x54;
        mem[0x5523] = 0x38;
        cpu.y = 0xA3;
        let cycles = 5;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.x, 0x38, "A reg");
    }
}