#![no_std]
#![no_main]
mod drivers;

use cortex_m_rt::entry;
use cortex_m::delay::Delay;
use rtt_target::{rtt_init_print, rprintln};
use stm32f1xx_hal::{
    pac,
    // adc::Adc,
    prelude::*,
    // timer::Timer,
    spi::{Spi, Mode},
};
use stm32f1xx_hal::gpio::{IOPinSpeed, OutputSpeed};
use stm32f1xx_hal::rcc::Config;
use drivers::nrf24l01::Nrf24l01;
use crate::drivers::nrf24l01::WriteRead;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let rcc_config = Config::default()
        .use_hse(8.MHz())
        .sysclk(64.MHz());
    rcc = rcc.freeze(rcc_config, &mut flash.acr);

    let mut gpioa = dp.GPIOA.split(&mut rcc);
    let mut delay = Delay::new(cp.SYST,rcc.clocks.sysclk().to_Hz());
    let mut cs = gpioa.pa4.into_push_pull_output(&mut gpioa.crl);
    cs.set_speed(&mut gpioa.crl,IOPinSpeed::Mhz50);
    let mut ce = gpioa.pa3.into_push_pull_output(&mut gpioa.crl);
    ce.set_speed(&mut gpioa.crl,IOPinSpeed::Mhz50);
    let mut clk = gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl);
    let mut miso = gpioa.pa6.into_floating_input(&mut gpioa.crl);
    let mut mosi = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);
    clk.set_speed(&mut gpioa.crl,IOPinSpeed::Mhz50);
    mosi.set_speed(&mut gpioa.crl,IOPinSpeed::Mhz50);

    let spi_pins = (
        Some(clk),
        Some(miso),
        Some(mosi),
    );

    let spi = Spi::new(
        dp.SPI1,
        spi_pins,
        Mode {
            polarity: stm32f1xx_hal::spi::Polarity::IdleLow,
            phase: stm32f1xx_hal::spi::Phase::CaptureOnFirstTransition,
        },
        1.MHz(),
        &mut rcc
    );

    let mut radio = Nrf24l01::new(spi,cs,ce);
    radio.min_setup();


    // let mut adc = Adc::new(dp.ADC1, &mut rcc);
    // adc.set_sample_time(SampleTime::T_71);
    // adc.set_continuous_mode(true);
    let mut delay: DelayWrapper = DelayWrapper{delay};

    loop {
        delay.delay.delay_ms(1000);
        radio.write_payload(b"Hello World!\0");
        radio.send(&mut delay);
    }
}

pub struct DelayWrapper{
    delay: Delay
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    rprintln!("PANIC: {}!", info);
    loop {}
}
