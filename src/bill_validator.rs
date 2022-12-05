use deku::prelude::*;

#[derive(DekuRead)]
#[deku(type = "u8")]
pub enum Command {
    #[deku(id = "0x30")]
    Reset,
    #[deku(id = "0x31")]
    Setup,
    #[deku(id = "0x32")]
    Security { test: u8 },
    #[deku(id = "0x33")]
    Poll,
    #[deku(id = "0x34")]
    BillType { test: u8 },
    #[deku(id = "0x35")]
    Escrow { test: u8 },
    #[deku(id = "0x36")]
    Stacker,
    #[deku(id = "0x37")]
    Expansion { sub_command: ExpansionSubCommand },
}

#[derive(DekuRead)]
#[deku(type = "u8")]
pub enum ExpansionSubCommand {
    #[deku(id = "0x00")]
    LevelOneIdentificationWithoutOptionBits,
    #[deku(id = "0x01")]
    FeatureEnable,
    #[deku(id = "0x02")]
    Payout,
    #[deku(id = "0x03")]
    PayoutStatus,
    #[deku(id = "0x04")]
    PayoutValuePoll,
    #[deku(id = "0x05")]
    SendDiagnosticStatus,
    #[deku(id = "0x06")]
    SendControlledManualFillReport,
    #[deku(id = "0x07")]
    SendControlledManualPayoutReport,
    #[deku(id = "0xff")]
    Diagnostics,
}

pub struct CoinTypes {}
