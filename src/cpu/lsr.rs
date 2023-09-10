use crate::cpu::CPU;

impl CPU {
    pub const LSR_A: u8 = 0x4A;
    pub const LSR_ZP: u8 = 0x46;
    pub const LSR_ZPX: u8 = 0x56;
    pub const LSR_ABS: u8 = 0x4e;
    pub const LSR_ABSX: u8 = 0x5e;

    pub fn run_lsr(&mut self, wait_for_tick: &dyn Fn(&mut CPU), set_pins: &dyn Fn(&mut CPU), inst: u8) -> bool {
        if inst == CPU::LSR_A {
            set_pins(self);
            wait_for_tick(self);
            self.c = self.a & 0b0000_0001 == 1;
            self.a = self.a >> 1;
            self.set_flags(self.a, CPU::FLAG_N | CPU::FLAG_Z);
        } else if inst == CPU::LSR_ZP {
            let addr = self.read_zero_page_addr(wait_for_tick, set_pins);
            let mut value = self.read_byte(wait_for_tick, set_pins, addr);
            set_pins(self);
            wait_for_tick(self);
            self.c = value & 0b0000_0001 == 1;
            value = value >> 1;
            self.write_byte(wait_for_tick, set_pins, addr, value);
            self.set_flags(value, CPU::FLAG_N | CPU::FLAG_Z);
        } else if inst == CPU::LSR_ZPX {
            let addr = self.read_zero_page_x_addr(wait_for_tick, set_pins);
            let mut value = self.read_byte(wait_for_tick, set_pins, addr);
            set_pins(self);
            wait_for_tick(self);
            self.c = value & 0b0000_0001 == 1;
            value = value >> 1;
            self.write_byte(wait_for_tick, set_pins, addr, value);
            self.set_flags(value, CPU::FLAG_N | CPU::FLAG_Z);
        } else if inst == CPU::LSR_ABS {
            let addr = self.read_abs_addr(wait_for_tick, set_pins);
            let mut value = self.read_byte(wait_for_tick, set_pins, addr);
            set_pins(self);
            wait_for_tick(self);
            self.c = value & 0b0000_0001 == 1;
            value = value >> 1;
            self.write_byte(wait_for_tick, set_pins, addr, value);
            self.set_flags(value, CPU::FLAG_N | CPU::FLAG_Z);
        } else if inst == CPU::LSR_ABSX {
            let addr = self.read_abs_x_addr(wait_for_tick, set_pins, false);
            let mut value = self.read_byte(wait_for_tick, set_pins, addr);
            set_pins(self);
            wait_for_tick(self);
            set_pins(self);
            wait_for_tick(self);
            self.c = value & 0b0000_0001 == 1;
            value = value >> 1;
            self.write_byte(wait_for_tick, set_pins, addr, value);
            self.set_flags(value, CPU::FLAG_N | CPU::FLAG_Z);
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
    fn test_lsr_a() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::LSR_A;
        cpu.a = 0b1010_1100;
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
        assert_eq!(cpu.a, 0b0101_0110, "a reg");
        assert_eq!(cpu.c, false, "c reg");
    }

    #[test]
    fn test_lsr_a_2() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::LSR_A;
        cpu.a = 0b1010_1101;
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
        assert_eq!(cpu.a, 0b0101_0110, "a reg");
        assert_eq!(cpu.c, true, "c reg");
    }

    #[test]
    fn test_lsr_zp() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::LSR_ZP;
        mem[0xFFFD] = 0x12;
        mem[0x0012] = 0b1010_1101;
        let cycles = 5;
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
        assert_eq!(mem[0x0012], 0b0101_0110, "a reg");
        assert_eq!(cpu.c, true, "c reg");
    }

    #[test]
    fn test_lsr_zpx() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::LSR_ZPX;
        mem[0xFFFD] = 0x12;
        mem[0x0036] = 0b1010_1101;
        cpu.x = 0x24;
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
        assert_eq!(mem[0x0036], 0b0101_0110, "a reg");
        assert_eq!(cpu.c, true, "c reg");
    }

    #[test]
    fn test_lsr_abs() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::LSR_ABS;
        mem[0xFFFD] = 0x12;
        mem[0xFFFE] = 0x34;
        mem[0x3412] = 0b0000_0001;
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
        assert_eq!(mem[0x3412], 0, "a reg");
        assert_eq!(cpu.c, true, "c reg");
    }

    #[test]
    fn test_lsr_absx() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::LSR_ABSX;
        mem[0xFFFD] = 0x12;
        mem[0xFFFE] = 0x55;
        mem[0x5536] = 0b1010_1101;
        cpu.x = 0x24;
        let cycles = 7;
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
        assert_eq!(mem[0x5536], 0b0101_0110, "a reg");
        assert_eq!(cpu.c, true, "c reg");
    }
}