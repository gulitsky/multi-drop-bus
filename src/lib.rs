use deku::prelude::*;

pub mod bill_validator;
pub mod cashless_device;
pub mod coin_changer;

#[derive(DekuRead)]
pub struct MasterBlock {
    pub response_code: ResponseCode,
}

#[derive(DekuRead)]
#[deku(type = "u8")]
pub enum ResponseCode {
    Acknowledgement = 0x00,
    Retransmit = 0xaa,
    NegativeAcknowledgement = 0xff,
}
