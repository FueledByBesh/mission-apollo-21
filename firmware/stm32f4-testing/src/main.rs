#![no_std]
#![no_main]

use cortex_m_rt::entry;
use rtt_target::{rprintln};
use stm32f4xx_hal::prelude::_fugit_RateExtU32;

mod drivers;
mod app;
mod helper;

use app::App;
use crate::drivers::bmi323::config::Configure;

#[entry]
fn main() -> ! {

    rprintln!("App going to start");
    let mut app = App::setup();
    rprintln!("App started");
    app.bmi323.soft_reset();
    app.delay.delay_ms(1000);
    app.bmi323.default_config();
    app.bmi323.who_am_i();
    loop {
        app.delay.delay_ms(500);
        app.bmi323.read_temp();
        app.bmi323.read_gyr();
    }

}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    rprintln!("{}", info);
    loop {}
}





