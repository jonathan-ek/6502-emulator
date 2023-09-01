use crate::cpu::CPU;

impl CPU {
    pub const EOR_IM: u8 = 0x49;
    pub const EOR_ZP: u8 = 0x45;
    pub const EOR_ZPX: u8 = 0x55;
    pub const EOR_ABS: u8 = 0x4D;
    pub const EOR_ABSX: u8 = 0x5D;
    pub const EOR_ABSY: u8 = 0x59;
    pub const EOR_INDX: u8 = 0x41;
    pub const EOR_INDY: u8 = 0x51;

    pub fn run_eor(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0xFFFF], inst: u8) -> bool {
        if inst == CPU::EOR_IM {
        } else if inst == CPU::EOR_ZP {
        } else if inst == CPU::EOR_ZPX {
        } else if inst == CPU::EOR_ABS {
        } else if inst == CPU::EOR_ABSX {
        } else if inst == CPU::EOR_ABSY {
        } else if inst == CPU::EOR_INDX {
        } else if inst == CPU::EOR_INDY {
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
        let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];
        mem[0xFFFC] = CPU::NOP;
        let cycles = 2;
        assert_eq!(cpu.run(cycles, &mut mem), cycles);
    }
}
