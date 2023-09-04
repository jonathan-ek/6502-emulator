use crate::cpu::CPU;

impl CPU {
    pub const SBC_IM: u8 = 0xE9;
    pub const SBC_ZP: u8 = 0xE5;
    pub const SBC_ZPX: u8 = 0xF5;
    pub const SBC_ABS: u8 = 0xED;
    pub const SBC_ABSX: u8 = 0xFD;
    pub const SBC_ABSY: u8 = 0xF9;
    pub const SBC_INDX: u8 = 0xE1;
    pub const SBC_INDY: u8 = 0xF1;

    pub fn run_sbc(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0x10000], inst: u8) -> bool {
        if inst == CPU::SBC_IM {
        } else if inst == CPU::SBC_ZP {
        } else if inst == CPU::SBC_ZPX {
        } else if inst == CPU::SBC_ABS {
        } else if inst == CPU::SBC_ABSX {
        } else if inst == CPU::SBC_ABSY {
        } else if inst == CPU::SBC_INDX {
        } else if inst == CPU::SBC_INDY {
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
