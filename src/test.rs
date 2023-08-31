#[cfg(test)]
mod tests {
    use crate::cpu::CPU;

    // LDA
    #[test]
    fn test_lda_im_1() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::LDA_IM;
        mem[0xFFFD] = 0x48;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, mem), cycles);
        assert_eq!(cpu.a, 0x48, "A reg");
        assert_eq!(cpu.n, false, "N flag");
        assert_eq!(cpu.z, false, "Z flag");
    }

    #[test]
    fn test_lda_im_2() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::LDA_IM;
        mem[0xFFFD] = 0b10000001;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, mem), cycles);
        assert_eq!(cpu.a, 0b10000001, "A reg");
        assert_eq!(cpu.n, true, "N flag");
        assert_eq!(cpu.z, false, "Z flag");
    }

    #[test]
    fn test_lda_im_3() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::LDA_IM;
        mem[0xFFFD] = 0b00000000;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, mem), cycles);
        assert_eq!(cpu.a, 0, "A reg");
        assert_eq!(cpu.n, false, "N flag");
        assert_eq!(cpu.z, true, "Z flag");
    }

    #[test]
    fn test_lda_zp_1() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::LDA_ZP;
        mem[0xFFFD] = 0x48;
        mem[0x0048] = 0x42;
        let cycles = 3;
        assert_eq!(cpu.run(cycles, mem), cycles);
        assert_eq!(cpu.a, 0x42, "A reg");
    }

    #[test]
    fn test_lda_zpx_1() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::LDA_ZPX;
        mem[0xFFFD] = 0x80;
        mem[0x008F] = 0x41;
        let cycles = 4;
        cpu.x = 0x0f;
        assert_eq!(cpu.run(cycles, mem), cycles);
        assert_eq!(cpu.a, 0x41, "A reg");
    }

    #[test]
    fn test_lda_zpx_2() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::LDA_ZPX;
        mem[0xFFFD] = 0x80;
        mem[0x007F] = 0x40;
        let cycles = 4;
        cpu.x = 0xff;
        assert_eq!(cpu.run(cycles, mem), cycles);
        assert_eq!(cpu.a, 0x40, "A reg");
    }

    #[test]
    fn test_lda_abs() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::LDA_ABS;
        mem[0xFFFD] = 0x80;
        mem[0xFFFE] = 0x54;
        mem[0x5480] = 0x39;
        let cycles = 4;
        assert_eq!(cpu.run(cycles, mem), cycles);
        assert_eq!(cpu.a, 0x39, "A reg");
    }

    #[test]
    fn test_lda_absx_1() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::LDA_ABSX;
        mem[0xFFFD] = 0x80;
        mem[0xFFFE] = 0x54;
        mem[0x5483] = 0x39;
        cpu.x = 0x03;
        let cycles = 4;
        assert_eq!(cpu.run(cycles, mem), cycles);
        assert_eq!(cpu.a, 0x39, "A reg");
    }

    #[test]
    fn test_lda_absx_2() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::LDA_ABSX;
        mem[0xFFFD] = 0x80;
        mem[0xFFFE] = 0x54;
        mem[0x5523] = 0x38;
        cpu.x = 0xA3;
        let cycles = 5;
        assert_eq!(cpu.run(cycles, mem), cycles);
        assert_eq!(cpu.a, 0x38, "A reg");
    }

    #[test]
    fn test_lda_absy_1() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::LDA_ABSY;
        mem[0xFFFD] = 0x80;
        mem[0xFFFE] = 0x54;
        mem[0x5483] = 0x39;
        cpu.y = 0x03;
        let cycles = 4;
        assert_eq!(cpu.run(cycles, mem), cycles);
        assert_eq!(cpu.a, 0x39, "A reg");
    }

    #[test]
    fn test_lda_absy_2() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::LDA_ABSY;
        mem[0xFFFD] = 0x80;
        mem[0xFFFE] = 0x54;
        mem[0x5523] = 0x38;
        cpu.y = 0xA3;
        let cycles = 5;
        assert_eq!(cpu.run(cycles, mem), cycles);
        assert_eq!(cpu.a, 0x38, "A reg");
    }

    #[test]
    fn test_lda_indx_1() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::LDA_INDX;
        mem[0xFFFD] = 0x80;
        mem[0x0090] = 0x54;
        mem[0x0091] = 0x32;
        mem[0x3254] = 0x38;
        cpu.x = 0x10;
        let cycles = 6;
        assert_eq!(cpu.run(cycles, mem), cycles);
        assert_eq!(cpu.a, 0x38, "A reg");
    }

    #[test]
    fn test_lda_indx_2() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::LDA_INDX;
        mem[0xFFFD] = 0x80;
        mem[0x0072] = 0x54;
        mem[0x0073] = 0x32;
        mem[0x3254] = 0x38;
        cpu.x = 0xF2;
        let cycles = 6;
        assert_eq!(cpu.run(cycles, mem), cycles);
        assert_eq!(cpu.a, 0x38, "A reg");
    }

    #[test]
    fn test_lda_indy_1() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::LDA_INDY;
        mem[0xFFFD] = 0x80;
        mem[0x0080] = 0x54;
        mem[0x0081] = 0x32;
        mem[0x3264] = 0x38;
        cpu.y = 0x10;
        let cycles = 5;
        assert_eq!(cpu.run(cycles, mem), cycles);
        assert_eq!(cpu.a, 0x38, "A reg");
    }

    #[test]
    fn test_lda_indy_2() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::LDA_INDY;
        mem[0xFFFD] = 0x80;
        mem[0x0080] = 0x54;
        mem[0x0081] = 0x32;
        mem[0x3344] = 0x38;
        cpu.y = 0xF0;
        let cycles = 6;
        assert_eq!(cpu.run(cycles, mem), cycles);
        assert_eq!(cpu.a, 0x38, "A reg");
    }

    //     LDX
    #[test]
    fn test_ldx_im_1() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::LDX_IM;
        mem[0xFFFD] = 0x48;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, mem), cycles);
        assert_eq!(cpu.x, 0x48, "X reg");
        assert_eq!(cpu.n, false, "N flag");
        assert_eq!(cpu.z, false, "Z flag");
    }

    #[test]
    fn test_ldx_im_2() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::LDX_IM;
        mem[0xFFFD] = 0b10000001;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, mem), cycles);
        assert_eq!(cpu.x, 0b10000001, "X reg");
        assert_eq!(cpu.n, true, "N flag");
        assert_eq!(cpu.z, false, "Z flag");
    }

    #[test]
    fn test_ldx_im_3() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::LDX_IM;
        mem[0xFFFD] = 0b00000000;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, mem), cycles);
        assert_eq!(cpu.x, 0, "X reg");
        assert_eq!(cpu.n, false, "N flag");
        assert_eq!(cpu.z, true, "Z flag");
    }

    #[test]
    fn test_ldx_zp_1() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::LDX_ZP;
        mem[0xFFFD] = 0x48;
        mem[0x0048] = 0x42;
        let cycles = 3;
        assert_eq!(cpu.run(cycles, mem), cycles);
        assert_eq!(cpu.x, 0x42, "X reg");
    }

    #[test]
    fn test_ldx_zpy_1() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::LDX_ZPY;
        mem[0xFFFD] = 0x80;
        mem[0x008F] = 0x41;
        let cycles = 4;
        cpu.y = 0x0f;
        assert_eq!(cpu.run(cycles, mem), cycles);
        assert_eq!(cpu.x, 0x41, "X reg");
    }

    #[test]
    fn test_ldx_zpy_2() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::LDX_ZPY;
        mem[0xFFFD] = 0x80;
        mem[0x007F] = 0x40;
        let cycles = 4;
        cpu.y = 0xff;
        assert_eq!(cpu.run(cycles, mem), cycles);
        assert_eq!(cpu.x, 0x40, "X reg");
    }

    #[test]
    fn test_ldx_abs() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::LDX_ABS;
        mem[0xFFFD] = 0x80;
        mem[0xFFFE] = 0x54;
        mem[0x5480] = 0x39;
        let cycles = 4;
        assert_eq!(cpu.run(cycles, mem), cycles);
        assert_eq!(cpu.x, 0x39, "X reg");
    }

    #[test]
    fn test_ldx_absy_1() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::LDX_ABSY;
        mem[0xFFFD] = 0x80;
        mem[0xFFFE] = 0x54;
        mem[0x5483] = 0x39;
        cpu.y = 0x03;
        let cycles = 4;
        assert_eq!(cpu.run(cycles, mem), cycles);
        assert_eq!(cpu.x, 0x39, "A reg");
    }

    #[test]
    fn test_ldx_absy_2() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::LDX_ABSY;
        mem[0xFFFD] = 0x80;
        mem[0xFFFE] = 0x54;
        mem[0x5523] = 0x38;
        cpu.y = 0xA3;
        let cycles = 5;
        assert_eq!(cpu.run(cycles, mem), cycles);
        assert_eq!(cpu.x, 0x38, "A reg");
    }
    //     LDY
    #[test]
    fn test_ldy_im_1() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::LDY_IM;
        mem[0xFFFD] = 0x48;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, mem), cycles);
        assert_eq!(cpu.y, 0x48, "Y reg");
        assert_eq!(cpu.n, false, "N flag");
        assert_eq!(cpu.z, false, "Z flag");
    }

    #[test]
    fn test_ldy_im_2() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::LDY_IM;
        mem[0xFFFD] = 0b10000001;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, mem), cycles);
        assert_eq!(cpu.y, 0b10000001, "Y reg");
        assert_eq!(cpu.n, true, "N flag");
        assert_eq!(cpu.z, false, "Z flag");
    }

    #[test]
    fn test_ldy_im_3() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::LDY_IM;
        mem[0xFFFD] = 0b00000000;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, mem), cycles);
        assert_eq!(cpu.y, 0, "Y reg");
        assert_eq!(cpu.n, false, "N flag");
        assert_eq!(cpu.z, true, "Z flag");
    }

    #[test]
    fn test_ldy_zp_1() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::LDY_ZP;
        mem[0xFFFD] = 0x48;
        mem[0x0048] = 0x42;
        let cycles = 3;
        assert_eq!(cpu.run(cycles, mem), cycles);
        assert_eq!(cpu.y, 0x42, "Y reg");
    }

    #[test]
    fn test_ldy_zpx_1() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::LDY_ZPX;
        mem[0xFFFD] = 0x80;
        mem[0x008F] = 0x41;
        let cycles = 4;
        cpu.x = 0x0f;
        assert_eq!(cpu.run(cycles, mem), cycles);
        assert_eq!(cpu.y, 0x41, "Y reg");
    }

    #[test]
    fn test_ldy_zpx_2() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::LDY_ZPX;
        mem[0xFFFD] = 0x80;
        mem[0x007F] = 0x40;
        let cycles = 4;
        cpu.x = 0xff;
        assert_eq!(cpu.run(cycles, mem), cycles);
        assert_eq!(cpu.y, 0x40, "Y reg");
    }

    #[test]
    fn test_ldy_abs() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::LDY_ABS;
        mem[0xFFFD] = 0x80;
        mem[0xFFFE] = 0x54;
        mem[0x5480] = 0x39;
        let cycles = 4;
        assert_eq!(cpu.run(cycles, mem), cycles);
        assert_eq!(cpu.y, 0x39, "Y reg");
    }

    #[test]
    fn test_ldy_absx_1() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::LDY_ABSX;
        mem[0xFFFD] = 0x80;
        mem[0xFFFE] = 0x54;
        mem[0x5483] = 0x39;
        cpu.x = 0x03;
        let cycles = 4;
        assert_eq!(cpu.run(cycles, mem), cycles);
        assert_eq!(cpu.y, 0x39, "Y reg");
    }

    #[test]
    fn test_ldy_absx_2() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::LDY_ABSX;
        mem[0xFFFD] = 0x80;
        mem[0xFFFE] = 0x54;
        mem[0x5523] = 0x38;
        cpu.x = 0xA3;
        let cycles = 5;
        assert_eq!(cpu.run(cycles, mem), cycles);
        assert_eq!(cpu.y, 0x38, "Y reg");
    }
    #[test]
    fn test_nop() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::NOP;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, mem), cycles);
    }
}