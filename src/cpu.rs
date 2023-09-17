use std::sync::mpsc::{Receiver, Sender};

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

pub struct CpuInputPins {
    pub(crate) data: u8,
    pub(crate) irq: bool,  // trigger interupt on low
    pub(crate) nmi: bool,  // trigger interupt on low
    pub(crate) phi2: bool,  // clock
    pub(crate) rdy: bool,  // pauses cpu on low
    pub(crate) res: bool,  // reset CPU, hold low 2 cycles, then 7 cycles before reset complete
    pub(crate) vdd: bool,
}
pub struct CpuOutputPins {
    pub(crate) addr: u16,
    pub(crate) data: u8,
    pub(crate) rwb: bool, // read or write address
    pub(crate) sync: bool, // High during op-code read
}

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
    pub(crate) inp: CpuInputPins,
    pub(crate) out: CpuOutputPins,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            pc: 0xFFFC,
            sp: 255,
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
            inp: CpuInputPins {
                data: 0,
                irq: false,
                nmi: false,
                phi2: false,
                rdy: true,
                res: false,
                vdd: true,
            },
            out: CpuOutputPins {
                addr: 0,
                data: 0,
                rwb: false,
                sync: false,
            },
        }
    }


    pub(crate) const FLAG_C: u8 = 0b00000001;
    pub(crate) const FLAG_Z: u8 = 0b00000010;
    pub(crate) const FLAG_I: u8 = 0b00000100;
    pub(crate) const FLAG_D: u8 = 0b00001000;
    pub(crate) const FLAG_B: u8 = 0b00010000;
    pub(crate) const FLAG_V: u8 = 0b01000000;
    pub(crate) const FLAG_N: u8 = 0b10000000;

    fn read_next_byte(&mut self, wait_for_tick: &dyn Fn(&mut CPU), set_pins: &dyn Fn(&mut CPU)) -> u8 {
        let res = self.read_byte(&wait_for_tick, &set_pins, self.pc);
        self.pc += 1;
        return res;
    }
    fn read_byte(&mut self, wait_for_tick: &dyn Fn(&mut CPU), set_pins: &dyn Fn(&mut CPU), addr: u16) -> u8 {
        self.out.addr = addr;
        self.out.rwb = true;
        set_pins(self);
        wait_for_tick(self);
        return self.inp.data;
    }
    fn write_byte(&mut self, wait_for_tick: &dyn Fn(&mut CPU), set_pins: &dyn Fn(&mut CPU), addr: u16, value: u8) -> u8 {
        self.out.addr = addr;
        self.out.rwb = false;
        self.out.data = value;
        set_pins(self);
        wait_for_tick(self);
        return self.inp.data;
    }
    fn push_to_stack(&mut self, wait_for_tick: &dyn Fn(&mut CPU), set_pins: &dyn Fn(&mut CPU), value: u8) {
        let addr: u16 = ((0x01u16) << 8) + (self.sp as u16);
        self.write_byte(&wait_for_tick, &set_pins, addr, value);
        self.sp = self.sp.wrapping_sub(1);
    }
    fn pop_from_stack(&mut self, wait_for_tick: &dyn Fn(&mut CPU), set_pins: &dyn Fn(&mut CPU)) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        let addr: u16 = ((0x01u16) << 8) + (self.sp as u16);
        return self.read_byte(&wait_for_tick, &set_pins, addr);
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
    fn read_zero_page_addr(&mut self, wait_for_tick: &dyn Fn(&mut CPU), set_pins: &dyn Fn(&mut CPU)) -> u16 {
        let addr = self.read_next_byte(&wait_for_tick, &set_pins);
        return addr as u16;
    }
    fn read_zero_page(&mut self, wait_for_tick: &dyn Fn(&mut CPU), set_pins: &dyn Fn(&mut CPU)) -> u8 {
        let addr = self.read_zero_page_addr(&wait_for_tick, &set_pins);
        return self.read_byte(&wait_for_tick, &set_pins, addr);
    }
    fn read_zero_page_x_addr(&mut self, wait_for_tick: &dyn Fn(&mut CPU), set_pins: &dyn Fn(&mut CPU)) -> u16 {
        let mut addr = self.read_next_byte(&wait_for_tick, &set_pins);
        set_pins(self);
        wait_for_tick(self);
        addr = addr.wrapping_add(self.x);
        return addr as u16;
    }
    fn read_zero_page_x(&mut self, wait_for_tick: &dyn Fn(&mut CPU), set_pins: &dyn Fn(&mut CPU)) -> u8 {
        let addr = self.read_zero_page_x_addr(&wait_for_tick, &set_pins);
        return self.read_byte(&wait_for_tick, &set_pins, addr);
    }
    fn read_zero_page_y_addr(&mut self, wait_for_tick: &dyn Fn(&mut CPU), set_pins: &dyn Fn(&mut CPU)) -> u16 {
        let mut addr = self.read_next_byte(&wait_for_tick, &set_pins);
        set_pins(self);
        wait_for_tick(self);
        addr = addr.wrapping_add(self.y);
        return addr as u16;
    }
    fn read_zero_page_y(&mut self, wait_for_tick: &dyn Fn(&mut CPU), set_pins: &dyn Fn(&mut CPU)) -> u8 {
        let addr = self.read_zero_page_y_addr(&wait_for_tick, &set_pins);
        return self.read_byte(&wait_for_tick, &set_pins, addr);
    }
    fn read_abs_addr(&mut self, wait_for_tick: &dyn Fn(&mut CPU), set_pins: &dyn Fn(&mut CPU)) -> u16 {
        let lsb = self.read_next_byte(&wait_for_tick, &set_pins);
        let msb = self.read_next_byte(&wait_for_tick, &set_pins);
        return ((msb as u16) << 8) + (lsb as u16);
    }
    fn read_abs(&mut self, wait_for_tick: &dyn Fn(&mut CPU), set_pins: &dyn Fn(&mut CPU)) -> u8 {

        let addr = self.read_abs_addr(&wait_for_tick, &set_pins);
        return self.read_byte(&wait_for_tick, &set_pins, addr);
    }
    fn read_abs_x_addr(&mut self, wait_for_tick: &dyn Fn(&mut CPU), set_pins: &dyn Fn(&mut CPU), page_cycle: bool) -> u16 {

        let lsb = self.read_next_byte(&wait_for_tick, &set_pins);
        let msb = self.read_next_byte(&wait_for_tick, &set_pins);
        let addr: u16 = ((msb as u16) << 8) + (lsb as u16);
        let offset_addr = addr + (self.x as u16);
        if page_cycle && addr >> 8 != offset_addr >> 8 {
            set_pins(self);
            wait_for_tick(self);
        }
        return offset_addr;
    }
    fn read_abs_x(&mut self, wait_for_tick: &dyn Fn(&mut CPU), set_pins: &dyn Fn(&mut CPU)) -> u8 {

        let addr = self.read_abs_x_addr(&wait_for_tick, &set_pins, true);
        return self.read_byte(&wait_for_tick, &set_pins, addr);
    }
    fn read_abs_y_addr(&mut self, wait_for_tick: &dyn Fn(&mut CPU), set_pins: &dyn Fn(&mut CPU), page_cycle: bool) -> u16 {

        let lsb = self.read_next_byte(&wait_for_tick, &set_pins);
        let msb = self.read_next_byte(&wait_for_tick, &set_pins);
        let addr: u16 = ((msb as u16) << 8) + (lsb as u16);
        let offset_addr = addr + (self.y as u16);
        if page_cycle && addr >> 8 != offset_addr >> 8 {
            set_pins(self);
            wait_for_tick(self);
        }
        return offset_addr;
    }
    fn read_indexed_indirect_addr(&mut self, wait_for_tick: &dyn Fn(&mut CPU), set_pins: &dyn Fn(&mut CPU)) -> u16 {
        // INDX

        let mut zp_addr = self.read_next_byte(&wait_for_tick, &set_pins);
        set_pins(self);
        wait_for_tick(self);  // read x
        zp_addr = zp_addr.wrapping_add(self.x);
        let lsb = self.read_byte(&wait_for_tick, &set_pins, zp_addr as u16);
        let msb = self.read_byte(&wait_for_tick, &set_pins, (zp_addr + 1) as u16);
        return ((msb as u16) << 8) + (lsb as u16);
    }
    fn read_indirect_indexed_addr(&mut self, wait_for_tick: &dyn Fn(&mut CPU), set_pins: &dyn Fn(&mut CPU), page_cycle: bool) -> u16 {
        // INDY

        let zp_addr = self.read_next_byte(&wait_for_tick, &set_pins);
        let lsb = self.read_byte(&wait_for_tick, &set_pins, zp_addr as u16);
        let msb = self.read_byte(&wait_for_tick, &set_pins, (zp_addr + 1) as u16);
        let addr: u16 = ((msb as u16) << 8) + (lsb as u16);
        let offset_addr = addr + (self.y as u16);
        if page_cycle && (addr >> 8) != (offset_addr >> 8) {
            set_pins(self);
            wait_for_tick(self);
        }
        return offset_addr;
    }
    fn read_abs_y(&mut self, wait_for_tick: &dyn Fn(&mut CPU), set_pins: &dyn Fn(&mut CPU)) -> u8 {

        let addr = self.read_abs_y_addr(&wait_for_tick, &set_pins, true);
        return self.read_byte(&wait_for_tick, &set_pins, addr);
    }
    pub fn run(&mut self, input: Receiver<CpuInputPins>, output: Sender<CpuOutputPins>) {
        let wait_for_tick = |cpu: &mut CPU| {
            let cpu_inp: CpuInputPins = input.recv().unwrap();
            cpu.inp.data =  cpu_inp.data;
            cpu.inp.irq = cpu_inp.irq;
            cpu.inp.nmi = cpu_inp.nmi;
            cpu.inp.phi2 = cpu_inp.phi2;
            cpu.inp.rdy = cpu_inp.rdy;
            cpu.inp.res = cpu_inp.res;
            cpu.inp.vdd =  cpu_inp.vdd;
            return ;
        };
        let set_pins = |cpu: &mut CPU| {
            let out = CpuOutputPins{
                addr: cpu.out.addr,
                data: cpu.out.data,
                rwb: cpu.out.rwb,
                sync: cpu.out.sync,
            };
            output.send(out).unwrap();
            return ;
        };
        loop {
            if !self.inp.vdd {
                return ;
            }
            if !self.inp.res {
                while !self.inp.res {
                    set_pins(self);
                    wait_for_tick(self);
                }
                for i in 0..7 {
                    set_pins(self);
                    wait_for_tick(self);
                }
                let lsb = self.read_byte(&wait_for_tick, &set_pins, 0xFFFC);
                let msb = self.read_byte(&wait_for_tick, &set_pins, 0xFFFD);
                self.pc =  ((msb as u16) << 8) + (lsb as u16);
                self.sp = 255;
                self.a = 0;
                self.x = 0;
                self.y = 0;
                self.c = false;
                self.z = false;
                self.i = false;
                self.d = false;
                self.b = false;
                self.v = false;
                self.n = false;
            }
            self.out.sync = true;
            let inst = self.read_next_byte(&wait_for_tick, &set_pins);
            // println!("{:#06x}: {:#04x}", self.pc, inst);
            self.out.sync = false;
            if self.run_lda(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_ldx(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_ldy(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_lsr(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_ora(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_nop(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_pha(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_php(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_pla(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_plp(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_adc(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_and(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_asl(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_rol(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_bcc(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_bcs(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_beq(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_bit(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_bmi(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_bne(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_bpl(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_brk(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_bvc(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_bvs(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_clc(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_cld(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_cli(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_clv(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_cmp(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_cpx(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_cpy(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_dec(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_dex(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_dey(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_eor(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_inc(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_inx(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_iny(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_jmp(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_jsr(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_ror(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_rti(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_rts(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_sbc(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_sec(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_sed(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_sei(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_sta(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_stx(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_sty(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_tax(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_tay(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_tsx(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_txa(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_txs(&wait_for_tick, &set_pins, inst) {
                continue;
            } else if self.run_tya(&wait_for_tick, &set_pins, inst) {
                continue;
            }  else {
                panic!("Unknown instruction: {:#4x}", inst);
            }
        }
    }
}