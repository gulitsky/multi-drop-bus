use deku::prelude::*;
use flagset::flags;

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
    CoinType {
        coin_enable: CoinTypeFlags,
        manual_dispense_enable: CoinTypeFlags,
    },
    #[deku(id = "0x0d")]
    Dispense {
        #[deku(bits = 4)]
        coin_type: u8,
        #[deku(bits = 4)]
        number_of_coins: u8,
    },
    #[deku(id = "0x0f")]
    Expansion { sub_command: ExpansionSubCommand },
}

#[derive(DekuRead)]
#[deku(type = "u8")]
pub enum ExpansionSubCommand {
    #[deku(id = "0x00")]
    Identification,
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

flags! {
    pub enum CoinTypeFlags: u16 {
        Type0,
        Type1,
        Type2,
        Type3,
        Type4,
        Type5,
        Type6,
        Type7,
        Type8,
        Type9,
        Type10,
        Type11,
        Type12,
        Type13,
        Type14,
        Type15,
    }
}

impl<'a> DekuRead<'a> for CoinTypeFlags {
    fn read(
        input: &'a deku::bitvec::BitSlice<u8, deku::bitvec::Msb0>,
        ctx: (),
    ) -> Result<(&'a deku::bitvec::BitSlice<u8, deku::bitvec::Msb0>, Self), DekuError>
    where
        Self: Sized,
    {
        todo!()
    }
}
