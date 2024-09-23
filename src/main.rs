#![no_std]
#![no_main]


use cortex_m::asm::nop;
use cortex_m_rt::entry;
use nrf52832_hal::{self as hal, Temp};
use hal::pac;

use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};


#[entry]
fn main() -> ! {
    _init_rtt();

    let p: pac::Peripherals = pac::Peripherals::take().unwrap();
    let mut temp: Temp = hal::temp::Temp::new(p.TEMP);
    
    loop{
        let deg_c: f32 = temp.measure().to_num();
        rprintln!("Temp: {}", deg_c);
        for _ in 0..400_000 {
            nop();
        }
    }
}

fn _init_rtt() {
    rtt_init_print!();
}
