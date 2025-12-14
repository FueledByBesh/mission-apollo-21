pub struct Registers;

impl Registers {

    pub const CONFIG: u8 = 0x00;
    pub const EN_AA: u8 = 0x01;
    pub const EN_RXADDR: u8 = 0x02;
    pub const SETUP_AW: u8 = 0x03;
    pub const SETUP_RETR: u8 = 0x04;
    pub const STATUS: u8 = 0x07;
    pub const RX_ADDR_P0: u8 = 0x0A;
    pub const RX_ADDR_P1: u8 = 0x0B;
    pub const RX_ADDR_P2: u8 = 0x0C;
    pub const RX_ADDR_P3: u8 = 0x0D;
    pub const RX_ADDR_P4: u8 = 0x0E;
    pub const RX_ADDR_P5: u8 = 0x0F;
    pub const TX_ADDR: u8 = 0x10;
    pub const FIFO_STATUS: u8 = 0x17;

    pub const ACK_PAYLOAD: u8 = 0x19;
    pub const TX_PAYLOAD: u8 = 0x18;
    pub const RX_PAYLOAD: u8 = 0x1A;

    pub const DYNPD: u8 = 0x1C;
    pub const FEATURE: u8 = 0x1D;


}