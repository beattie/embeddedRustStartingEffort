#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};

use cortex_m_rt::entry;
use microbit::board::Board;
use microbit::hal::{prelude::*, Timer};

#[entry]
fn main() -> ! {
    let mut tick = 0;

    rtt_init_print!();
    rprintln!("Hello World");

    let mut board = Board::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);

    loop {
        tick += 1;
        timer.delay_ms(1_000u32);
    }
}
