use crate::cpu::CPU;

impl CPU {
    pub const RTI: u8 = 0x40;

    pub fn run_rti(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0x10000], inst: u8) -> bool {
        if inst == CPU::RTI {
            let res = self.pop_from_stack(&mut cycles, mem);
            self.c = res & CPU::FLAG_C > 0;
            self.z = res & CPU::FLAG_Z > 0;
            self.i = res & CPU::FLAG_I > 0;
            self.d = res & CPU::FLAG_D > 0;
            self.v = res & CPU::FLAG_V > 0;
            self.n = res & CPU::FLAG_N > 0;
            let lsb = self.pop_from_stack(&mut cycles, mem);
            let msb = self.pop_from_stack(&mut cycles, mem);
            *cycles -= 1;
            self.pc = ((msb as u16) << 8) + (lsb as u16);
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
    fn test_rti() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::RTI;
        mem[0x01FF] = 0x20;
        mem[0x01FE] = 0x10;
        mem[0x01FD] = CPU::FLAG_C | CPU::FLAG_N;
        cpu.sp = 0xFC;

        let cycles = 6;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(cpu.sp, 0xFF);
        assert_eq!(cpu.pc, 0x2010);
        assert_eq!(cpu.c, true);
        assert_eq!(cpu.n, true);
        assert_eq!(cpu.v, false);
        assert_eq!(cpu.b, false);
    }
}