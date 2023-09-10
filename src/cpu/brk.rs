use crate::cpu::CPU;

impl CPU {
    pub const BRK: u8 = 0x00;

    pub fn run_brk(&mut self, wait_for_tick: &dyn Fn(&mut CPU), set_pins: &dyn Fn(&mut CPU), inst: u8) -> bool {
        if inst == CPU::BRK {
            let tmp = self.pc + 1;
            let lsb = (tmp & 0x00ff) as u8;
            let msb = ((tmp & 0xff00) >> 8) as u8;
            let mut res: u8 = 0;
            if self.c { res += CPU::FLAG_C; }
            if self.z { res += CPU::FLAG_Z; }
            if self.i { res += CPU::FLAG_I; }
            if self.d { res += CPU::FLAG_D; }
            if self.v { res += CPU::FLAG_V; }
            if self.n { res += CPU::FLAG_N; }

            self.i = true;
            // res += CPU::FLAG_I;
            res += CPU::FLAG_B;
            res += 0b0010_0000;
            self.push_to_stack(wait_for_tick, set_pins, msb);
            self.push_to_stack(wait_for_tick, set_pins, lsb);
            self.push_to_stack(wait_for_tick, set_pins, res);
            set_pins(self);
        wait_for_tick(self);
            let irq_lsb = self.read_byte(wait_for_tick, set_pins, 0xfffe);
            let irq_msb = self.read_byte(wait_for_tick, set_pins, 0xffff);
            self.pc = ((irq_msb as u16) << 8) + (irq_lsb as u16);
        } else {
            return false;
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::thread;
    use crate::cpu::{CPU, CpuInputPins, CpuOutputPins};

    #[test]
    fn test_brk_1() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::BRK;
        mem[0xFFFE] = 0x34;
        mem[0xFFFF] = 0x45;
        let cycles = 7;
        assert_eq!(cpu.sp, 0xff);
        let (transmitt_to_cpu, receive_on_cpu) = mpsc::channel();
        let (transmitt_from_cpu, receive_from_cpu) = mpsc::channel();
        let mut data: u8;

        let handler = thread::spawn(move || {
            cpu.run(receive_on_cpu, transmitt_from_cpu);
            return cpu;
        });
        for i in 0..cycles {
            let output_pins: CpuOutputPins = receive_from_cpu.recv().unwrap();
            if output_pins.rwb {
                data = mem[usize::from(output_pins.addr)];
            } else {
                data = output_pins.data;
                mem[usize::from(output_pins.addr)] = data;
            }
            transmitt_to_cpu.send(CpuInputPins {
                data: data,
                irq: true,
                nmi: true,
                phi2: true,
                rdy: true,
                res: true,
                vdd: i == 0,
            }).unwrap();
        }
        cpu = handler.join().unwrap();
        assert_eq!(mem[0x01FF], 0xFF);
        assert_eq!(mem[0x01FE], 0xFE);
        assert_eq!(mem[0x01FD], CPU::FLAG_B | 0b0010_0000);
        assert_eq!(cpu.pc, 0x4534);
        assert_eq!(cpu.i, true);
        assert_eq!(cpu.sp, 0xfc);
    }
}
