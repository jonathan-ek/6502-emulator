use crate::cpu::CPU;

impl CPU {
    pub const BRK: u8 = 0x00;

    pub fn run_brk(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0x10000], inst: u8) -> bool {
        if inst == CPU::BRK {
            let tmp = self.pc + 1;
            let lsb = (tmp & 0x00ff) as u8;
            let msb = ((tmp & 0xff00) >> 8) as u8;
            self.push_to_stack(&mut cycles, mem, lsb);
            self.push_to_stack(&mut cycles, mem, msb);
            self.b = true;
            self.i = true;
            let mut res: u8 = 0;
            if self.c { res += CPU::FLAG_C; }
            if self.z { res += CPU::FLAG_Z; }
            if self.i { res += CPU::FLAG_I; }
            if self.d { res += CPU::FLAG_D; }
            if self.b { res += CPU::FLAG_B; }
            if self.v { res += CPU::FLAG_V; }
            if self.n { res += CPU::FLAG_N; }
            self.push_to_stack(&mut cycles, mem, res);
            self.b = false;
            let irq_lsb = self.read_byte(&mut cycles, *mem, 0xfffe);
            let irq_msb = self.read_byte(&mut cycles, *mem, 0xffff);
            *cycles -= 2;
            self.pc = ((irq_msb as u16) << 8) + (irq_lsb as u16);
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
    fn test_brk_1() {
        let mut cpu = CPU::new();
        let mut mem: [u8; 0x10000] = [0; 0x10000];
        mem[0xFFFC] = CPU::BRK;
        mem[0xFFFE] = 0x34;
        mem[0xFFFF] = 0x45;
        let cycles = 7;
        assert_eq!(cpu.sp, 0xff);
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
        assert_eq!(mem[0x01FF], 0xFE);
        assert_eq!(mem[0x01FE], 0xFF);
        assert_eq!(mem[0x01FD], CPU::FLAG_I | CPU::FLAG_B);
        assert_eq!(cpu.pc, 0x4534);
        assert_eq!(cpu.i, true);
        assert_eq!(cpu.sp, 0xfc);
    }
}
