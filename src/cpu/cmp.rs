use crate::cpu::CPU;

impl CPU {
    pub const CMP_IM: u8 = 0xC9;
    pub const CMP_ZP: u8 = 0xC5;
    pub const CMP_ZPX: u8 = 0xD5;
    pub const CMP_ABS: u8 = 0xCD;
    pub const CMP_ABSX: u8 = 0xDD;
    pub const CMP_ABSY: u8 = 0xD9;
    pub const CMP_INDX: u8 = 0xC1;
    pub const CMP_INDY: u8 = 0xD1;

    fn cmp(&mut self, tmp: u8) {
        let val = self.a.wrapping_sub(tmp);
        self.n = val & 0x80 > 0;
        self.c = self.a >= tmp;
        self.z = self.a == tmp;
    }

    pub fn run_cmp(&mut self, wait_for_tick: &dyn Fn(&mut CPU), set_pins: &dyn Fn(&mut CPU), inst: u8) -> bool {
        if inst == CPU::CMP_IM {
            let tmp = self.read_next_byte(wait_for_tick, set_pins);
            self.cmp(tmp);
        } else if inst == CPU::CMP_ZP {
            let tmp = self.read_zero_page(wait_for_tick, set_pins);
            self.cmp(tmp);
        } else if inst == CPU::CMP_ZPX {
            let tmp = self.read_zero_page_x(wait_for_tick, set_pins);
            self.cmp(tmp);
        } else if inst == CPU::CMP_ABS {
            let tmp = self.read_abs(wait_for_tick, set_pins);
            self.cmp(tmp);
        } else if inst == CPU::CMP_ABSX {
            let tmp = self.read_abs_x(wait_for_tick, set_pins);
            self.cmp(tmp);
        } else if inst == CPU::CMP_ABSY {
            let tmp = self.read_abs_y(wait_for_tick, set_pins);
            self.cmp(tmp);
        } else if inst == CPU::CMP_INDX {
            let addr = self.read_indexed_indirect_addr(wait_for_tick, set_pins);
            let tmp = self.read_byte(wait_for_tick, set_pins, addr);
            self.cmp(tmp);
        } else if inst == CPU::CMP_INDY {
            let addr = self.read_indirect_indexed_addr(wait_for_tick, set_pins, true);
            let tmp = self.read_byte(wait_for_tick, set_pins, addr);
            self.cmp(tmp);
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
    fn test_cmp_1() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::CMP_IM;
        mem[0xFFFD] = 42;
        cpu.a = 42;
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
        assert_eq!(cpu.z, true);
        assert_eq!(cpu.n, false);
        assert_eq!(cpu.c, true);
    }
    #[test]
    fn test_cmp_2() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::CMP_IM;
        mem[0xFFFD] = 12;
        cpu.a = 42;
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
        assert_eq!(cpu.z, false);
        assert_eq!(cpu.n, false);
        assert_eq!(cpu.c, true);
    }
    #[test]
    fn test_cmp_3() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::CMP_IM;
        mem[0xFFFD] = 42;
        cpu.a = 132;
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
        assert_eq!(cpu.z, false);
        assert_eq!(cpu.n, false);
        assert_eq!(cpu.c, true);
    }
    #[test]
    fn test_cmp_4() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::CMP_IM;
        mem[0xFFFD] = 132;
        cpu.a = 42;
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
        assert_eq!(cpu.z, false);
        assert_eq!(cpu.n, true);
        assert_eq!(cpu.c, false);
    }
}
