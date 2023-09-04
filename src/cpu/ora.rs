use crate::cpu::CPU;

impl CPU {
    pub const ORA_IM: u8 = 0x09;
    pub const ORA_ZP: u8 = 0x05;
    pub const ORA_ZPX: u8 = 0x15;
    pub const ORA_ABS: u8 = 0x0D;
    pub const ORA_ABSX: u8 = 0x1D;
    pub const ORA_ABSY: u8 = 0x19;
    pub const ORA_INDX: u8 = 0x01;
    pub const ORA_INDY: u8 = 0x11;

    pub fn run_ora(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0x10000], inst: u8) -> bool {
        if inst == CPU::ORA_IM {
            let value = self.read_next_byte(&mut cycles, *mem);
            self.a = self.a | value;
            self.set_flags(self.a, CPU::FLAG_N | CPU::FLAG_Z);
        } else if inst == CPU::ORA_ZP {
            let addr = self.read_zero_page_addr(&mut cycles, *mem);
            let value = self.read_byte(&mut cycles, *mem, addr);
            self.a = self.a | value;
            self.set_flags(self.a, CPU::FLAG_N | CPU::FLAG_Z);
        } else if inst == CPU::ORA_ZPX {
            let addr = self.read_zero_page_x_addr(&mut cycles, *mem);
            let value = self.read_byte(&mut cycles, *mem, addr);
            self.a = self.a | value;
            self.set_flags(self.a, CPU::FLAG_N | CPU::FLAG_Z);
        } else if inst == CPU::ORA_ABS {
            let addr = self.read_abs_addr(&mut cycles, *mem);
            let value = self.read_byte(&mut cycles, *mem, addr);
            self.a = self.a | value;
            self.set_flags(self.a, CPU::FLAG_N | CPU::FLAG_Z);
        } else if inst == CPU::ORA_ABSX {
            let addr = self.read_abs_x_addr(&mut cycles, *mem, true);
            let value = self.read_byte(&mut cycles, *mem, addr);
            self.a = self.a | value;
            self.set_flags(self.a, CPU::FLAG_N | CPU::FLAG_Z);
        } else if inst == CPU::ORA_ABSY {
            let addr = self.read_abs_y_addr(&mut cycles, *mem, true);
            let value = self.read_byte(&mut cycles, *mem, addr);
            self.a = self.a | value;
            self.set_flags(self.a, CPU::FLAG_N | CPU::FLAG_Z);
        } else if inst == CPU::ORA_INDX {
            let addr = self.read_indexed_indirect_addr(&mut cycles, *mem);
            let value = self.read_byte(&mut cycles, *mem, addr);
            self.a = self.a | value;
            self.set_flags(self.a, CPU::FLAG_N | CPU::FLAG_Z);
        } else if inst == CPU::ORA_INDY {
            let addr = self.read_indirect_indexed_addr(&mut cycles, *mem, true);
            let value = self.read_byte(&mut cycles, *mem, addr);
            self.a = self.a | value;
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
    fn test_ora_absx() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::ORA_ABSX;
        mem[0xFFFD] = 0x12;
        mem[0xFFFE] = 0x55;
        mem[0x5606] = 0b1010_1101;
        cpu.x = 0xF4;
        cpu.a = 0b1111_0000;
        let cycles = 5;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.a, 0b1111_1101, "a reg");
        assert_eq!(cpu.n, true, "n reg");
    }
}