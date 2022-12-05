use deku::prelude::*;

#[derive(DekuRead)]
#[deku(type = "u8")]
pub enum Command {
    #[deku(id = "0x08")]
    Reset,
    #[deku(id = "0x09")]
    Setup,
    #[deku(id = "0x0a")]
    TubeStatus,
    #[deku(id = "0x0b")]
    Poll,
    #[deku(id = "0x0c")]
    CoinType { test: u8 },
    #[deku(id = "0x0d")]
    Dispense { test: u8 },
    #[deku(id = "0x0f")]
    Expansion,
}

#[derive(DekuRead)]
pub struct SetupResponse {
    level: u8,
    coin_scaling_factor: u8,
    decimals_places: u8,
}

#[derive(DekuRead)]
#[deku(type = "u8")]
pub enum Status {
    EscrowRequest = 0b00000001,
    ChangerPayoutBusy = 0b00000010,
    NoCredit = 0b00000011,
    DefectiveTubeSensor = 0b00000100,
    DoubleArrival = 0b00000101,
    AcceptorUnplugged = 0b00000110,
    TubeJam = 0b00000111,
    RomChecksumError = 0b00001000,
    CoinRoutingError = 0b00001001,
    ChangerBusy = 0b00001010,
    ChangerWasReset = 0b00001011,
    CoinJam = 0b00001100,
    PossibleCreditedCoinRemoval = 0b00001101,
}
