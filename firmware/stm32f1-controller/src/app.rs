use crate::drivers::{joystick::Joysticks, nrf24l01::Nrf24l01};
use cortex_m::delay::Delay;
use stm32f1xx_hal::gpio::{Input,PullUp};
use stm32f1xx_hal::pac::SPI1;
use stm32f1xx_hal::{
    gpio::{Alternate, Analog, Floating, GpioExt, Output, Pin, PushPull},
    pac::{self, ADC1},
    prelude::*,
    rcc::{Config, Rcc},
    spi::{Mode, Spi},
};

pub struct App{
    pub rcc: Rcc,
    pub delay: Delay,
    pub modules: Modules,
}

pub struct Modules {
    pub joysticks: Joysticks,
    pub radio: Nrf24l01,
    pub led: Pin<'C', 13, Output<PushPull>>,
}

impl App{
    pub fn init() -> Self {
        let cp = cortex_m::Peripherals::take().unwrap();
        let dp = pac::Peripherals::take().unwrap();

        let mut flash = dp.FLASH.constrain();
        let mut rcc = dp.RCC.constrain();
        let rcc_config = Config::default().use_hse(8.MHz()).sysclk(64.MHz());

        rcc = rcc.freeze(rcc_config, &mut flash.acr);

        let delay = Delay::new(cp.SYST, rcc.clocks.hclk().raw());

        let mut gpioa = dp.GPIOA.split(&mut rcc);
        let mut gpiob = dp.GPIOB.split(&mut rcc);
        let mut gpioc = dp.GPIOC.split(&mut rcc);

        let led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

        let left_x_pin = gpiob.pb0.into_analog(&mut gpiob.crl);
        let left_y_pin = gpiob.pb1.into_analog(&mut gpiob.crl);
        let left_sw = gpiob.pb10.into_pull_up_input(&mut gpiob.crh);
        let right_x_pin = gpioa.pa0.into_analog(&mut gpioa.crl);
        let right_y_pin = gpioa.pa1.into_analog(&mut gpioa.crl);
        let right_sw = gpioc.pc15.into_pull_up_input(&mut gpioc.crh);

        let (ce, cs, sck, miso, mosi) = (
            gpioa.pa10.into_push_pull_output(&mut gpioa.crh),
            gpioa.pa11.into_push_pull_output(&mut gpioa.crh),
            gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl),
            gpioa.pa6.into_floating_input(&mut gpioa.crl),
            gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl),
        );

        let modules = Modules {
            joysticks: Self::init_joystick(
                dp.ADC1,
                &mut rcc,
                left_x_pin,
                left_y_pin,
                left_sw,
                right_x_pin,
                right_y_pin,
                right_sw,
            ),
            radio: Self::init_nrf24l01(dp.SPI1, &mut rcc, ce, cs, sck, miso, mosi),
            led
        };

        Self {
            rcc,
            delay,
            modules,
        }
    }

    fn init_joystick(
        adc: ADC1,
        rcc: &mut Rcc,
        left_x_pin: Pin<'B', 0, Analog>,
        left_y_pin: Pin<'B', 1, Analog>,
        left_sw: Pin<'B', 10, Input<PullUp>>,
        right_x_pin: Pin<'A', 0, Analog>,
        right_y_pin: Pin<'A', 1, Analog>,
        right_sw: Pin<'C', 15, Input<PullUp>>,
    ) -> Joysticks {
        Joysticks::init(adc, left_x_pin, left_y_pin,left_sw, right_x_pin, right_y_pin,right_sw, rcc)
    }

    fn init_nrf24l01(
        spi1: SPI1,
        rcc: &mut Rcc,
        ce: Pin<'A', 10, Output<PushPull>>,
        cs: Pin<'A', 11, Output<PushPull>>,
        clk: Pin<'A', 5, Alternate<PushPull>>,
        miso: Pin<'A', 6, Input<Floating>>,
        mosi: Pin<'A', 7, Alternate<PushPull>>,
    ) -> Nrf24l01 {
        let spi = Spi::new(
            spi1,
            (Some(clk), Some(miso), Some(mosi)),
            Mode {
                polarity: stm32f1xx_hal::spi::Polarity::IdleLow,
                phase: stm32f1xx_hal::spi::Phase::CaptureOnFirstTransition,
            },
            8.MHz(),
            rcc,
        );

        Nrf24l01::new(spi, ce, cs)
    }
}
