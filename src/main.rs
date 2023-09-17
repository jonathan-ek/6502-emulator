use std::{fs, io};
use std::io::Write;
use crate::cpu::{CPU, CpuInputPins, CpuOutputPins};
use crate::display::{Display, DisplayInputPins, DisplayOutputPins};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub mod cpu;
mod display;
mod cgrom;


fn main() {
    let mut cpu = CPU::new();
    let mut display = Display::new();
    // let mut mem: [u8; 0x10000] = [0; 0x10000];
    let file_path = "inputs/display.out";
    let contents: Vec<u8> = fs::read(file_path)
        .expect("Should have been able to read the file");
    let boxed_slice: Box<[u8]> = contents.into_boxed_slice();
    let rom_box: Box<[u8; 0x8000]> = boxed_slice.try_into().unwrap();
    let rom: [u8; 0x8000] = *rom_box;
    // println!("{:?}", rom);
    let mut ram: [u8; 0x8000] = [0; 0x8000];
    let (transmitt_to_cpu, receive_on_cpu) = mpsc::channel();
    let (transmitt_from_cpu, receive_from_cpu) = mpsc::channel();
    let (transmitt_to_disp, receive_on_disp) = mpsc::channel();
    let (transmitt_from_disp, receive_from_disp) = mpsc::channel();
    thread::spawn(move || {
        display.run(receive_on_disp, transmitt_from_disp);
    });
    let mut port_a_direction = 0;
    let mut port_b_direction = 0;
    thread::spawn(move || {
        let mut cycles: u64 = 0;
        let mut data: u8;
        loop {
            let output_pins: CpuOutputPins = receive_from_cpu.recv().unwrap();
            if output_pins.rwb {
                // Read
                if output_pins.addr & 0b1000_0000_0000_0000 > 0 {
                    data = rom[usize::from(output_pins.addr - 0x8000)];
                } else if output_pins.addr & 0b0100_0000_0000_0000 > 0 &&
                    output_pins.addr & 0b0010_0000_0000_0000 > 0 &&
                    output_pins.addr & 0b1000_0000_0000_0000 == 0 {
                    // bus
                    data = 0;
                } else {
                    data = ram[usize::from(output_pins.addr)];
                }
            } else {
                // Write
                data = output_pins.data;
                if output_pins.addr & 0b1000_0000_0000_0000 > 0 {
                    // ROM
                } else if output_pins.addr & 0b0100_0000_0000_0000 > 0 &&
                    output_pins.addr & 0b0010_0000_0000_0000 > 0 &&
                    output_pins.addr & 0b1000_0000_0000_0000 == 0 {
                    // VIA
                    // PORTB = $6000
                    // PORTA = $6001
                    // DDRB = $6002
                    // DDRA = $6003
                    if output_pins.addr & 0x000F == 0 {
                        // PORTB
                        if port_b_direction == 0b1111_1111 {
                            transmitt_to_disp.send(DisplayInputPins {
                                data: Option::from(data),
                                rs: None,
                                rwb: None,
                                e: None,
                            }).unwrap();
                        } else {
                            let output_pins: DisplayOutputPins = receive_from_disp.recv().unwrap();
                            data = output_pins.data;
                        }
                    } else if output_pins.addr & 0x000F == 1 {
                        // PORTA
                        // E  = %10000000
                        // RW = %01000000
                        // RS = %00100000
                        transmitt_to_disp.send(DisplayInputPins {
                            data: None,
                            rs: Option::from(data & 0b0010_0000 > 0),
                            rwb: Option::from(data & 0b0100_0000 > 0),
                            e: Option::from(data & 0b1000_0000 > 0),
                        }).unwrap();
                    } else if output_pins.addr & 0x000F == 2 {
                        // DDRB
                        // Set data direction port B
                        port_b_direction = data
                    } else if output_pins.addr & 0x000F == 3 {
                        // DDRA
                        // Set data direction port A
                        port_a_direction = data
                    }
                    // blink
                    //print!("{:#010b}\r", data);
                    //io::stdout().flush().unwrap();
                } else {
                    ram[usize::from(output_pins.addr)] = data;
                }
            }
            transmitt_to_cpu.send(CpuInputPins {
                data: data,
                irq: true,
                nmi: true,
                phi2: true,
                rdy: true,
                res: cycles > 4,
                vdd: true,
            }).unwrap();
            cycles += 1;

            // thread::sleep(Duration::from_millis(10));
        }
    });

    cpu.run(receive_on_cpu, transmitt_from_cpu);
}
