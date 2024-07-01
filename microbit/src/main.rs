#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, Timer},
};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut matrix = [
        [1, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];
    let mut state = [0, 0];

    loop {
        let [x, y] = state;
        matrix[y as usize][x as usize] = 0;

        let [x, y] = iterate(&mut state);
        matrix[*y as usize][*x as usize] = 1;

        display.show(&mut timer, matrix, 100);

        display.clear();
        timer.delay_ms(100_u32);
    }
}

fn iterate(state: &mut [u8; 2]) -> &[u8; 2] {
    match state {
        [0, y] if *y > 0 => {
            state[1] = *y - 1;
            state
        }
        [x, 4] => {
            state[0] = *x - 1;
            state
        }
        [4, y] => {
            state[1] = *y + 1;
            state
        }
        [x, 0] => {
            state[0] = *x + 1;
            state
        }
        _ => unreachable!(),
    }
}
