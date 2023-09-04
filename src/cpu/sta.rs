use crate::cpu::CPU;

impl CPU {
    pub const STA_ZP: u8 = 0x85;
    pub const STA_ZPX: u8 = 0x95;
    pub const STA_ABS: u8 = 0x8D;
    pub const STA_ABSX: u8 = 0x9D;
    pub const STA_ABSY: u8 = 0x99;
    pub const STA_INDX: u8 = 0x81;
    pub const STA_INDY: u8 = 0x91;

    pub fn run_sta(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0x10000], inst: u8) -> bool {
        if inst == CPU::STA_ZP {
        } else if inst == CPU::STA_ZPX {
        } else if inst == CPU::STA_ABS {
        } else if inst == CPU::STA_ABSX {
        } else if inst == CPU::STA_ABSY {
        } else if inst == CPU::STA_INDX {
        } else if inst == CPU::STA_INDY {
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
