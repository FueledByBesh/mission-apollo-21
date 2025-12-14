#![no_std]
#![no_main]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print,rprintln};
use stm32f4xx_hal::prelude::_fugit_RateExtU32;

mod drivers;
mod app;
mod helper;

use app::App;
use crate::drivers::bmi323::config::Configure;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("App going to start");
    let mut app = App::setup();
    rprintln!("App started");
    app.delay.delay_ms(4000);
    let duty:u8 = 7;
    rprintln!("Duty: {}%",duty);
    app.motors.set_duty(duty);
    rprintln!("Max Duty: {}",app.motors.channel.get_max_duty());
    rprintln!("Current Duty: {}", app.motors.channel.get_duty());
    loop {
       cortex_m::asm::nop();
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    rprintln!("PANIC: {}!", info);
    loop {}
}





