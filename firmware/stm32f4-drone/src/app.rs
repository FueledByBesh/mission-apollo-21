use rtt_target::{rprintln};
use stm32f4xx_hal::{
    pac,
    prelude::*,
    rcc::{Rcc,Config},
};
use stm32f4xx_hal::gpio::{Alternate, Output, Pin, PushPull};
use stm32f4xx_hal::pac::{SPI1, TIM3};
use stm32f4xx_hal::spi::{Mode, Spi};
use cortex_m::Peripherals as CorePeripherals;
use cortex_m::delay::Delay;
use stm32f4xx_hal::timer::{Timer};
use crate::drivers::bmi323::{Bmi323};
use crate::drivers::motors::{Motor};

pub struct App
{
    pub rcc: Rcc,
    // pub bmi323: Bmi323<SPI1,'A',4>,
    pub delay: Delay,
    pub motors: Motor<0>
}

impl App{

    pub fn setup() -> Self{
        Self::init()
    }

    fn init() -> Self{
        let dp = pac::Peripherals::take().unwrap();
        let cp = CorePeripherals::take().unwrap();
        let rcc= dp.RCC.constrain();

        let rcc_config:Config = Config::default()
            .use_hse(24.MHz())
            .sysclk(84.MHz());

        let mut rcc = rcc.freeze(rcc_config);

        let syst_delay = Delay::new(cp.SYST,rcc.clocks.sysclk().to_Hz());

        // let gpioa = dp.GPIOA.split(&mut rcc);
        let gpiob = dp.GPIOB.split(&mut rcc);

        //bmi323 initialization
        // let bmi323 = Self::init_bmi323(
        //     dp.SPI1,
        //     gpioa.pa5.into_alternate(),
        //     gpioa.pa6.into_alternate(),
        //     gpioa.pa7.into_alternate(),
        //     gpioa.pa4.into_push_pull_output(),
        //     &mut rcc
        // );
        // rprintln!("BMI323 initialized");

        let (mut pwm_manager,
            (
                ch1,
                _ch2,
                _ch3,
                _ch4
            )) = Timer::new(dp.TIM3, &mut rcc).pwm_hz(1.kHz());
        pwm_manager.set_period(50.Hz());
        let mut ch1 = ch1.with(gpiob.pb4.into_alternate());
        ch1.enable();
        let max_duty = ch1.get_max_duty();
        rprintln!("CH1 Max duty: {}", max_duty);

        Self{
            // bmi323,
            rcc,
            delay: syst_delay,
            motors: Motor{channel: ch1}
        }

    }
    
    fn init_nrf24l01(&mut self){
        
        
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