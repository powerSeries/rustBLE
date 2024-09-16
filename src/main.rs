#![no_std]
#![no_main]


use cortex_m::asm::nop;
use cortex_m_rt::entry;
use embedded_hal::digital::{OutputPin, PinState};
use nrf52832_hal::{self as hal, gpio::Level};
use hal::pac;

use panic_halt as _;


#[entry]
fn main() -> ! {
    let p: pac::Peripherals = pac::Peripherals::take().unwrap();
    let port0 = hal::gpio::p0::Parts::new(p.P0);
    let mut row1  = port0.p0_17.into_push_pull_output(Level::Low);
    let mut is_on: bool = false;
    loop{
        let _ = row1.set_state(PinState::from(is_on));
        for _ in 0..400_000 {
            nop();
        }
        is_on = !is_on;
    }
}
