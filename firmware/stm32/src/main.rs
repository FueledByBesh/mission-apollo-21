#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f1xx_hal::{
    pac,
    prelude::*,
    dma::{dma1, Event as DmaEvent},
    timer::Timer,
};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Такты
    let mut rcc = dp.RCC.constrain();

    
    let clocks = rcc.configure()
        .use_hse(8.mhz())
        .sysclk(72.mhz())
        .pclk1(36.mhz())
        .freeze(&mut dp.FLASH.constrain().acr);

    // GPIOC
    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    // Подготовим данные для DMA (toggle):
    // PC13 = bit 13
    static mut PATTERN: [u32; 2] = [
        1 << 13, // LED ON
        0 << 13, // LED OFF
    ];

    // Разрешаем DMA
    let mut dma_channels = dp.DMA1.split(&mut rcc.ahb);

    // Настроим таймер TIM2 на 1 Гц
    let mut timer = Timer::tim2(dp.TIM2, &clocks, &mut rcc.apb1)
        .start_count_down(1.hz());
    timer.listen(stm32f1xx_hal::timer::Event::Update);

    // DMA: TIM2_UP → write to GPIOC_ODR
    let dma = dma_channels.2; // канал 2 = TIM2_UP

    unsafe {
        dma.circ_write(
            PATTERN.as_ptr(),
            &dp.GPIOC.odr as *const _ as u32,  // целевой адрес
            DmaEvent::TransferComplete,
        );
    }

    loop {
        // CPU ничего не делает
    }
}
