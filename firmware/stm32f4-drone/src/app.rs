use rtt_target::{rprintln};
use stm32f4xx_hal::{
    pac,
    prelude::*,
    rcc::{Rcc,Config},
};
use stm32f4xx_hal::gpio::{Alternate, Output, Pin, PushPull};
use stm32f4xx_hal::pac::{SPI1};
use stm32f4xx_hal::spi::{Mode, Spi};
use cortex_m::Peripherals as CorePeripherals;
use cortex_m::delay::Delay;
use crate::drivers::bmi323::{Bmi323};

pub struct App
{
    pub rcc: Rcc,
    pub bmi323: Bmi323<SPI1,'A',4>,
    pub delay: Delay
}

impl App{

    pub fn setup() -> Self{
        Self::init()
    }

    fn init() -> Self{
        let peripherals = pac::Peripherals::take().unwrap();
        let cp = CorePeripherals::take().unwrap();
        let rcc= peripherals.RCC.constrain();

        let rcc_config:Config = Config::default()
            .use_hse(24.MHz())
            .sysclk(84.MHz());

        let mut rcc = rcc.freeze(rcc_config);

        let syst_delay = Delay::new(cp.SYST,rcc.clocks.sysclk().to_Hz());

        let gpioa = peripherals.GPIOA.split(&mut rcc);

        //bmi323 initialization
        let bmi323 = Self::init_bmi323(
            peripherals.SPI1,
            gpioa.pa5.into_alternate(),
            gpioa.pa6.into_alternate(),
            gpioa.pa7.into_alternate(),
            gpioa.pa4.into_push_pull_output(),
            &mut rcc
        );
        rprintln!("BMI323 initialized");

        Self{
            bmi323,
            rcc,
            delay: syst_delay
        }

    }

    fn init_bmi323(
        spi1: SPI1,
        sck: Pin<'A',5,Alternate<5>>,
        miso: Pin<'A',6,Alternate<5>>,
        mosi: Pin<'A',7,Alternate<5>>,
        cs: Pin<'A',4,Output<PushPull>>,
        rcc: &mut Rcc
    ) -> Bmi323<pac::SPI1,'A',4,> {
        let mode = Mode {
            polarity: stm32f4xx_hal::spi::Polarity::IdleLow,
            phase: stm32f4xx_hal::spi::Phase::CaptureOnFirstTransition
        };

        let spi = Spi::new(
            spi1,
            (Some(sck), Some(miso), Some(mosi)),
            mode,
            8.MHz(),
            rcc
        );

        Bmi323::new(spi, cs)
    }

    // pub fn configure_bmi323(&mut self){
    //     let config = Bmi323Config::default();
    //     self.bmi323.configure(config);
    // }

}