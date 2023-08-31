pub struct CPU {
    pub(crate) pc: u16,
    pub(crate) sp: u8,
    pub(crate) a: u8,
    pub(crate) x: u8,
    pub(crate) y: u8,
    pub(crate) c: bool,
    pub(crate) z: bool,
    pub(crate) i: bool,
    pub(crate) d: bool,
    pub(crate) b: bool,
    pub(crate) v: bool,
    pub(crate) n: bool,
}
impl CPU {
    pub fn new() -> CPU {
        CPU {
            pc: 0xFFFC,
            sp: 0,
            a: 0,
            x: 0,
            y: 0,
            c: false,
            z: false,
            i: false,
            d: false,
            b: false,
            v: false,
            n: false,
        }
    }

    const FLAG_C: u8 = 0b1000000;
    const FLAG_Z: u8 = 0b0100000;
    const FLAG_I: u8 = 0b0010000;
    const FLAG_D: u8 = 0b0001000;
    const FLAG_B: u8 = 0b0000100;
    const FLAG_V: u8 = 0b0000010;
    const FLAG_N: u8 = 0b0000001;
    pub const LDA_IM: u8 = 0xA9;
    pub const LDA_ZP: u8 = 0xA5;
    pub const LDA_ZPX: u8 = 0xB5;
    pub const LDA_ABS: u8 = 0xAD;
    pub const LDA_ABSX: u8 = 0xBD;
    pub const LDA_ABSY: u8 = 0xB9;
    pub const LDA_INDX: u8 = 0xA1;
    pub const LDA_INDY: u8 = 0xB1;

