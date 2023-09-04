mod lda;
mod ldx;
mod ldy;
mod lsr;
mod ora;
mod nop;
mod pha;
mod php;
mod pla;
mod plp;
mod adc;
mod and;
mod rol;
mod asl;
mod bcc;
mod bcs;
mod beq;
mod bit;
mod bmi;
mod bne;
mod brk;
mod bpl;
mod bvc;
mod bvs;
mod cld;
mod clc;
mod cli;
mod clv;
mod cmp;
mod cpx;
mod cpy;
mod dec;
mod dex;
mod dey;
mod eor;
mod inc;
mod inx;
mod iny;
mod jmp;
mod ror;
mod jsr;
mod rti;
mod sbc;
mod rts;
mod sec;
mod sed;
mod sta;
mod stx;
mod sty;
mod sei;
mod tax;
mod tay;
mod tsx;
mod txa;
mod txs;
mod tya;

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


    pub(crate) const FLAG_C: u8 = 0b00000001;
    pub(crate) const FLAG_Z: u8 = 0b00000010;
    pub(crate) const FLAG_I: u8 = 0b00000100;
    pub(crate) const FLAG_D: u8 = 0b00001000;
    pub(crate) const FLAG_B: u8 = 0b00010000;
    pub(crate) const FLAG_V: u8 = 0b01000000;
    pub(crate) const FLAG_N: u8 = 0b10000000;

    fn read_next_byte(&mut self, mut cycles: &mut u32, mem: [u8; 0x10000]) -> u8 {
        let res = self.read_byte(&mut cycles, mem, self.pc);
        self.pc += 1;
        return res;
    }
    fn read_byte(&mut self, cycles: &mut u32, mem: [u8; 0x10000], addr: u16) -> u8 {
        *cycles += 1;
        return mem[usize::from(addr)];
    }
    fn write_byte(&mut self, cycles: &mut u32, mem: &mut [u8; 0x10000], addr: u16, value: u8) -> u8 {
        *cycles += 1;
        (*mem)[usize::from(addr)] = value;
        return mem[usize::from(addr)];
    }
    fn push_to_stack(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0x10000], value: u8) {
        let addr: u16 = ((0x01u16) << 8) + (self.sp as u16);
        self.write_byte(&mut cycles, mem, addr, value);
        *cycles += 1;
        self.sp += 1;
    }
    fn pop_from_stack(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0x10000]) -> u8 {
        *cycles += 1;
        self.sp -= 1;
        let addr: u16 = ((0x01u16) << 8) + (self.sp as u16);
        return self.read_byte(&mut cycles, *mem, addr);
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
    fn read_zero_page_addr(&mut self, mut cycles: &mut u32, mem: [u8; 0x10000]) -> u16 {
        let addr = self.read_next_byte(&mut cycles, mem);
        return addr as u16;
    }
    fn read_zero_page(&mut self, mut cycles: &mut u32, mem: [u8; 0x10000]) -> u8 {
        let addr = self.read_zero_page_addr(&mut cycles, mem);
        return self.read_byte(&mut cycles, mem, addr);
    }
    fn read_zero_page_x_addr(&mut self, mut cycles: &mut u32, mem: [u8; 0x10000]) -> u16 {
        let mut addr = self.read_next_byte(&mut cycles, mem);
        *cycles += 1;  // read x
        addr = addr.wrapping_add(self.x);
        return addr as u16;
    }
    fn read_zero_page_x(&mut self, mut cycles: &mut u32, mem: [u8; 0x10000]) -> u8 {
        let addr = self.read_zero_page_x_addr(&mut cycles, mem);
        return self.read_byte(&mut cycles, mem, addr);
    }
    fn read_zero_page_y_addr(&mut self, mut cycles: &mut u32, mem: [u8; 0x10000]) -> u16 {
        let mut addr = self.read_next_byte(&mut cycles, mem);
        *cycles += 1;  // read y
        addr = addr.wrapping_add(self.y);
        return addr as u16;
    }
    fn read_zero_page_y(&mut self, mut cycles: &mut u32, mem: [u8; 0x10000]) -> u8 {
        let addr = self.read_zero_page_y_addr(&mut cycles, mem);
        return self.read_byte(&mut cycles, mem, addr);
    }
    fn read_abs_addr(&mut self, mut cycles: &mut u32, mem: [u8; 0x10000]) -> u16 {
        let lsb = self.read_next_byte(&mut cycles, mem);
        let msb = self.read_next_byte(&mut cycles, mem);
        return ((msb as u16) << 8) + (lsb as u16);
    }
    fn read_abs(&mut self, mut cycles: &mut u32, mem: [u8; 0x10000]) -> u8 {
        let addr = self.read_abs_addr(&mut cycles, mem);
        return self.read_byte(&mut cycles, mem, addr);
    }
    fn read_abs_x_addr(&mut self, mut cycles: &mut u32, mem: [u8; 0x10000], page_cycle: bool) -> u16 {
        let lsb = self.read_next_byte(&mut cycles, mem);
        let msb = self.read_next_byte(&mut cycles, mem);
        let addr: u16 = ((msb as u16) << 8) + (lsb as u16);
        let offset_addr = addr + (self.x as u16);
        if page_cycle && addr >> 8 != offset_addr >> 8 {
            *cycles += 1;
        }
        return offset_addr;
    }
    fn read_abs_x(&mut self, mut cycles: &mut u32, mem: [u8; 0x10000]) -> u8 {
        let addr = self.read_abs_x_addr(&mut cycles, mem, true);
        return self.read_byte(&mut cycles, mem, addr);
    }
    fn read_abs_y_addr(&mut self, mut cycles: &mut u32, mem: [u8; 0x10000], page_cycle: bool) -> u16 {
        let lsb = self.read_next_byte(&mut cycles, mem);
        let msb = self.read_next_byte(&mut cycles, mem);
        let addr: u16 = ((msb as u16) << 8) + (lsb as u16);
        let offset_addr = addr + (self.y as u16);
        if page_cycle && addr >> 8 != offset_addr >> 8 {
            *cycles += 1;
        }
        return offset_addr;
    }
    fn read_indexed_indirect_addr(&mut self, mut cycles: &mut u32, mem: [u8; 0x10000]) -> u16 {
        // INDX
        let mut zp_addr = self.read_next_byte(&mut cycles, mem);
        *cycles += 1;  // read x
        zp_addr = zp_addr.wrapping_add(self.x);
        let lsb = self.read_byte(&mut cycles, mem, zp_addr as u16);
        let msb = self.read_byte(&mut cycles, mem, (zp_addr + 1) as u16);
        return ((msb as u16) << 8) + (lsb as u16);
    }
    fn read_indirect_indexed_addr(&mut self, mut cycles: &mut u32, mem: [u8; 0x10000], page_cycle: bool) -> u16 {
        // INDY
        let zp_addr = self.read_next_byte(&mut cycles, mem);
        let lsb = self.read_byte(&mut cycles, mem, zp_addr as u16);
        let msb = self.read_byte(&mut cycles, mem, (zp_addr + 1) as u16);
        let addr: u16 = ((msb as u16) << 8) + (lsb as u16);
        let offset_addr = addr + (self.y as u16);
        if page_cycle && (addr >> 8) != (offset_addr >> 8) {
            *cycles += 1;
        }
        return offset_addr;
    }
    fn read_abs_y(&mut self, mut cycles: &mut u32, mem: [u8; 0x10000]) -> u8 {
        let addr = self.read_abs_y_addr(&mut cycles, mem, true);
        return self.read_byte(&mut cycles, mem, addr);
    }
    pub fn run(&mut self, cycles: u32, mem: &mut [u8; 0x10000]) -> u32 {
        let mut cpu_cycles = 0;
        while cpu_cycles < cycles {
            let inst = self.read_next_byte(&mut cpu_cycles, *mem);
            if self.run_lda(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_ldx(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_ldy(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_lsr(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_ora(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_nop(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_pha(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_php(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_pla(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_plp(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_adc(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_and(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_asl(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_rol(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_bcc(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_bcs(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_beq(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_bit(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_bmi(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_bne(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_bpl(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_brk(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_bvc(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_bvs(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_clc(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_cld(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_cli(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_clv(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_cmp(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_cpx(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_cpy(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_dec(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_dex(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_dey(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_eor(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_inc(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_inx(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_iny(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_jmp(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_jsr(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_ror(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_rti(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_rts(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_sbc(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_sec(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_sed(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_sei(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_sta(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_stx(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_sty(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_tax(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_tay(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_tsx(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_txa(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_txs(&mut cpu_cycles, mem, inst) {
                continue;
            } else if self.run_tya(&mut cpu_cycles, mem, inst) {
                continue;
            }  else {
                panic!("Unknown instruction")
            }
        }
        return cpu_cycles;
    }
}