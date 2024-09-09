#![no_main]
#![no_std]

use core::fmt::Write;
use cortex_m_rt::entry;
use heapless::Vec;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use microbit::{
    hal::prelude::*,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
};

mod serial_setup;
use serial_setup::UartePort;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };
    let mut buffer: Vec<u8, 32> = Vec::new();

    loop {
        let byte = nb::block!(serial.read()).unwrap();

        let result = match byte {
            13 => {
                for byte in buffer.iter().rev() {
                    nb::block!(serial.write(*byte));
                }
                buffer.clear();
                nb::block!(serial.flush());
                Ok(())
            }
            _ => buffer.push(byte),
        };

        if let Err(err) = result {
            buffer.clear();
            write!(serial, "Error 🐗 {}", err).unwrap()
        }
    }
}
