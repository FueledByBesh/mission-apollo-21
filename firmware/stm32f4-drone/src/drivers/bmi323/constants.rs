pub struct Registers;

impl Registers {
    /// Chip ID register address
    pub const CHIP_ID: u8 = 0x00;
    /// Error register address
    pub const ERR_REG: u8 = 0x01;
    /// Status register address
    pub const STATUS: u8 = 0x02;
    /// Accelerometer X-axis data register address
    pub const ACC_DATA_X: u8 = 0x03;
    /// Gyroscope X-axis data register address
    pub const GYR_DATA_X: u8 = 0x06;
    /// Temperature data register address
    pub const TEMP_DATA: u8 = 0x09;
    /// Accelerometer configuration register address
    pub const ACC_CONF: u8 = 0x20;
    /// Gyroscope configuration register address
    pub const GYR_CONF: u8 = 0x21;
    /// Command register address
    pub const CMD: u8 = 0x7E;
    /// Expected chip ID for BMI323
    pub const BMI323_CHIP_ID: u8 = 0x43;
    /// Soft reset command value
    pub const CMD_SOFT_RESET: u16 = 0xDEAF;
}

pub enum ODR{
    Odr0p78hz,
    Odr1p56hz,
    Odr3p12hz,
    Odr6p25hz,
    Odr12p5hz,
    Odr25hz,
    Odr50hz,
    Odr100hz,
    Odr200hz,
    Odr400hz,
    Odr800hz,
    Odr1600hz,
    Odr3200hz,
    Odr6400hz
}

impl ODR{
    pub fn to_u8(&self) -> u8{
        match self {
            ODR::Odr0p78hz => {0x1}
            ODR::Odr1p56hz => {0x2}
            ODR::Odr3p12hz => {0x3}
            ODR::Odr6p25hz => {0x4}
            ODR::Odr12p5hz => {0x5}
            ODR::Odr25hz => {0x6}
            ODR::Odr50hz => {0x7}
            ODR::Odr100hz => {0x8}
            ODR::Odr200hz => {0x9}
            ODR::Odr400hz => {0xA}
            ODR::Odr800hz => {0xB}
            ODR::Odr1600hz => {0xC}
            ODR::Odr3200hz => {0xD}
            ODR::Odr6400hz => {0xE}
        }
    }

    pub fn from_u8(val: u8) -> Self{
        match val {
            0x1 => ODR::Odr0p78hz,
            0x2 => ODR::Odr1p56hz,
            0x3 => ODR::Odr3p12hz,
            0x4 => ODR::Odr6p25hz,
            0x5 => ODR::Odr12p5hz,
            0x6 => ODR::Odr25hz,
            0x7 => ODR::Odr50hz,
            0x8 => ODR::Odr100hz,
            0x9 => ODR::Odr200hz,
            0xA => ODR::Odr400hz,
            0xB => ODR::Odr800hz,
            0xC => ODR::Odr1600hz,
            0xD => ODR::Odr3200hz,
            0xE => ODR::Odr6400hz,
            _ => panic!("Invalid ODR value {}", val)
        }
    }
}

pub enum Mode{
    Disable,
    LowPower,
    Normal,
    HighPerformance
}

impl Mode {
    pub fn to_u8(&self) -> u8 {
        match self {
            Mode::Disable => 0x0,
            Mode::LowPower => 0x3,
            Mode::Normal => 0x4,
            Mode::HighPerformance => 0x7,
        }
    }
    pub fn from_u8(val: u8) -> Self {
        match val {
            0x0 => Mode::Disable,
            0x3 => Mode::LowPower,
            0x4 => Mode::Normal,
            0x7 => Mode::HighPerformance,
            _ => panic!("Invalid mode value"),
        }
    }
}

pub enum AvgSampleNum{
    NoFilter,
    TwoSamples,
    FourSamples,
    EightSamples,
    SixteenSamples,
    ThirtyTwoSamples,
    SixtyFourSamples
}

impl AvgSampleNum {
    pub fn to_u8(&self) -> u8 {
        match self {
            AvgSampleNum::NoFilter => {0x0}
            AvgSampleNum::TwoSamples => {0x1}
            AvgSampleNum::FourSamples => {0x2}
            AvgSampleNum::EightSamples => {0x3}
            AvgSampleNum::SixteenSamples => {0x4}
            AvgSampleNum::ThirtyTwoSamples => {0x5}
            AvgSampleNum::SixtyFourSamples => {0x6}
        }
    }
    pub fn from_u8(val: u8) -> Self {
        match val {
            0x0 => AvgSampleNum::NoFilter,
            0x1 => AvgSampleNum::TwoSamples,
            0x2 => AvgSampleNum::FourSamples,
            0x3 => AvgSampleNum::EightSamples,
            0x4 => AvgSampleNum::SixteenSamples,
            0x5 => AvgSampleNum::ThirtyTwoSamples,
            0x6 => AvgSampleNum::SixtyFourSamples,
            _ => panic!("Invalid avg sample num value"),
        }
    }
}


pub enum AccRange{
    TwoG,
    FourG,
    EightG,
    SixteenG
}
impl AccRange {
    pub fn to_u8(&self) -> u8 {
        match self {
            AccRange::TwoG => {0x0}
            AccRange::FourG => {0x1}
            AccRange::EightG => {0x2}
            AccRange::SixteenG => {0x3}
        }
    }
    pub fn from_u8(val: u8) -> Self {
        match val {
            0x0 => AccRange::TwoG,
            0x1 => AccRange::FourG,
            0x2 => AccRange::EightG,
            0x3 => AccRange::SixteenG,
            _ => panic!("Invalid acc range value"),
        }
    }
}

pub enum GyroRange{
    A125,
    A250,
    A500,
    A1000,
    A2000
}

impl GyroRange {
    pub fn to_u8(&self) -> u8 {
        match self {
            GyroRange::A125 => { 0x0 }
            GyroRange::A250 => { 0x1 }
            GyroRange::A500 => { 0x2 }
            GyroRange::A1000 => { 0x3 }
            GyroRange::A2000 => { 0x4 }
        }
    }
    pub fn from_u8(val: u8) -> Self {
        match val {
            0x0 => GyroRange::A125,
            0x1 => GyroRange::A250,
            0x2 => GyroRange::A500,
            0x3 => GyroRange::A1000,
            0x4 => GyroRange::A2000,
            _ => panic!("Invalid gyro range value"),
        }
    }
}


pub enum  BW{
    Low,
    High
}
impl BW {
    pub fn to_u8(&self) -> u8 {
        match self {
            BW::Low => {0x0}
            BW::High => {0x1}
        }
    }
    pub fn from_u8(val: u8) -> Self {
        match val {
             0x0 => BW::Low,
             0x1 => BW::High,
            _ => panic!("Invalid avg sample num value"),
        }
    }
}

