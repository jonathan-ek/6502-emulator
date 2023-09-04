use crate::cpu::CPU;

impl CPU {
    pub const RTI: u8 = 0x40;

    pub fn run_rti(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0x10000], inst: u8) -> bool {
        if inst == CPU::RTI {

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