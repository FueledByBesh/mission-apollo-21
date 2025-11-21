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
    timer::{Timer,Timer2},
    spi::{Mode}
};
use rtt_target::{rtt_init_print, rprintln};
use stm32f4xx_hal::spi::{Phase, Polarity, Spi};

mod drivers;
use drivers::imu::bmi323::{Bmi323};
use crate::drivers::imu::Imu;

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
    //SPI pins
    let sck = gpioa.pa5.into_alternate();
    let miso = gpioa.pa6.into_alternate();
    let mosi = gpioa.pa7.into_alternate();

    //Chip select pin
    let mut cs = gpioa.pa4.into_push_pull_output();
    cs.set_high();

    let spi_mode = Mode{
        polarity: Polarity::IdleLow,
        phase: Phase::CaptureOnFirstTransition
    };

    let mut spi = Spi::new(
        peripherals.SPI1,
        (Some(sck),Some(miso),Some(mosi)),
        spi_mode,
        1.MHz(),
        &mut clocks
    );

    // let delay = Timer::syst(cp.SYST,&clocks.clocks).delay();

    let mut imu = Bmi323::new(&mut spi,&mut cs);

    let whoami = imu.read_register(0x72);

    rprintln!("whoami: {}",whoami);


    loop {
        cortex_m::asm::nop();
    }
}