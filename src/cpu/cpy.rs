use crate::cpu::CPU;

impl CPU {
    pub const CPY_IM: u8 = 0xC0;
    pub const CPY_ZP: u8 = 0xC4;
    pub const CPY_ABS: u8 = 0xCC;

    fn cpy(&mut self, tmp: u8) {
        let val = self.y.wrapping_sub(tmp);
        self.n = val & 0x80 > 0;
        self.c = self.y >= tmp;
        self.z = self.y == tmp;
    }

    pub fn run_cpy(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0x10000], inst: u8) -> bool {
        if inst == CPU::CPY_IM {
            let tmp = self.read_next_byte(&mut cycles, *mem);
            self.cpy(tmp);
        } else if inst == CPU::CPY_ZP {
            let tmp = self.read_zero_page(&mut cycles, *mem);
            self.cpy(tmp);
        } else if inst == CPU::CPY_ABS {
            let tmp = self.read_abs(&mut cycles, *mem);
            self.cpy(tmp);
        } else {
            return false;
        }
        return true;
    }
}