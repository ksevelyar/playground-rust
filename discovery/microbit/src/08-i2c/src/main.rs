//#![deny(unsafe_code)]
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

use microbit::{hal::twim, pac::twim0::frequency::FREQUENCY_A};

use lsm303agr::{AccelOutputDataRate, Lsm303agr, MagOutputDataRate};

mod serial_setup;
use serial_setup::UartePort;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };

    // Code from documentation
    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();
    sensor.set_mag_odr(MagOutputDataRate::Hz50).unwrap();

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

        match byte {
            13 => {
                match core::str::from_utf8(&buffer) {
                    Ok("magnetometer") => {
                        if sensor.mag_status().unwrap().xyz_new_data {
                            let data = sensor.mag_data().unwrap();
                            write!(
                                serial,
                                "magnetometer: x {} y {} z {}\r\n",
                                data.x, data.y, data.z
                            )
                            .unwrap()
                        }
                    }
                    Ok("accelerometer") => {
                        if sensor.accel_status().unwrap().xyz_new_data {
                            let data = sensor.accel_data().unwrap();
                            write!(
                                serial,
                                "accelerometer: x {} y {} z {}\r\n",
                                data.x, data.y, data.z
                            )
                            .unwrap()
                        }
                    }
                    Err(err) => write!(serial, "{}", err).unwrap(),
                    _ => (),
                }
                buffer.clear();
            }
            _ => {
                if buffer.push(byte).is_err() {
                    write!(serial, "error: buffer full\r\n").unwrap();
                    buffer.clear();
                }
            }
        }

        nb::block!(serial.flush()).unwrap()
    }
}
