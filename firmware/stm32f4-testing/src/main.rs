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
    spi::{Spi,Mode},
    gpio::{Speed}
};
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::hal::digital::OutputPin;
use stm32f4xx_hal::hal::spi::SpiBus;

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

    let mut clk = gpioa.pa5.into_alternate();
    let mut miso = gpioa.pa6.into_alternate();
    let mut mosi = gpioa.pa7.into_alternate();
    clk.set_speed(Speed::Medium);
    miso.set_speed(Speed::Medium);
    mosi.set_speed(Speed::Medium);

    let mut chip_select = gpioa.pa4.into_push_pull_output();
    chip_select.set_high();
    chip_select.set_speed(Speed::Medium);

    let mode = Mode{
        polarity: hal::spi::Polarity::IdleLow,
        phase: hal::spi::Phase::CaptureOnFirstTransition,
    };


    let mut spi = Spi::new(
        peripherals.SPI1,
        (Some(clk),Some(miso),Some(mosi)),
        mode,
        100.kHz(),
        &mut clocks
    );

    let mut delay = Timer::syst(cp.SYST,&clocks.clocks).delay();
    delay.delay_ms(1000);


    let tx = [0b1000_0000,0];

    loop {
        let mut rx = [0u8; 4];
        delay.delay_ms(1000);
        chip_select.set_low();
        spi.transfer(&mut rx, &tx).unwrap();
        chip_select.set_high();
        rprintln!("{:?}",rx);
        delay.delay_ms(3000);
    }
}


