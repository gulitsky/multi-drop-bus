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
    CoinType {},
    #[deku(id = "0x0d")]
    Dispense {},
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
pub enum Status {
    NoCredit,
    DefectiveTubeSensor,
    DoubleArrival,
    AcceptorUnplugged,
    TubeJam,
    RomChecksumError,
    CoinRoutingError,
    ChangerBusy,
    ChangerWasReset,
    CoinJam,
    PossibleCreditedCoinRemoval,
}
