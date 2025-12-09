#![no_std]
#![no_main]

mod drivers;

use cortex_m_rt::entry;
use cortex_m::delay::Delay;
use rtt_target::{rtt_init_print, rprintln};
use stm32f1xx_hal::{
    pac,
    adc::Adc,
    prelude::*,
    // timer::Timer,
    // spi::{Spi, Mode},
};
use stm32f1xx_hal::adc::{SampleTime};
use stm32f1xx_hal::rcc::Config;

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
    // let mut led = gpioa.pa4.into_push_pull_output(&mut gpioa.crl);
    let mut delay = Delay::new(cp.SYST,rcc.clocks.sysclk().to_Hz());

    // let spi_pins = (
    //     Some(gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl)),
    //     Some(gpioa.pa6.into_floating_input(&mut gpioa.crl)),
    //     Some(gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl)),
    // );
    //
    // let spi = Spi::new(
    //     dp.SPI1,
    //     spi_pins,
    //     Mode {
    //         polarity: stm32f1xx_hal::spi::Polarity::IdleLow,
    //         phase: stm32f1xx_hal::spi::Phase::CaptureOnFirstTransition,
    //     },
    //     1.MHz(),
    //     &mut rcc
    // );

    let mut analog_pin = gpioa.pa1.into_analog(&mut gpioa.crl);
    // let mut adc = dp.ADC1.adc(&mut rcc);
    let mut adc = Adc::new(dp.ADC1, &mut rcc);
    adc.set_sample_time(SampleTime::T_71);
    adc.set_continuous_mode(true);


    loop {
        delay.delay_ms(1000);
        // led.toggle();
        let result:u16 = adc.read(&mut analog_pin).unwrap();
        let vref = adc.read_vref();
        rprintln!("ADC: {}, Vref: {}", result,vref);
    }
}


#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    rprintln!("PANIC: {}!", info);
    loop {}
}
