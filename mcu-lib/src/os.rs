use eyre::{eyre, Result};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(FromPrimitive, ToPrimitive, Debug, Eq, PartialEq, Copy, Clone)]
pub enum OsMessageId {
    Echo = 0,
    ConsEchoCtrl = 1,
    Taskstat = 2,
    Mpstat = 3,
    DatetimeStr = 4,
    Reset = 5,
}

impl From<&OsMessageId> for u8 {
    fn from(group: &OsMessageId) -> Self {
        group.to_u8().unwrap() // Conversion can never fail.
    }
}
impl From<OsMessageId> for u8 {
    fn from(group: OsMessageId) -> Self {
        (&group).into()
    }
}
