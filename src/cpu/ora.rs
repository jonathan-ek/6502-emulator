use crate::cpu::CPU;

impl CPU {
    pub const ORA_IM: u8 = 0x09;
    pub const ORA_ZP: u8 = 0x05;
    pub const ORA_ZPX: u8 = 0x15;
    pub const ORA_ABS: u8 = 0x0D;
    pub const ORA_ABSX: u8 = 0x1D;
    pub const ORA_ABSY: u8 = 0x19;
    pub const ORA_INDX: u8 = 0x01;
    pub const ORA_INDY: u8 = 0x11;

    pub fn run_ora(&mut self, wait_for_tick: &dyn Fn(&mut CPU), set_pins: &dyn Fn(&mut CPU), inst: u8) -> bool {
        if inst == CPU::ORA_IM {
            let value = self.read_next_byte(wait_for_tick, set_pins);
            self.a = self.a | value;
            self.set_flags(self.a, CPU::FLAG_N | CPU::FLAG_Z);
        } else if inst == CPU::ORA_ZP {
            let addr = self.read_zero_page_addr(wait_for_tick, set_pins);
            let value = self.read_byte(wait_for_tick, set_pins, addr);
            self.a = self.a | value;
            self.set_flags(self.a, CPU::FLAG_N | CPU::FLAG_Z);
        } else if inst == CPU::ORA_ZPX {
            let addr = self.read_zero_page_x_addr(wait_for_tick, set_pins);
            let value = self.read_byte(wait_for_tick, set_pins, addr);
            self.a = self.a | value;
            self.set_flags(self.a, CPU::FLAG_N | CPU::FLAG_Z);
        } else if inst == CPU::ORA_ABS {
            let addr = self.read_abs_addr(wait_for_tick, set_pins);
            let value = self.read_byte(wait_for_tick, set_pins, addr);
            self.a = self.a | value;
            self.set_flags(self.a, CPU::FLAG_N | CPU::FLAG_Z);
        } else if inst == CPU::ORA_ABSX {
            let addr = self.read_abs_x_addr(wait_for_tick, set_pins, true);
            let value = self.read_byte(wait_for_tick, set_pins, addr);
            self.a = self.a | value;
            self.set_flags(self.a, CPU::FLAG_N | CPU::FLAG_Z);
        } else if inst == CPU::ORA_ABSY {
            let addr = self.read_abs_y_addr(wait_for_tick, set_pins, true);
            let value = self.read_byte(wait_for_tick, set_pins, addr);
            self.a = self.a | value;
            self.set_flags(self.a, CPU::FLAG_N | CPU::FLAG_Z);
        } else if inst == CPU::ORA_INDX {
            let addr = self.read_indexed_indirect_addr(wait_for_tick, set_pins);
            let value = self.read_byte(wait_for_tick, set_pins, addr);
            self.a = self.a | value;
            self.set_flags(self.a, CPU::FLAG_N | CPU::FLAG_Z);
        } else if inst == CPU::ORA_INDY {
            let addr = self.read_indirect_indexed_addr(wait_for_tick, set_pins, true);
            let value = self.read_byte(wait_for_tick, set_pins, addr);
            self.a = self.a | value;
            self.set_flags(self.a, CPU::FLAG_N | CPU::FLAG_Z);
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
    fn test_ora_absx() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::ORA_ABSX;
        mem[0xFFFD] = 0x12;
        mem[0xFFFE] = 0x55;
        mem[0x5606] = 0b1010_1101;
        cpu.x = 0xF4;
        cpu.a = 0b1111_0000;
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
        assert_eq!(cpu.a, 0b1111_1101, "a reg");
        assert_eq!(cpu.n, true, "n reg");
    }
}