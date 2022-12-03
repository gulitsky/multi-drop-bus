use deku::prelude::*;

pub mod coin_changer;

#[derive(DekuRead)]
pub struct Block {
    pub response_code: ResponseCode,
}

#[derive(DekuRead)]
#[deku(type = "u8")]
pub enum ResponseCode {
    Acknowledgement = 0x00,
    Retransmit = 0xaa,
    NegativeAcknowledgement = 0xff,
}
