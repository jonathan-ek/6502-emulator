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

    pub fn run_cpy(&mut self, wait_for_tick: &dyn Fn(&mut CPU), set_pins: &dyn Fn(&mut CPU), inst: u8) -> bool {
        if inst == CPU::CPY_IM {
            let tmp = self.read_next_byte(wait_for_tick, set_pins);
            self.cpy(tmp);
        } else if inst == CPU::CPY_ZP {
            let tmp = self.read_zero_page(wait_for_tick, set_pins);
            self.cpy(tmp);
        } else if inst == CPU::CPY_ABS {
            let tmp = self.read_abs(wait_for_tick, set_pins);
            self.cpy(tmp);
        } else {
            return false;
        }
        return true;
    }
}