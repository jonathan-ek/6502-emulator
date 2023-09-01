use crate::cpu::CPU;

impl CPU {
    pub const AND_IM: u8 = 0x29;
    pub const AND_ZP: u8 = 0x25;
    pub const AND_ZPX: u8 = 0x35;
    pub const AND_ABS: u8 = 0x2D;
    pub const AND_ABSX: u8 = 0x3D;
    pub const AND_ABSY: u8 = 0x39;
    pub const AND_INDX: u8 = 0x21;
    pub const AND_INDY: u8 = 0x31;

    pub fn run_and(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0xFFFF], inst: u8) -> bool {
        if inst == CPU::AND_IM {
        } else if inst == CPU::AND_ZP {
        } else if inst == CPU::AND_ZPX {
        } else if inst == CPU::AND_ABS {
        } else if inst == CPU::AND_ABSX {
        } else if inst == CPU::AND_ABSY {
        } else if inst == CPU::AND_INDX {
        } else if inst == CPU::AND_INDY {
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
