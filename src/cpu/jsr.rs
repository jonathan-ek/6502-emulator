use crate::cpu::CPU;

impl CPU {
    pub const JSR: u8 = 0x20;

    pub fn run_jsr(&mut self, wait_for_tick: &dyn Fn(&mut CPU), set_pins: &dyn Fn(&mut CPU), inst: u8) -> bool {
        if inst == CPU::JSR {
            let addr = self.read_abs_addr(wait_for_tick, set_pins);
            let tmp = self.pc - 1;
            let lsb = (tmp & 0x00ff) as u8;
            let msb = ((tmp & 0xff00) >> 8) as u8;
            self.push_to_stack(wait_for_tick, set_pins, msb);
            self.push_to_stack(wait_for_tick, set_pins, lsb);
            self.pc = addr;
            set_pins(self);
        wait_for_tick(self);
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
    fn test_jsr() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::JSR;
        mem[0xFFFD] = 0x00;
        mem[0xFFFE] = 0x60;
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
        assert_eq!(cpu.pc, 0x6000);
        assert_eq!(cpu.sp, 0xFD);
        assert_eq!(mem[0x01FF], 0xFF);
        assert_eq!(mem[0x01FE], 0xFE);
    }
}