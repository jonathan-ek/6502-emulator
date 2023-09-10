use crate::cpu::CPU;

impl CPU {
    pub const STA_ZP: u8 = 0x85;
    pub const STA_ZPX: u8 = 0x95;
    pub const STA_ABS: u8 = 0x8D;
    pub const STA_ABSX: u8 = 0x9D;
    pub const STA_ABSY: u8 = 0x99;
    pub const STA_INDX: u8 = 0x81;
    pub const STA_INDY: u8 = 0x91;

    pub fn run_sta(&mut self, wait_for_tick: &dyn Fn(&mut CPU), set_pins: &dyn Fn(&mut CPU), inst: u8) -> bool {
        if inst == CPU::STA_ZP {
            let addr = self.read_zero_page_addr(wait_for_tick, set_pins);
            self.write_byte(wait_for_tick, set_pins, addr, self.a);
        } else if inst == CPU::STA_ZPX {
            let addr = self.read_zero_page_x_addr(wait_for_tick, set_pins);
            self.write_byte(wait_for_tick, set_pins, addr, self.a);
        } else if inst == CPU::STA_ABS {
            let addr = self.read_abs_addr(wait_for_tick, set_pins);
            self.write_byte(wait_for_tick, set_pins, addr, self.a);
        } else if inst == CPU::STA_ABSX {
            let addr = self.read_abs_x_addr(wait_for_tick, set_pins, false);
            set_pins(self);
            wait_for_tick(self);
            self.write_byte(wait_for_tick, set_pins, addr, self.a);
        } else if inst == CPU::STA_ABSY {
            let addr = self.read_abs_y_addr(wait_for_tick, set_pins, false);
            set_pins(self);
            wait_for_tick(self);
            self.write_byte(wait_for_tick, set_pins, addr, self.a);
        } else if inst == CPU::STA_INDX {
            let addr = self.read_indexed_indirect_addr(wait_for_tick, set_pins);
            self.write_byte(wait_for_tick, set_pins, addr, self.a);
        } else if inst == CPU::STA_INDY {
            let addr = self.read_indirect_indexed_addr(wait_for_tick, set_pins, false);
            set_pins(self);
            wait_for_tick(self);
            self.write_byte(wait_for_tick, set_pins, addr, self.a);
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
    fn test_sta_zp() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::STA_ZP;
        mem[0xFFFD] = 0x25;
        cpu.a = 62;
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
        handler.join().unwrap();
        assert_eq!(mem[0x0025], 62);
    }

    #[test]
    fn test_sta_zpx() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::STA_ZPX;
        mem[0xFFFD] = 0x25;
        cpu.a = 62;
        cpu.x = 2;
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
        handler.join().unwrap();
        assert_eq!(mem[0x0027], 62);
    }

    #[test]
    fn test_sta_abs() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::STA_ABS;
        mem[0xFFFD] = 0x25;
        mem[0xFFFE] = 0x48;
        cpu.a = 62;
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
        handler.join().unwrap();
        assert_eq!(mem[0x4825], 62);
    }

    #[test]
    fn test_sta_absx() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::STA_ABSX;
        mem[0xFFFD] = 0x25;
        mem[0xFFFE] = 0x48;
        cpu.a = 62;
        cpu.x = 2;
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
        handler.join().unwrap();
        assert_eq!(mem[0x4827], 62);
    }

    #[test]
    fn test_sta_absy() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::STA_ABSY;
        mem[0xFFFD] = 0x25;
        mem[0xFFFE] = 0x48;
        cpu.a = 62;
        cpu.y = 2;
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
        handler.join().unwrap();
        assert_eq!(mem[0x4827], 62);
    }

    #[test]
    fn test_sta_indx() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::STA_INDX;
        mem[0xFFFD] = 0x25;
        mem[0x0027] = 0x29;
        mem[0x0028] = 0x48;
        cpu.a = 62;
        cpu.x = 2;
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
        assert_eq!(mem[0x4829], 62);
    }

    #[test]
    fn test_sta_indy() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::STA_INDY;
        mem[0xFFFD] = 0x25;
        mem[0x0025] = 0x29;
        mem[0x0026] = 0x48;
        cpu.a = 62;
        cpu.y = 2;
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
        assert_eq!(mem[0x482B], 62);
    }
}
