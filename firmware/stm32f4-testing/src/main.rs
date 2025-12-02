#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::hal::delay::DelayNs;
use stm32f4xx_hal::prelude::_fugit_RateExtU32;

mod drivers;
mod app;
use app::App;

#[entry]
fn main() -> ! {

    rtt_init_print!();
    rprintln!("App going to start");

    let mut app = App::setup();

    rprintln!("App started");
    app.delay.delay_ms(2000);
    app.bmi323.who_am_i();
    loop {
        app.delay.delay_ms(2000);
        rprintln!("accel_config");
        app.bmi323.read_accel_config();
    }

}





