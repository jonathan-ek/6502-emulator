use crate::cpu::CPU;

impl CPU {
    pub const RTS: u8 = 0x60;

    pub fn run_rts(&mut self, wait_for_tick: &dyn Fn(&mut CPU), set_pins: &dyn Fn(&mut CPU), inst: u8) -> bool {
        if inst == CPU::RTS {
            let lsb = self.pop_from_stack(wait_for_tick, set_pins);
            let msb = self.pop_from_stack(wait_for_tick, set_pins);
            set_pins(self);
        wait_for_tick(self);
            set_pins(self);
        wait_for_tick(self);
            set_pins(self);
        wait_for_tick(self);
            self.pc = ((msb as u16) << 8) + (lsb as u16) + 1;
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
    fn test_rts() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::RTS;
        mem[0x01FE] = 0x20;
        mem[0x01FD] = 0x10;
        cpu.sp = 0xFC;

        let cycles = 6;
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
        assert_eq!(cpu.sp, 0xFE);
        assert_eq!(cpu.pc, 0x2011);
    }
    #[test]
    fn test_rts_2() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        cpu.pc = 0xA0FC;
        mem[0xA0FC] = CPU::JSR;
        mem[0xA0FD] = 0x10;
        mem[0xA0FE] = 0x20;
        mem[0x2010] = CPU::RTS;

        let cycles = 12;
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
                vdd: i < 8,
            }).unwrap();
        }
        cpu = handler.join().unwrap();
        assert_eq!(cpu.sp, 255);
        assert_eq!(cpu.pc, 0xA0FF);
    }
}