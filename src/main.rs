use std::fs;
use crate::cpu::{CPU, CpuInputPins, CpuOutputPins};
use std::sync::mpsc;
use std::thread;
// use std::time::Duration;

pub mod cpu;

fn main() {
    let mut cpu = CPU::new();
    // let mut mem: [u8; 0x10000] = [0; 0x10000];
    let file_path = "inputs/program.bin";
    let contents: Vec<u8> = fs::read(file_path)
        .expect("Should have been able to read the file");
    let boxed_slice: Box<[u8]> = contents.into_boxed_slice();
    let m: Box<[u8; 0x10000]> = boxed_slice.try_into().unwrap();
    let mut mem: [u8; 0x10000] = *m;
    let (transmitt_to_cpu, receive_on_cpu) = mpsc::channel();
    let (transmitt_from_cpu, receive_from_cpu) = mpsc::channel();
    thread::spawn(move || {
        let mut clock_state = true;
        let mut half_cycles: u64 = 0;
        let mut data: u8;
        loop {
            let output_pins: CpuOutputPins = receive_from_cpu.recv().unwrap();
            if output_pins.rwb {
                data = mem[usize::from(output_pins.addr)];
            } else {
                data = output_pins.data;
                mem[usize::from(output_pins.addr)] = data;
            }
            transmitt_to_cpu.send(CpuInputPins{
                data: data,
                irq: true,
                nmi: true,
                phi2: clock_state,
                rdy: true,
                res: half_cycles > 4,
                vdd: true,
            }).unwrap();
            clock_state = !clock_state;
            half_cycles += 1;

            // thread::sleep(Duration::from_millis(500));
        }
    });

    cpu.pc = 0x0400;

    cpu.run(receive_on_cpu, transmitt_from_cpu);
}
