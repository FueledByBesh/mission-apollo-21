use crate::drivers::{
    bmi323::Bmi323,
    nrf24l01::{
        Nrf24l01,
    }
};
use cortex_m::{
    delay::Delay,
    peripheral::NVIC
};
use stm32f4xx_hal::{
    gpio::{Alternate, Output, Pin, PushPull, Speed},
    pac,
    pac::{SPI1,SPI2,TIM2},
    prelude::*,
    rcc::{Config, Rcc},
    spi::{Mode, Phase, Polarity, Spi},
    hal::{
        digital::OutputPin
    },
    timer::{Event,Timer}
};

pub struct App {
    pub rcc: Rcc,
    pub delay: Delay,
    pub modules: Modules
}

pub struct Modules{
    pub bmi323: Option<Bmi323<pac::SPI1, 'A', 4>>,
    pub radio: Option<Nrf24l01>
}

impl App {
    pub fn init() -> Self {
        let dp = pac::Peripherals::take().unwrap();
        let cp = cortex_m::Peripherals::take().unwrap();
        let rcc = dp.RCC.constrain();
        let rcc_config: Config = Config::default()
            .use_hse(24.MHz())
            .sysclk(84.MHz());

        let mut rcc = rcc.freeze(rcc_config);

        let syst_delay = Delay::new(cp.SYST, rcc.clocks.sysclk().to_Hz());

        let gpioa = dp.GPIOA.split(&mut rcc);
        let gpiob = dp.GPIOB.split(&mut rcc);

        // let bmi323 = Self::init_bmi323(
        //     dp.SPI1,
        //     gpioa.pa5.into_alternate(),
        //     gpioa.pa6.into_alternate(),
        //     gpioa.pa7.into_alternate(),
        //     gpioa.pa4.into_push_pull_output(),
        //     &mut rcc,
        // );

        // Self::setup_pid_timer(dp.TIM2, &mut rcc);
        let radio = Self::init_nrf24l01(
            dp.SPI2, 
            gpiob.pb13.into_alternate(), 
            gpiob.pb14.into_alternate(), 
            gpiob.pb15.into_alternate(), 
            gpioa.pa11.into_push_pull_output(), 
            gpioa.pa10.into_push_pull_output(),
            &mut rcc
        );
        
        Self {
            rcc,
            delay: syst_delay,
            modules: Modules{
                bmi323: None,
                radio: Some(radio)
            }
        }
    }

    fn init_nrf24l01(
        spi2: SPI2,
        mut sck: Pin<'B', 13, Alternate<5>>,
        mut miso: Pin<'B', 14, Alternate<5>>,
        mut mosi: Pin<'B', 15, Alternate<5>>,
        mut csn: Pin<'A', 11, Output<PushPull>>,
        mut ce: Pin<'A', 10, Output<PushPull>>,
        rcc: &mut Rcc,
    ) -> Nrf24l01{
        let mode = Mode{
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        };
        sck.set_speed(Speed::High);
        miso.set_speed(Speed::High);
        mosi.set_speed(Speed::High);
        ce.set_speed(Speed::High);
        csn.set_speed(Speed::High);

        let spi = Spi::new(spi2, (Some(sck), Some(miso), Some(mosi)), mode, 8.MHz(), rcc);

        Nrf24l01::init(spi, ce, csn)
    }

    fn init_bmi323(
        spi1: SPI1,
        mut sck: Pin<'A', 5, Alternate<5>>,
        mut miso: Pin<'A', 6, Alternate<5>>,
        mut mosi: Pin<'A', 7, Alternate<5>>,
        mut cs: Pin<'A', 4, Output<PushPull>>,
        rcc: &mut Rcc,
    ) -> Bmi323<pac::SPI1, 'A', 4> {
        let mode = Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        };
        sck.set_speed(Speed::High);
        miso.set_speed(Speed::High);
        mosi.set_speed(Speed::High);
        cs.set_speed(Speed::High);

        let spi = Spi::new(
            spi1,
            (Some(sck), Some(miso), Some(mosi)),
            mode,
            8.MHz(),
            rcc,
        );

        Bmi323::new(spi, cs)
    }

    fn setup_pid_timer(tim: TIM2, rcc: &mut Rcc){
        unsafe { NVIC::unmask(pac::interrupt::TIM2); }

        let timer = Timer::new(tim, rcc);
        let mut counter = timer.counter_hz();
        counter.start(1.kHz()).unwrap();
        counter.listen(Event::Update);
    }

    // fn init_motors<TIM:Instance,const A: char, const N: u8>(timer: Timer<TIM>,pin1: Pin<A,N>) {
    //     // let (mut pwm_manager,
    //     //     (
    //     //         ch1,
    //     //         _ch2,
    //     //         _ch3,
    //     //         _ch4
    //     //     )) = Timer::new(dp.TIM3, &mut rcc).pwm_hz(1.kHz());
    //     let (mut pwm_manager,
    //         (
    //             ch1,
    //             _ch2,
    //             _ch3,
    //             _ch4
    //         )) = timer.pwm_hz(1.kHz());
    //     pwm_manager.set_period(50.Hz());
    //     let mut ch1 = ch1.with(pin1.into_alternate());
    //     ch1.enable();
    //     let max_duty = ch1.get_max_duty();
    //     rprintln!("CH1 Max duty: {}", max_duty);
    // }

}
