use crate::cpu::CPU;

impl CPU {
    pub const INC_ZP: u8 = 0xE6;
    pub const INC_ZPX: u8 = 0xF6;
    pub const INC_ABS: u8 = 0xEE;
    pub const INC_ABSX: u8 = 0xFE;

    fn inc(&mut self, wait_for_tick: &dyn Fn(&mut CPU), set_pins: &dyn Fn(&mut CPU), addr: u16) {
        let mut val = self.read_byte(wait_for_tick, set_pins, addr);
        val = val.wrapping_add(1);
        set_pins(self);
        wait_for_tick(self);
        self.write_byte(wait_for_tick, set_pins, addr, val);
        self.set_flags(val, CPU::FLAG_N | CPU::FLAG_Z);
        return;
    }

    pub fn run_inc(&mut self, wait_for_tick: &dyn Fn(&mut CPU), set_pins: &dyn Fn(&mut CPU), inst: u8) -> bool {
        if inst == CPU::INC_ZP {
            let addr = self.read_zero_page_addr(wait_for_tick, set_pins);
            self.inc(wait_for_tick, set_pins, addr);
        } else if inst == CPU::INC_ZPX {
            let addr = self.read_zero_page_x_addr(wait_for_tick, set_pins);
            self.inc(wait_for_tick, set_pins, addr);
        } else if inst == CPU::INC_ABS {
            let addr = self.read_abs_addr(wait_for_tick, set_pins);
            self.inc(wait_for_tick, set_pins, addr);
        } else if inst == CPU::INC_ABSX {
            let addr = self.read_abs_x_addr(wait_for_tick, set_pins, false);
            set_pins(self);
        wait_for_tick(self);
            self.inc(wait_for_tick, set_pins, addr);
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
    fn test_inc_1() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::INC_ZP;
        mem[0xFFFD] = 0x54;
        mem[0x0054] = 54;
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
        assert_eq!(mem[0x0054], 55);
        assert_eq!(cpu.n, false);
        assert_eq!(cpu.z, false);
    }
    #[test]
    fn test_inc_2() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::INC_ZP;
        mem[0xFFFD] = 0x54;
        mem[0x0054] = 0xff;
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
        assert_eq!(mem[0x0054], 0);
        assert_eq!(cpu.n, false);
        assert_eq!(cpu.z, true);
    }
    #[test]
    fn test_inc_3() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::INC_ZP;
        mem[0xFFFD] = 0x54;
        mem[0x0054] = 0x7F;
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
        assert_eq!(mem[0x0054], 0x80);
        assert_eq!(cpu.n, true);
        assert_eq!(cpu.z, false);
    }
    #[test]
    fn test_inc_4() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::INC_ZPX;
        mem[0xFFFD] = 0x50;
        mem[0x0054] = 8;
        cpu.x = 4;
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
        handler.join().unwrap();
        assert_eq!(mem[0x0054], 9);
    }
    #[test]
    fn test_inc_5() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::INC_ABS;
        mem[0xFFFD] = 0x50;
        mem[0xFFFE] = 0x40;
        mem[0x4050] = 8;
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
        handler.join().unwrap();
        assert_eq!(mem[0x4050], 9);
    }
    #[test]
    fn test_inc_6() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::INC_ABSX;
        mem[0xFFFD] = 0x50;
        mem[0xFFFE] = 0x40;
        mem[0x4054] = 8;
        cpu.x = 4;
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
        assert_eq!(mem[0x4054], 9);
    }
}
