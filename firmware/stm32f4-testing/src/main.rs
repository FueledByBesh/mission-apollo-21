#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal as hal;
use hal::
{
    pac,
    prelude::*,
    rcc::Config,
    timer::{Timer},
    spi::{Mode}
};
// use stm32f4xx_hal::gpio;
use rtt_target::rtt_init_print;
use stm32f4xx_hal::hal_02::blocking::spi::Transfer;
use stm32f4xx_hal::hal_02::digital::v2::OutputPin;

mod blink_led;
use blink_led::blink;

#[entry]
fn main() -> ! {

    rtt_init_print!();
    let cp = cortex_m::Peripherals::take().unwrap();
    let peripherals = pac::Peripherals::take().unwrap();
    let rcc= peripherals.RCC.constrain();

    let rcc_config:Config = Config::default()
        .use_hse(24.MHz())
        .sysclk(84.MHz());

    let mut clocks = rcc.freeze(rcc_config);

    let gpioa = peripherals.GPIOA.split(&mut clocks);
    let mut led_pin = gpioa.pa5.into_push_pull_output();

    let mut delay = Timer::syst(cp.SYST,&clocks.clocks).delay();
    delay.delay_ms(1000);

    loop {
        blink(&mut led_pin);
        delay.delay_ms(200);
    }
}


