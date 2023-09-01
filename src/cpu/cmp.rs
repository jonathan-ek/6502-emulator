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

    pub fn run_cmp(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0xFFFF], inst: u8) -> bool {
        if inst == CPU::CMP_IM {
        } else if inst == CPU::CMP_ZP {
        } else if inst == CPU::CMP_ZPX {
        } else if inst == CPU::CMP_ABS {
        } else if inst == CPU::CMP_ABSX {
        } else if inst == CPU::CMP_ABSY {
        } else if inst == CPU::CMP_INDX {
        } else if inst == CPU::CMP_INDY {
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
