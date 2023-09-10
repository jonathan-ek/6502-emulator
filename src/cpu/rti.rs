use crate::cpu::CPU;

impl CPU {
    pub const RTI: u8 = 0x40;

    pub fn run_rti(&mut self, wait_for_tick: &dyn Fn(&mut CPU), set_pins: &dyn Fn(&mut CPU), inst: u8) -> bool {
        if inst == CPU::RTI {
            let res = self.pop_from_stack(wait_for_tick, set_pins);
            self.c = res & CPU::FLAG_C > 0;
            self.z = res & CPU::FLAG_Z > 0;
            self.i = res & CPU::FLAG_I > 0;
            self.d = res & CPU::FLAG_D > 0;
            self.v = res & CPU::FLAG_V > 0;
            self.n = res & CPU::FLAG_N > 0;
            set_pins(self);
        wait_for_tick(self);
            set_pins(self);
        wait_for_tick(self);
            let lsb = self.pop_from_stack(wait_for_tick, set_pins);
            let msb = self.pop_from_stack(wait_for_tick, set_pins);
            self.pc = ((msb as u16) << 8) + (lsb as u16);
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
    fn test_rti() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::RTI;
        mem[0x01FF] = 0x20;
        mem[0x01FE] = 0x10;
        mem[0x01FD] = CPU::FLAG_C | CPU::FLAG_N;
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
        assert_eq!(cpu.sp, 0xFF);
        assert_eq!(cpu.pc, 0x2010);
        assert_eq!(cpu.c, true);
        assert_eq!(cpu.n, true);
        assert_eq!(cpu.v, false);
        assert_eq!(cpu.b, false);
    }
}