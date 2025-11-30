use cortex_m::prelude::_embedded_hal_blocking_spi_Transfer;
use rtt_target::rprintln;
use stm32f4xx_hal::gpio::{Output, Pin};
use stm32f4xx_hal::hal_02::blocking::spi::Transfer;
use stm32f4xx_hal::pac::SPI1;
use stm32f4xx_hal::spi::Spi;

fn spi_init(gpioa: &mut stm32f4xx_hal::gpio::gpioa::Parts){

    let clk = gpioa.pa5.into_alternate();
    let miso = gpioa.pa6.into_alternate();
    let mosi = gpioa.pa7.into_alternate();
    
    let spi = Spi::

}

fn imu_spi_connection(spi: &mut Spi<SPI1>){

    
}

fn get_whoami<SPI,const P: char,const N:u8>(spi: &mut SPI, cs: &mut Pin<P,N,Output>)
where
    SPI: Transfer<u8>
{
    let reg:u8 = 0b1000_0000;

    let mut tx = [reg, 0x00];
    // // Место для принятия ответов
    // let mut rx = [0u8; 2];

    cs.set_low();
    let buf = spi.transfer(&mut tx).ok().unwrap();
    cs.set_high();

    rprintln!("whoami: {} : {}",buf[0],buf[1]);
}
