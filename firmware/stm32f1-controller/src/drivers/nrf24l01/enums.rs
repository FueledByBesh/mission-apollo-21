
pub enum AddressWidth{
    ThreeBytes,
    FourBytes,
    FiveBytes
}

impl AddressWidth {

    pub fn get_bits(&self) -> u8{
        match self {
            AddressWidth::ThreeBytes => {0x01}
            AddressWidth::FourBytes => {0x02}
            AddressWidth::FiveBytes => {0x03}
        }
    }

}