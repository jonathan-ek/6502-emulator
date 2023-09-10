use crate::cpu::CPU;

impl CPU {
    pub const PHP: u8 = 0x08;

    pub fn run_php(&mut self, wait_for_tick: &dyn Fn(&mut CPU), set_pins: &dyn Fn(&mut CPU), inst: u8) -> bool {
        if inst == CPU::PHP {
            let mut res: u8 = 0;
            if self.c { res += CPU::FLAG_C; }
            if self.z { res += CPU::FLAG_Z; }
            if self.i { res += CPU::FLAG_I; }
            if self.d { res += CPU::FLAG_D; }
            res += CPU::FLAG_B;
            if self.v { res += CPU::FLAG_V; }
            if self.n { res += CPU::FLAG_N; }
            res += 0b0010_0000;
            self.push_to_stack(wait_for_tick, set_pins, res);
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
    fn test_php() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::PHP;
        cpu.c = true;
        cpu.n = true;
        cpu.z = true;
        let cycles = 3;
        assert_eq!(cpu.sp, 0xFF, "sp reg");
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
        assert_eq!(mem[0x01FF], 0b1011_0011, "a reg");
        assert_eq!(cpu.sp, 0xFE, "sp reg");
    }
}