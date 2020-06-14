use crate::os::OsMessageId;
use eyre::{eyre, Result};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(FromPrimitive, ToPrimitive, Debug, Eq, PartialEq, Copy, Clone)]
pub enum ManagementGroup {
    /**
     * The first 64 groups are reserved for system level mcumgr commands.
     * Per-user commands are then defined after group 64.
     */
    Os = 0,
    Image = 1,
    Stat = 2,
    Config = 3,
    Log = 4,
    Crash = 5,
    Split = 6,
    Run = 7,
    Fs = 8,
    Shell = 9,
    Peruser = 64,
}
impl From<&ManagementGroup> for u16 {
    fn from(group: &ManagementGroup) -> Self {
        group.to_u16().unwrap() // Conversion can never fail.
    }
}
impl From<ManagementGroup> for u16 {
    fn from(group: ManagementGroup) -> Self {
        (&group).into()
    }
}
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum MessageGroupId {
    Os(OsMessageId),
    OtherId(ManagementGroup, u8),
    Other(u16, u8),
}

impl MessageGroupId {
    pub fn from_raw(group: u16, id: u8) -> Self {
        if let Some(mgmt_group) = ManagementGroup::from_u16(group) {
            match mgmt_group {
                ManagementGroup::Os => {
                    if let Some(v) = OsMessageId::from_u8(id) {
                        return MessageGroupId::Os(v);
                    }
                }
                _ => {}
            }
            return MessageGroupId::OtherId(mgmt_group, id);
        }
        return MessageGroupId::Other(group, id);
    }
    pub fn into_raw(&self) -> (u16, u8) {
        match self {
            MessageGroupId::OtherId(mgmt_group, id) => (mgmt_group.into(), *id),
            MessageGroupId::Other(group, id) => (*group, *id),
            MessageGroupId::Os(v) => (ManagementGroup::Os.into(), v.into()),
        }
    }
    pub fn group_id(&self) -> Option<ManagementGroup> {
        match self {
            MessageGroupId::OtherId(mgmt_group, id) => Some(*mgmt_group),
            MessageGroupId::Other(group, id) => None,
            MessageGroupId::Os(v) => Some(ManagementGroup::Os),
        }
    }
    pub fn group_id_u16(&self) -> u16 {
        match self {
            MessageGroupId::OtherId(mgmt_group, _id) => mgmt_group.into(),
            MessageGroupId::Other(group, _id) => *group,
            MessageGroupId::Os(v) => ManagementGroup::Os.into(),
        }
    }
    pub fn message_id_u8(&self) -> u8 {
        match self {
            MessageGroupId::OtherId(mgmt_group, id) => *id,
            MessageGroupId::Other(group, id) => *id,
            MessageGroupId::Os(v) => v.into(),
        }
    }
}
