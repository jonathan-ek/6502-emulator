use crate::cpu::CPU;

impl CPU {
    pub const ROL_A: u8 = 0x2A;
    pub const ROL_ZP: u8 = 0x26;
    pub const ROL_ZPX: u8 = 0x36;
    pub const ROL_ABS: u8 = 0x2E;
    pub const ROL_ABSX: u8 = 0x3E;

    fn rol(&mut self, val: u8) -> u8 {
        let bit_0: u16 = if self.c { 1 } else { 0 };
        self.c = val & 0b1000_0000 > 0;
        let res = ((((val as u16) << 1) + bit_0) & 0x00FF) as u8;
        self.set_flags(res, CPU::FLAG_N | CPU::FLAG_Z);
        return res;
    }

    pub fn run_rol(&mut self, wait_for_tick: &dyn Fn(&mut CPU), set_pins: &dyn Fn(&mut CPU), inst: u8) -> bool {
        if inst == CPU::ROL_A {
            self.a = self.rol(self.a);
            set_pins(self);
        wait_for_tick(self);
        } else if inst == CPU::ROL_ZP {
            let addr = self.read_zero_page_addr(wait_for_tick, set_pins);
            let value = self.read_byte(wait_for_tick, set_pins, addr);
            set_pins(self);
        wait_for_tick(self);
            let val = self.rol(value);
            self.write_byte(wait_for_tick, set_pins, addr, val);
        } else if inst == CPU::ROL_ZPX {
            let addr = self.read_zero_page_x_addr(wait_for_tick, set_pins);
            let value = self.read_byte(wait_for_tick, set_pins, addr);
            set_pins(self);
        wait_for_tick(self);
            let val = self.rol(value);
            self.write_byte(wait_for_tick, set_pins, addr, val);
        } else if inst == CPU::ROL_ABS {
            let addr = self.read_abs_addr(wait_for_tick, set_pins);
            let value = self.read_byte(wait_for_tick, set_pins, addr);
            set_pins(self);
        wait_for_tick(self);
            let val = self.rol(value);
            self.write_byte(wait_for_tick, set_pins, addr, val);
        } else if inst == CPU::ROL_ABSX {
            let addr = self.read_abs_x_addr(wait_for_tick, set_pins, false);
            let value = self.read_byte(wait_for_tick, set_pins, addr);
            set_pins(self);
        wait_for_tick(self);
            set_pins(self);
        wait_for_tick(self);
            let val = self.rol(value);
            self.write_byte(wait_for_tick, set_pins, addr, val);
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
    fn test_rol_1() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::ROL_A;
        cpu.c = false;
        cpu.a = 0b1000_0000;
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
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.c, true);
        assert_eq!(cpu.z, true);
        assert_eq!(cpu.n, false);
    }

    #[test]
    fn test_rol_1_b() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::ROL_A;
        cpu.c = true;
        cpu.a = 0b1000_0000;
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
        assert_eq!(cpu.a, 1);
        assert_eq!(cpu.c, true);
        assert_eq!(cpu.z, false);
        assert_eq!(cpu.n, false);
    }

    #[test]
    fn test_rol_2() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::ROL_A;
        cpu.a = 0b1100_0001;
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
        assert_eq!(cpu.a, 0b1000_0010);
        assert_eq!(cpu.c, true);
        assert_eq!(cpu.z, false);
        assert_eq!(cpu.n, true);
    }

    #[test]
    fn test_rol_3() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::ROL_A;
        cpu.a = 0b0100_0001;
        cpu.c = true;
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
        assert_eq!(cpu.a, 0b1000_0011);
        assert_eq!(cpu.c, false);
        assert_eq!(cpu.z, false);
        assert_eq!(cpu.n, true);
    }

    #[test]
    fn test_rol_4() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::ROL_ZP;
        mem[0xFFFD] = 0x01;
        mem[0x0001] = 0b0100_0001;
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
        assert_eq!(mem[0x0001], 0b1000_0010);
        assert_eq!(cpu.c, false, "C");
        assert_eq!(cpu.z, false, "Z");
        assert_eq!(cpu.n, true, "N");
    }

    #[test]
    fn test_rol_zpx() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::ROL_ZPX;
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
        assert_eq!(mem[0x0036], 0b0101_1010, "a reg");
        assert_eq!(cpu.c, true, "c reg");
        assert_eq!(cpu.z, false, "Z");
        assert_eq!(cpu.n, false, "N");
    }

    #[test]
    fn test_rol_abs() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::ROL_ABS;
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
        assert_eq!(mem[0x3412], 0b0000_0010, "a reg");
        assert_eq!(cpu.c, false, "c reg");
    }

    #[test]
    fn test_rol_absx() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::ROL_ABSX;
        mem[0xFFFD] = 0x12;
        mem[0xFFFE] = 0x55;
        mem[0x5536] = 0;
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
        handler.join().unwrap();
        assert_eq!(mem[0x5536], 0, "a reg");
    }
}
