use crate::{FromHciBytes, FromHciBytesError, ReadHci, ReadHciError, WriteHci};

mod cmd_mask;
mod event_masks;
mod feature_masks;
mod le;
mod macros;
mod primitives;
mod status;

pub use cmd_mask::*;
pub use event_masks::*;
pub use feature_masks::*;
pub use le::*;
pub(crate) use macros::param;
pub use status::*;

param!(struct BdAddr([u8; 6]));

impl BdAddr {
    pub fn new(val: [u8; 6]) -> Self {
        Self(val)
    }
}

param!(struct ConnHandle(u16));

impl ConnHandle {
    pub fn new(val: u16) -> Self {
        assert!(val <= 0xeff);
        Self(val)
    }
}

#[repr(transparent)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Duration<const N: u32 = 1>(u16);

impl<const N: u32> WriteHci for Duration<N> {
    fn size(&self) -> usize {
        WriteHci::size(&self.0)
    }

    fn write_hci<W: ::embedded_io::blocking::Write>(&self, writer: W) -> Result<(), W::Error> {
        self.0.write_hci(writer)
    }

    #[cfg(feature = "async")]
    #[allow(unused_mut)]
    async fn write_hci_async<W: ::embedded_io::asynch::Write>(&self, writer: W) -> Result<(), W::Error> {
        self.0.write_hci_async(writer).await
    }
}

impl<'de, const N: u32> FromHciBytes<'de> for Duration<N> {
    fn from_hci_bytes(data: &'de [u8]) -> Result<(Self, &'de [u8]), FromHciBytesError> {
        u16::from_hci_bytes(data).map(|(x, y)| (Self(x), y))
    }
}

impl<'de, const N: u32> ReadHci<'de> for Duration<N> {
    fn read_hci<R: embedded_io::blocking::Read>(
        reader: R,
        buf: &'de mut [u8],
    ) -> Result<(Self, &'de mut [u8]), ReadHciError<R::Error>> {
        u16::read_hci(reader, buf).map(|(x, y)| (Self(x), y))
    }

    #[cfg(feature = "async")]
    async fn read_hci_async<R: embedded_io::asynch::Read>(
        reader: R,
        buf: &'de mut [u8],
    ) -> Result<(Self, &'de mut [u8]), ReadHciError<R::Error>> {
        u16::read_hci_async(reader, buf).await.map(|(x, y)| (Self(x), y))
    }
}

impl<const N: u32> Duration<N> {
    pub fn from_u16(val: u16) -> Self {
        Self(val)
    }

    pub fn from_micros(val: u32) -> Self {
        Self::from_u16((val / (625 * N)) as u16)
    }

    pub fn from_millis(val: u32) -> Self {
        Self::from_u16((unwrap!(val.checked_mul(8)) / (5 * N)) as u16)
    }

    pub fn from_secs(val: u32) -> Self {
        Self::from_millis(unwrap!(val.checked_mul(1000)))
    }

    pub fn as_u16(&self) -> u16 {
        self.0
    }

    pub fn as_micros(&self) -> u32 {
        u32::from(self.as_u16()) * (625 * N)
    }

    pub fn as_millis(&self) -> u32 {
        (u32::from(self.as_u16()) * (5 * N)) / 8
    }

    pub fn as_secs(&self) -> u32 {
        self.as_millis() / 1000
    }
}

param!(
    enum DisconnectReason {
        AuthenticationFailure = 0x05,
        RemoteUserTerminatedConn = 0x13,
        RemoteDeviceTerminatedConnLowResources = 0x14,
        RemoteDeviceTerminatedConnPowerOff = 0x15,
        UnsupportedRemoteFeature = 0x1A,
        PairingWithUnitKeyNotSupported = 0x29,
        UnacceptableConnParameters = 0x3b,
    }
);

param! {
    enum PowerLevelKind {
        Current = 0,
        Maximum = 1,
    }
}

param! {
    enum ControllerToHostFlowControl {
        Off = 0,
        AclOnSyncOff = 1,
        AclOffSyncOn = 2,
        BothOn = 3,
    }
}

param!(struct CoreSpecificationVersion(u8));

impl CoreSpecificationVersion {
    pub const VERSION_1_0B: CoreSpecificationVersion = CoreSpecificationVersion(0x00);
    pub const VERSION_1_1: CoreSpecificationVersion = CoreSpecificationVersion(0x01);
    pub const VERSION_1_2: CoreSpecificationVersion = CoreSpecificationVersion(0x02);
    pub const VERSION_2_0_EDR: CoreSpecificationVersion = CoreSpecificationVersion(0x03);
    pub const VERSION_2_1_EDR: CoreSpecificationVersion = CoreSpecificationVersion(0x04);
    pub const VERSION_3_0_HS: CoreSpecificationVersion = CoreSpecificationVersion(0x05);
    pub const VERSION_4_0: CoreSpecificationVersion = CoreSpecificationVersion(0x06);
    pub const VERSION_4_1: CoreSpecificationVersion = CoreSpecificationVersion(0x07);
    pub const VERSION_4_2: CoreSpecificationVersion = CoreSpecificationVersion(0x08);
    pub const VERSION_5_0: CoreSpecificationVersion = CoreSpecificationVersion(0x09);
    pub const VERSION_5_1: CoreSpecificationVersion = CoreSpecificationVersion(0x0A);
    pub const VERSION_5_2: CoreSpecificationVersion = CoreSpecificationVersion(0x0B);
    pub const VERSION_5_3: CoreSpecificationVersion = CoreSpecificationVersion(0x0C);
}
