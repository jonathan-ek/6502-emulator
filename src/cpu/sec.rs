use crate::cpu::CPU;

impl CPU {
    pub const SEC: u8 = 0x38;

    pub fn run_sec(&mut self, wait_for_tick: &dyn Fn(&mut CPU), set_pins: &dyn Fn(&mut CPU), inst: u8) -> bool {
        if inst == CPU::SEC {
            self.c = true;
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
    fn test_sec() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::SEC;
        cpu.c = false;
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
        assert_eq!(cpu.c, true);
    }
}