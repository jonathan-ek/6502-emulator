use crate::cpu::CPU;

impl CPU {
    pub const ADC_IM: u8 = 0x69;
    pub const ADC_ZP: u8 = 0x65;
    pub const ADC_ZPX: u8 = 0x75;
    pub const ADC_ABS: u8 = 0x6D;
    pub const ADC_ABSX: u8 = 0x7D;
    pub const ADC_ABSY: u8 = 0x79;
    pub const ADC_INDX: u8 = 0x61;
    pub const ADC_INDY: u8 = 0x71;

    pub fn run_adc(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0xFFFF], inst: u8) -> bool {
        if inst == CPU::ADC_IM {
        } else if inst == CPU::ADC_ZP {
        } else if inst == CPU::ADC_ZPX {
        } else if inst == CPU::ADC_ABS {
        } else if inst == CPU::ADC_ABSX {
        } else if inst == CPU::ADC_ABSY {
        } else if inst == CPU::ADC_INDX {
        } else if inst == CPU::ADC_INDY {

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
