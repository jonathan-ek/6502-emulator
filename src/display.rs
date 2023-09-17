use std::mem::discriminant;
use std::sync::mpsc::{Receiver, Sender};

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use crate::cgrom::{CGROM};
pub struct DisplayInputPins {
    pub(crate) data: Option<u8>,
    pub(crate) rs: Option<bool>,
    pub(crate) rwb: Option<bool>,
    pub(crate) e: Option<bool>,
}

pub struct DisplayOutputPins {
    pub(crate) data: u8,
}

pub struct Display {
    pub(crate) data: u8,
    pub(crate) rs: bool,
    pub(crate) rwb: bool,
    pub(crate) e: bool,
    pub(crate) address_counter: u8,

    pub(crate) cgram: [u8; 64],
    pub(crate) drram: [u8; 128],

    pub(crate) i_d: bool,
    // true = increment, false = decrement
    pub(crate) shift: bool,
    // Shift display
    pub(crate) display: bool,
    // on/off
    pub(crate) cursor: bool,
    pub(crate) blink: bool,
    pub(crate) s_c: bool,
    pub(crate) r_l: bool,
    pub(crate) cg_dd: bool, // cg_ram (True) or ddram (False)
}


impl Display {
    pub fn new() -> Display {
        Display {
            data: 0,
            rs: false,
            rwb: false,
            e: false,
            address_counter: 0,
            cgram: [0; 64],
            drram: [0b00100000; 128],
            i_d: true,
            shift: false,
            display: false,
            cursor: false,
            blink: false,
            s_c: false,
            r_l: false,
            cg_dd: false,
        }
    }
    pub fn run(&mut self, input: Receiver<DisplayInputPins>, output: Sender<DisplayOutputPins>) {
        let wait_for_tick = |disp: &mut Display| {
            let mut prev_e = disp.e;
            while !(prev_e == false && disp.e == true) {
                prev_e = disp.e;
                let disp_inp: DisplayInputPins = input.recv().unwrap();
                disp.data = disp_inp.data.unwrap_or(disp.data);
                disp.rs = disp_inp.rs.unwrap_or(disp.rs);
                disp.rwb = disp_inp.rwb.unwrap_or(disp.rwb);
                disp.e = disp_inp.e.unwrap_or(disp.e);
            }
            return;
        };
        let set_pins = |data: u8| {
            let out = DisplayOutputPins {
                data: data,
            };
            output.send(out).unwrap();
            return;
        };
        let sdl_context = sdl2::init().expect("");
        let video_subsystem = sdl_context.video().expect("");
        let pixel_size = 4;
        let side_border = 6;
        let top_bottom_border = 2;
        let char_space = 1;
        let dot_space = 1;
        let window_width = (16 * (5 + char_space) + 2 * side_border - char_space) * (pixel_size + dot_space);
        let window_height = ((2 * (8 + char_space)) + (top_bottom_border * 2)) * (pixel_size + dot_space);
        let window = video_subsystem
            .window(
                "Display",
                window_width,
                window_height,
            )
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string()).expect("");

        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string()).expect("");
        canvas.set_draw_color(Color::RGB(0x2b, 0x4b, 0xe5));
        canvas.clear();
        canvas.present();
        loop {
            wait_for_tick(self);
            // #2b4be5
            canvas.set_draw_color(Color::RGB(0x2b, 0x4b, 0xe5));
            canvas.clear();
            canvas.set_draw_color(Color::RGB(0x21, 0x21, 0x23));

            if !self.rs {
                // Instructions
                if self.rwb == false && self.data & 0b1111_1111 == 1 {
                    // Clear display
                    // Clears entire display and sets DDRAM address 0 in address counter.
                    self.drram = [0b00100000; 128];
                    self.address_counter = 0;
                    self.cg_dd = false;
                } else if self.rwb == false && self.data & 0b1111_1110 == 0b0000_0010 {
                    // Return home
                    // Sets DDRAM address 0 in address counter.
                    // Also returns display from being shifted to original position.
                    // DDRAM contents remain unchanged.
                    self.address_counter = 0;
                } else if self.rwb == false && self.data & 0b1111_1100 == 0b0000_0100 {
                    // Entry mode set
                    // Sets cursor move direction and specifies display shift.
                    // These operations are performed during data write and read.
                    // TODO: Not implemented
                    self.i_d = self.data & 0b0000_0010 == 0b0000_0010;
                    self.shift = self.data & 0b0000_0001 == 0b0000_0001;
                } else if self.rwb == false && self.data & 0b1111_1000 == 0b0000_1000 {
                    // Display on/off control
                    // Sets entire display (D) on/off,
                    // cursor on/off (C), and blinking of cursor position character (B).
                    self.display = self.data & 0b0000_0100 == 0b0000_0100;
                    self.cursor = self.data & 0b0000_0010 == 0b0000_0010;
                    // TODO: Blink is not implemented
                    self.blink = self.data & 0b0000_0001 == 0b0000_0001;
                } else if self.rwb == false && self.data & 0b1111_0000 == 0b0001_0000 {
                    // Cursor or display shift
                    // Moves cursor and shifts display without changing DDRAM contents.
                    // TODO: Not implemented
                    self.s_c = self.data & 0b0000_1000 == 0b0000_1000;
                    self.r_l = self.data & 0b0000_0100 == 0b0000_0100;
                } else if self.rwb == false && self.data & 0b1110_0000 == 0b0010_0000 {
                    // Function set
                    // Sets interface data length (DL), number of display lines (N), and character font (F).
                    let d_l = self.data & 0b0001_0000 == 0b0001_0000;
                    let n = self.data & 0b0000_1000 == 0b0000_1000;
                    let f = self.data & 0b0000_0100 == 0b0000_0100;
                    if !(d_l && n && !f) {
                        panic!("Not implemented")
                    }
                } else if self.rwb == false && self.data & 0b1100_0000 == 0b0100_0000 {
                    // Set CGRAM address
                    // CGRAM data is sent and received after this setting.
                    self.address_counter = self.data & 0b0011_1111;
                    self.cg_dd = true;
                } else if self.rwb == false && self.data & 0b1000_0000 == 0b1000_0000 {
                    // Set DDRAM address
                    // DDRAM data is sent and received after this setting.
                    self.address_counter = self.data & 0b0111_1111;
                    self.cg_dd = false;
                } else if self.rwb == true {
                    // Read busy flag & address
                    // Reads busy flag (BF) indicating internal operation
                    // is being performed and reads address counter contents.
                    // TODO: Not really correctly implemented.
                    set_pins(self.address_counter & 0b0111_1111)
                }
            } else {
                if self.rwb == false {
                    // Writes data into DDRAM or CGRAM.
                    if self.cg_dd {
                        // CGRAM
                        self.cgram[self.address_counter as usize] = self.data;
                        self.address_counter += 1;
                    } else {
                        // DDRAM
                        self.drram[self.address_counter as usize] = self.data;
                        self.address_counter += 1;
                    }
                } else {
                    // Reads data from DDRAM or CGRAM.
                    // TODO: Not implemented, but cant get the physical display to read correct.
                }
            }
            // Update display
            for row in 0..2 {
                for char in 0..16 {
                    let base_addr:u8 = if row == 1 { 0b01000000 } else { 0 };
                    let cg_addr = self.drram[(base_addr | char) as usize];
                    for c_y in 0..8 {
                        let addr: usize = (((cg_addr as u16) * 8) + c_y) as usize;
                        let c_row = if cg_addr <= 0x0F { self.cgram[addr] } else { CGROM[addr] };
                        for c_x in 0..5 {
                            // println!("{:#010b}", row);
                            let x = (side_border + (char as u32 * (5 + char_space) ) + c_x) * (pixel_size + dot_space);
                            let y = (top_bottom_border + (row * (8 + char_space)) + c_y as u32) * (pixel_size + dot_space);
                            if self.display && ((self.cursor && c_y == 7 && self.address_counter == (base_addr | char) && !self.cg_dd) ||
                               (c_x == 0 && 0b00010000 & c_row > 0) ||
                               (c_x == 1 && 0b00001000 & c_row > 0) ||
                               (c_x == 2 && 0b00000100 & c_row > 0) ||
                               (c_x == 3 && 0b00000010 & c_row > 0) ||
                               (c_x == 4 && 0b00000001 & c_row > 0)) {
                                canvas.set_draw_color(Color::RGB(0x21, 0x21, 0x23));
                                canvas.fill_rect(Rect::new(x as i32, y as i32, pixel_size, pixel_size)).expect("");
                            } else {
                                // #2640b9
                                canvas.set_draw_color(Color::RGB(0x26, 0x46, 0xe0));
                                // canvas.set_draw_color(Color::RGB(0x26, 0x40, 0xb9));
                                canvas.fill_rect(Rect::new(x as i32, y as i32, pixel_size, pixel_size)).expect("");
                            }
                        }
                    }
                }
            }
            canvas.present();
        }
    }
}