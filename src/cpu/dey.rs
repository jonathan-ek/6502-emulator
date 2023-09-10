use crate::cpu::CPU;

impl CPU {
    pub const DEY: u8 = 0x88;

    pub fn run_dey(&mut self, wait_for_tick: &dyn Fn(&mut CPU), set_pins: &dyn Fn(&mut CPU), inst: u8) -> bool {
        if inst == CPU::DEY {
            self.y = self.y.wrapping_sub(1);
            set_pins(self);
        wait_for_tick(self);
            self.set_flags(self.y, CPU::FLAG_N | CPU::FLAG_Z);
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
    fn test_dey() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::DEY;
        cpu.y = 0x42;
        let cycles = 2;
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
        assert_eq!(cpu.y, 0x41);
    }
    #[test]
    fn test_dey_2() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::DEY;
        cpu.y = 0x01;
        let cycles = 2;
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
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.z, true);
    }
    #[test]
    fn test_dey_3() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::DEY;
        cpu.y = 0x00;
        let cycles = 2;
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
        assert_eq!(cpu.y, 0xFF);
        assert_eq!(cpu.z, false);
        assert_eq!(cpu.n, true);
    }
}