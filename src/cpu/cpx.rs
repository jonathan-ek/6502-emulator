use crate::cpu::CPU;

impl CPU {
    pub const CPX_IM: u8 = 0xE0;
    pub const CPX_ZP: u8 = 0xE4;
    pub const CPX_ABS: u8 = 0xEC;

    fn cpx(&mut self, tmp: u8) {
        let val = self.x.wrapping_sub(tmp);
        self.n = val & 0x80 > 0;
        self.c = self.x >= tmp;
        self.z = self.x == tmp;
    }

    pub fn run_cpx(&mut self, mut cycles: &mut u32, mem: &mut [u8; 0x10000], inst: u8) -> bool {
        if inst == CPU::CPX_IM {
            let tmp = self.read_next_byte(&mut cycles, *mem);
            self.cpx(tmp);
        } else if inst == CPU::CPX_ZP {
            let tmp = self.read_zero_page(&mut cycles, *mem);
            self.cpx(tmp);
        } else if inst == CPU::CPX_ABS {
            let tmp = self.read_abs(&mut cycles, *mem);
            self.cpx(tmp);
        } else {
            return false;
        }
        return true;
    }
}
