use crate::cpu::CPU;

impl CPU {
    pub const JSR: u8 = 0x20;

    pub fn run_jsr(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0x10000], inst: u8) -> bool {
        if inst == CPU::JSR {
            let addr = self.read_abs_addr(&mut cycles, *mem);
            let tmp = self.pc - 1;
            let lsb = (tmp & 0x00ff) as u8;
            let msb = ((tmp & 0xff00) >> 8) as u8;
            self.push_to_stack(&mut cycles, mem, lsb);
            self.push_to_stack(&mut cycles, mem, msb);
            self.pc = addr;
            *cycles += 1;
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
    fn test_nop() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::NOP;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
    }
}