    pub const LDX_IM: u8 = 0xA2;
    pub const LDX_ZP: u8 = 0xA6;
    pub const LDX_ZPY: u8 = 0xB6;
    pub const LDX_ABS: u8 = 0xAE;
    pub const LDX_ABSY: u8 = 0xBE;
    pub const LDY_IM: u8 = 0xA0;
    pub const LDY_ZP: u8 = 0xA4;
    pub const LDY_ZPX: u8 = 0xB4;
    pub const LDY_ABS: u8 = 0xAC;
    pub const LDY_ABSX: u8 = 0xBC;
    pub const NOP: u8 = 0xEA;
    pub const LSR_A: u8 = 0x4A;
    pub const LSR_ZP: u8 = 0x46;
    pub const LSR_ZPX: u8 = 0x56;
    pub const LSR_ABS: u8 = 0x4e;
    pub const LSR_ABSX: u8 = 0x5e;
    fn read_next_byte(&mut self, mut cycles: &mut u32, mut mem: [u8; 0xFFFF]) -> u8 {
        let res = self.read_byte(&mut cycles, mem, self.pc);
        self.pc += 1;
        return res;
    }
    fn read_byte(&mut self, cycles: &mut u32, mut mem: [u8; 0xFFFF], addr: u16) -> u8 {
        *cycles += 1;
        return mem[usize::from(addr)];
    }
    fn set_flags(&mut self, val: u8, flags: u8) {
        if (CPU::FLAG_N & flags) == CPU::FLAG_N {
            if val & 0b1000_0000 == 0b1000_0000 {
                self.n = true
            } else {
                self.n = false
            }
        }
        if (CPU::FLAG_Z & flags) == CPU::FLAG_Z {
            if val == 0 {
                self.z = true
            } else {
                self.z = false
            }
        }
    }
    fn read_zero_page(&mut self, mut cycles: &mut u32, mut mem: [u8; 0xFFFF]) -> u8 {
        let addr = self.read_next_byte(&mut cycles, mem);
        return self.read_byte(&mut cycles, mem, addr as u16);
    }
    fn read_zero_page_x(&mut self, mut cycles: &mut u32, mut mem: [u8; 0xFFFF]) -> u8 {
        let mut addr = self.read_next_byte(&mut cycles, mem);
        *cycles += 1;  // read x
        addr = addr.wrapping_add(self.x);
        return self.read_byte(&mut cycles, mem, addr as u16);
    }
    fn read_zero_page_y(&mut self, mut cycles: &mut u32, mut mem: [u8; 0xFFFF]) -> u8 {
        let mut addr = self.read_next_byte(&mut cycles, mem);
        *cycles += 1;  // read y
        addr = addr.wrapping_add(self.y);
        return self.read_byte(&mut cycles, mem, addr as u16);
    }
    fn read_abs(&mut self, mut cycles: &mut u32, mut mem: [u8; 0xFFFF]) -> u8 {
        let lsb = self.read_next_byte(&mut cycles, mem);
        let msb = self.read_next_byte(&mut cycles, mem);
        let addr: u16 = ((msb as u16) << 8) + (lsb as u16);
        return self.read_byte(&mut cycles, mem, addr);
    }
    fn read_abs_x(&mut self, mut cycles: &mut u32, mut mem: [u8; 0xFFFF]) -> u8 {
        let lsb = self.read_next_byte(&mut cycles, mem);
        let msb = self.read_next_byte(&mut cycles, mem);
        let addr: u16 = ((msb as u16) << 8) + (lsb as u16);
        let offset_addr = addr + (self.x as u16);
        if addr >> 8 != offset_addr >> 8 {
            *cycles += 1;
        }
        return self.read_byte(&mut cycles, mem, offset_addr);
    }
    fn read_abs_y(&mut self, mut cycles: &mut u32, mut mem: [u8; 0xFFFF]) -> u8 {
        let lsb = self.read_next_byte(&mut cycles, mem);
        let msb = self.read_next_byte(&mut cycles, mem);
        let addr: u16 = ((msb as u16) << 8) + (lsb as u16);
        let offset_addr = addr + (self.y as u16);
        if addr >> 8 != offset_addr >> 8 {
            *cycles += 1;
        }
        return self.read_byte(&mut cycles, mem, offset_addr);
    }
    pub fn run(&mut self, cycles: u32, mut mem: [u8; 0xFFFF]) -> u32 {
        let mut cpu_cycles = 0;
        while cpu_cycles < cycles {
            let inst = self.read_next_byte(&mut cpu_cycles, mem);
            if inst == CPU::LDA_IM {
                self.a = self.read_next_byte(&mut cpu_cycles, mem);
                self.set_flags(self.a, CPU::FLAG_N | CPU::FLAG_Z);
            } else if inst == CPU::LDA_ZP {
                self.a = self.read_zero_page(&mut cpu_cycles, mem);
                self.set_flags(self.a, CPU::FLAG_N | CPU::FLAG_Z);
            } else if inst == CPU::LDA_ZPX {
                self.a = self.read_zero_page_x(&mut cpu_cycles, mem);
                self.set_flags(self.a, CPU::FLAG_N | CPU::FLAG_Z);
            } else if inst == CPU::LDA_ABS {
                self.a = self.read_abs(&mut cpu_cycles, mem);
                self.set_flags(self.a, CPU::FLAG_N | CPU::FLAG_Z);
            } else if inst == CPU::LDA_ABSX {
                self.a = self.read_abs_x(&mut cpu_cycles, mem);
                self.set_flags(self.a, CPU::FLAG_N | CPU::FLAG_Z);
            } else if inst == CPU::LDA_ABSY {
                self.a = self.read_abs_y(&mut cpu_cycles, mem);
                self.set_flags(self.a, CPU::FLAG_N | CPU::FLAG_Z);
            } else if inst == CPU::LDA_INDX {
                let mut zp_addr = self.read_next_byte(&mut cpu_cycles, mem);
                cpu_cycles += 1;  // read x
                zp_addr = zp_addr.wrapping_add(self.x);
                let lsb = self.read_byte(&mut cpu_cycles, mem, zp_addr as u16);
                let msb = self.read_byte(&mut cpu_cycles, mem, (zp_addr+1) as u16);
                let addr: u16 = ((msb as u16) << 8) + (lsb as u16);
                self.a = self.read_byte(&mut cpu_cycles, mem, addr);
                self.set_flags(self.a, CPU::FLAG_N | CPU::FLAG_Z);
            } else if inst == CPU::LDA_INDY {
                let mut zp_addr = self.read_next_byte(&mut cpu_cycles, mem);
                let lsb = self.read_byte(&mut cpu_cycles, mem, zp_addr as u16);
                let msb = self.read_byte(&mut cpu_cycles, mem, (zp_addr+1) as u16);
                let addr: u16 = ((msb as u16) << 8) + (lsb as u16);
                let offset_addr = addr + (self.y as u16);
                if (addr >> 8) != (offset_addr >> 8) {
                    cpu_cycles += 1;
                }
                self.a = self.read_byte(&mut cpu_cycles, mem, offset_addr);
                self.set_flags(self.a, CPU::FLAG_N | CPU::FLAG_Z);
            } else if inst == CPU::LDX_IM {
                self.x = self.read_next_byte(&mut cpu_cycles, mem);
                self.set_flags(self.x, CPU::FLAG_N | CPU::FLAG_Z);
            } else if inst == CPU::LDX_ZP {
                self.x = self.read_zero_page(&mut cpu_cycles, mem);
                self.set_flags(self.x, CPU::FLAG_N | CPU::FLAG_Z);
            } else if inst == CPU::LDX_ZPY {
                self.x = self.read_zero_page_y(&mut cpu_cycles, mem);
                self.set_flags(self.x, CPU::FLAG_N | CPU::FLAG_Z);
            } else if inst == CPU::LDX_ABS {
                self.x = self.read_abs(&mut cpu_cycles, mem);
                self.set_flags(self.x, CPU::FLAG_N | CPU::FLAG_Z);
            } else if inst == CPU::LDX_ABSY {
                self.x = self.read_abs_y(&mut cpu_cycles, mem);
                self.set_flags(self.x, CPU::FLAG_N | CPU::FLAG_Z);
            } else if inst == CPU::LDY_IM {
                self.y = self.read_next_byte(&mut cpu_cycles, mem);
                self.set_flags(self.y, CPU::FLAG_N | CPU::FLAG_Z);
            } else if inst == CPU::LDY_ZP {
                self.y = self.read_zero_page(&mut cpu_cycles, mem);
                self.set_flags(self.y, CPU::FLAG_N | CPU::FLAG_Z);
            } else if inst == CPU::LDY_ZPX {
                self.y = self.read_zero_page_x(&mut cpu_cycles, mem);
                self.set_flags(self.y, CPU::FLAG_N | CPU::FLAG_Z);
            } else if inst == CPU::LDY_ABS {
                self.y = self.read_abs(&mut cpu_cycles, mem);
                self.set_flags(self.y, CPU::FLAG_N | CPU::FLAG_Z);
            } else if inst == CPU::LDY_ABSX {
                self.y = self.read_abs_x(&mut cpu_cycles, mem);
                self.set_flags(self.y, CPU::FLAG_N | CPU::FLAG_Z);
            } else if inst == CPU::LSR_A {
                cpu_cycles += 1;
                self.c = self.a & 0b0000_0001 == 1;
                self.a = self.a >> 1;
                self.set_flags(self.a, CPU::FLAG_N | CPU::FLAG_Z);
            } else if inst == CPU::LSR_ZP {
            } else if inst == CPU::LSR_ZPX {
            } else if inst == CPU::LSR_ABS {
            } else if inst == CPU::LSR_ABSX {
            } else if inst == CPU::NOP {
                cpu_cycles += 1;
            } else {
                panic!("Unknown instruction")
            }
        }
        return cpu_cycles;
    }
}