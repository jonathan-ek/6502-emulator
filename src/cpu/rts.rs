use crate::cpu::CPU;

impl CPU {
    pub const RTS: u8 = 0x60;

    pub fn run_rts(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0x10000], inst: u8) -> bool {
        if inst == CPU::RTS {

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