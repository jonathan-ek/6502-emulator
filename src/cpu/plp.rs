use crate::cpu::CPU;

impl CPU {
    pub const PLP: u8 = 0x28;

    pub fn run_plp(&mut self, wait_for_tick: &dyn Fn(&mut CPU), set_pins: &dyn Fn(&mut CPU), inst: u8) -> bool {
        if inst == CPU::PLP {
            let res = self.pop_from_stack(wait_for_tick, set_pins);
            self.c = res & CPU::FLAG_C > 0;
            self.z = res & CPU::FLAG_Z > 0;
            self.i = res & CPU::FLAG_I > 0;
            self.d = res & CPU::FLAG_D > 0;
            // self.b = res & CPU::FLAG_B > 0;
            self.v = res & CPU::FLAG_V > 0;
            self.n = res & CPU::FLAG_N > 0;
            set_pins(self);
        wait_for_tick(self);
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
    fn test_plp() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::PLP;
        mem[0x01FF] = 0b1000_0011;
        cpu.sp = 0xFE;
        let cycles = 4;
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
        assert_eq!(cpu.c, true, "c reg");
        assert_eq!(cpu.n, true, "n reg");
        assert_eq!(cpu.z, true, "z reg");
        assert_eq!(cpu.b, false, "b reg");
        assert_eq!(cpu.d, false, "d reg");
        assert_eq!(cpu.i, false, "i reg");
        assert_eq!(cpu.v, false, "v reg");
        assert_eq!(cpu.sp, 0xFF, "sp reg");
    }
}