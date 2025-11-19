#![no_std]
#![no_main]
extern crate alloc;

use alloc::format;
use alloc::string::String;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal as hal;
use hal::
{
    pac,
    prelude::*,
    rcc::Config,
    timer::{Timer,Timer1}
};
use rtt_target::{rtt_init_print, rprintln};

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

    let gpioc = peripherals.GPIOC.split(&mut clocks);

    let mut led = gpioc.pc13.into_push_pull_output();

    let mut delay = Timer::syst(cp.SYST,&clocks.clocks).delay();

    // let tim1 = Timer1::new(peripherals.TIM1,&mut clocks);

    let final_clocks = &clocks.clocks;
    let (sys,ahb,apb1,apb2) = (final_clocks.sysclk(),final_clocks.hclk(),final_clocks.pclk1(),final_clocks.pclk2());
    rprintln!("System: {}; ahb: {}; apb1:{}; apb2:{};",sys,ahb,apb1,apb2);
    let mut is_off: bool = true;
    loop {

        if is_off {
            led.set_high();
            is_off = false;
            delay.delay_ms(2000u32);
        }else {
            led.set_low();
            is_off = true;
            delay.delay_ms(500u32);
        }

    }
}