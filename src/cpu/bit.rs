use crate::cpu::CPU;

impl CPU {
    pub const BIT_ZP: u8 = 0x24;
    pub const BIT_ABS: u8 = 0x2C;

    pub fn run_bit(&mut self, wait_for_tick: &dyn Fn(&mut CPU), set_pins: &dyn Fn(&mut CPU), inst: u8) -> bool {
        if inst == CPU::BIT_ZP {
            let addr = self.read_zero_page_addr(wait_for_tick, set_pins);
            let val = self.read_byte(wait_for_tick, set_pins, addr);
            self.n = (val & 0b1000_0000) > 0;
            self.v = (val & 0b0100_0000) > 0;
            self.z = (val & self.a) == 0;
        } else if inst == CPU::BIT_ABS {
            let addr = self.read_abs_addr(wait_for_tick, set_pins);
            let val = self.read_byte(wait_for_tick, set_pins, addr);
            self.n = (val & 0b1000_0000) > 0;
            self.v = (val & 0b0100_0000) > 0;
            self.z = (val & self.a) == 0;
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
    fn test_bit_1() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::BIT_ZP;
        mem[0xFFFD] = 0x12;
        mem[0x0012] = 0b1010_1010;
        cpu.a = 0b1111_0000;
        let cycles = 3;
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
        assert_eq!(cpu.n, true, "n");
        assert_eq!(cpu.z, false, "z");
        assert_eq!(cpu.v, false, "v");
    }
    #[test]
    fn test_bit_2() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::BIT_ZP;
        mem[0xFFFD] = 0x12;
        mem[0x0012] = 0b1110_1010;
        cpu.a = 0;
        let cycles = 3;
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
        assert_eq!(cpu.n, true, "n");
        assert_eq!(cpu.z, true, "z");
        assert_eq!(cpu.v, true, "v");
    }
    #[test]
    fn test_bit_3() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::BIT_ABS;
        mem[0xFFFD] = 0x12;
        mem[0xFFFE] = 0xAB;
        mem[0xAB12] = 0b0110_1010;
        cpu.a = 0b1010_1010;
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
        assert_eq!(cpu.n, false, "n");
        assert_eq!(cpu.z, false, "z");
        assert_eq!(cpu.v, true, "v");
    }
}