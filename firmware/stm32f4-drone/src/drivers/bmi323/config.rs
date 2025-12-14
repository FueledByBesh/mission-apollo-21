use rtt_target::rprintln;
use stm32f4xx_hal::pac;
use crate::drivers::bmi323::{Bmi323, WriteRead16bit};
use super::constants::*;

#[derive(Clone,Copy)]
pub struct AccelConfig{
    pub mode: Mode,
    pub odr: ODR,
    pub avg_num: AvgSampleNum,
    pub range: AccRange,
    pub bw: BW
}
#[derive(Clone,Copy)]
pub struct GyroConfig{
    pub mode: Mode,
    pub odr: ODR,
    pub avg_num: AvgSampleNum,
    pub range: GyroRange,
    pub bw: BW
}
#[derive(Clone,Copy)]
pub struct RangeScales{
    pub accel_lsb_per_g: f32,
    pub gyro_lsb_per_dps: f32,
}
#[derive(Clone,Copy)]
pub struct Bmi323Config{
    pub gyro_config: GyroConfig,
    pub accel_config: AccelConfig,
    pub range_scales: RangeScales
}

impl Default for Bmi323Config{
    fn default() -> Self {

        let accel_config = AccelConfig{
            mode: Mode::Normal,
            odr: ODR::Odr1600hz,
            avg_num: AvgSampleNum::ThirtyTwoSamples,
            range: AccRange::FourG,
            bw: BW::Low
        };

        let gyro_config = GyroConfig{
            mode: Mode::Normal,
            odr: ODR::Odr1600hz,
            avg_num: AvgSampleNum::ThirtyTwoSamples,
            range: GyroRange::A1000,
            bw: BW::Low
        };

        let range_scales = RangeScales{
            accel_lsb_per_g: accel_config.range.get_lsb_per_g_value(),
            gyro_lsb_per_dps: gyro_config.range.get_lsb_per_dps_value()
        };

        Bmi323Config {
            gyro_config,
            accel_config,
            range_scales
        }
    }
}

pub trait Configure{
    fn default_config(&mut self);
    fn configure(&mut self, config: Bmi323Config);
    fn _set_accel_config(&mut self, conf: &AccelConfig);
    fn _set_gyro_config(&mut self, conf: &GyroConfig);
}

impl Configure for Bmi323<pac::SPI1, 'A', 4> {
    fn default_config(&mut self){
        self.configure(Bmi323Config::default());
        self.config = Some(Bmi323Config::default());
    }
    fn configure(&mut self, config: Bmi323Config){
        // self.soft_reset();
        self._set_accel_config(&config.accel_config);
        self._set_gyro_config(&config.gyro_config);
        rprintln!("Configuration done");
        self.config = Some(config);
    }

    fn _set_accel_config(&mut self,conf: &AccelConfig){
        let lsb_byte = (conf.bw.to_u8()<<7) | (conf.range.to_u8()<<4) | (conf.odr.to_u8()<<0);
        let msb_byte = (conf.mode.to_u8()<<4) | (conf.avg_num.to_u8()<<0);
        let old_conf = self._read_16bit_register(Registers::ACC_CONF);
        self._write_16bit_register(
            Registers::ACC_CONF,
            [old_conf[0] | msb_byte, lsb_byte]
        );
    }
    fn _set_gyro_config(&mut self,conf: &GyroConfig){
        let lsb_byte = (conf.bw.to_u8()<<7) | (conf.range.to_u8()<<4) | (conf.odr.to_u8()<<0);
        let msb_byte = (conf.mode.to_u8()<<4) | (conf.avg_num.to_u8()<<0);
        let old_conf = self._read_16bit_register(Registers::GYR_CONF);
        self._write_16bit_register(
            Registers::GYR_CONF,
            [old_conf[0] | msb_byte, lsb_byte]
        );
    }
}
