use crate::cpu::CPU;

impl CPU {
    pub const AND_IM: u8 = 0x29;
    pub const AND_ZP: u8 = 0x25;
    pub const AND_ZPX: u8 = 0x35;
    pub const AND_ABS: u8 = 0x2D;
    pub const AND_ABSX: u8 = 0x3D;
    pub const AND_ABSY: u8 = 0x39;
    pub const AND_INDX: u8 = 0x21;
    pub const AND_INDY: u8 = 0x31;

    fn and(&mut self, val: u8) {
        self.a = self.a & val;
        self.set_flags(self.a, CPU::FLAG_N | CPU::FLAG_Z);
    }
    pub fn run_and(&mut self, wait_for_tick: &dyn Fn(&mut CPU), set_pins: &dyn Fn(&mut CPU), inst: u8) -> bool {
        if inst == CPU::AND_IM {
            let val = self.read_next_byte(wait_for_tick, set_pins);
            self.and(val);
        } else if inst == CPU::AND_ZP {
            let addr = self.read_zero_page_addr(wait_for_tick, set_pins);
            let val = self.read_byte(wait_for_tick, set_pins, addr);
            self.and(val);
        } else if inst == CPU::AND_ZPX {
            let addr = self.read_zero_page_x_addr(wait_for_tick, set_pins);
            let val = self.read_byte(wait_for_tick, set_pins, addr);
            self.and(val);
        } else if inst == CPU::AND_ABS {
            let addr = self.read_abs_addr(wait_for_tick, set_pins);
            let val = self.read_byte(wait_for_tick, set_pins, addr);
            self.and(val);
        } else if inst == CPU::AND_ABSX {
            let addr = self.read_abs_x_addr(wait_for_tick, set_pins, true);
            let val = self.read_byte(wait_for_tick, set_pins, addr);
            self.and(val);
        } else if inst == CPU::AND_ABSY {
            let addr = self.read_abs_y_addr(wait_for_tick, set_pins, true);
            let val = self.read_byte(wait_for_tick, set_pins, addr);
            self.and(val);
        } else if inst == CPU::AND_INDX {
            let addr = self.read_indexed_indirect_addr(wait_for_tick, set_pins);
            let val = self.read_byte(wait_for_tick, set_pins, addr);
            self.and(val);
        } else if inst == CPU::AND_INDY {
            let addr = self.read_indirect_indexed_addr(wait_for_tick, set_pins, true);
            let val = self.read_byte(wait_for_tick, set_pins, addr);
            self.and(val);
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
    fn test_and_1() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::AND_IM;
        mem[0xFFFD] = 0b1010_0101;
        cpu.a = 0b0011_0011;
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
        assert_eq!(cpu.a, 0b0010_0001);
        assert_eq!(cpu.n, false);
        assert_eq!(cpu.z, false);
    }
    #[test]
    fn test_and_2() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::AND_IM;
        mem[0xFFFD] = 0b1010_0101;
        cpu.a = 0b1100_0011;
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
        assert_eq!(cpu.a, 0b1000_0001);
        assert_eq!(cpu.n, true);
        assert_eq!(cpu.z, false);
    }
    #[test]
    fn test_and_3() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::AND_IM;
        mem[0xFFFD] = 0x00;
        cpu.a = 0b1100_0011;
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
        assert_eq!(cpu.n, false);
        assert_eq!(cpu.z, true);
    }
}
