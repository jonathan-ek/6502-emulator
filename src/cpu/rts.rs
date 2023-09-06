use crate::cpu::CPU;

impl CPU {
    pub const RTS: u8 = 0x60;

    pub fn run_rts(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0x10000], inst: u8) -> bool {
        if inst == CPU::RTS {
            let msb = self.pop_from_stack(&mut cycles, mem);
            let lsb = self.pop_from_stack(&mut cycles, mem);
            *cycles += 1;
            self.pc = ((msb as u16) << 8) + (lsb as u16) + 1;
        } else {
            return false;
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::CPU;

    #[test]
    fn test_rts() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::RTS;
        mem[0x01FE] = 0x10;
        mem[0x01FD] = 0x20;
        cpu.sp = 0xFC;

        let cycles = 6;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.sp, 0xFE);
        assert_eq!(cpu.pc, 0x2011);
    }
    #[test]
    fn test_rts_2() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        cpu.pc = 0xA0FC;
        mem[0xA0FC] = CPU::JSR;
        mem[0xA0FD] = 0x10;
        mem[0xA0FE] = 0x20;
        mem[0x2010] = CPU::RTS;

        let cycles = 12;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.sp, 255);
        assert_eq!(cpu.pc, 0xA0FF);
    }
}