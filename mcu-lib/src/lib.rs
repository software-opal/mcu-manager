use eyre::{eyre, Result};
use msg_id::MessageGroupId;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

mod msg_id;
mod os;

#[derive(FromPrimitive, ToPrimitive, Debug, Eq, PartialEq, Copy, Clone)]
pub enum MessageOptions {
    Read = 0,
    ReadRsp = 1,
    Write = 2,
    WriteRsp = 3,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct MessageHeader {
    op: MessageOptions,
    flags: u8,
    length: u16,
    seq: u8,
    message_id: MessageGroupId,
}

impl MessageHeader {
    pub fn from_bytes(bytes: &[u8]) -> Result<MessageHeader> {
        if bytes.len() != 8 {
            return Err(eyre!(
                "Incorrect header length. Expecting 8, got {}",
                bytes.len()
            ));
        }
        // Low 3 bytes are opcodes, of which there are only 4.
        let op = match FromPrimitive::from_u8(bytes[0] & 0x7) {
            Some(op) => op,
            None => return Err(eyre!("Unsupported opcode. Got {:0X}:", bytes[0] & 0x07)),
        };
        let flags = bytes[1];
        let length = u16::from_be_bytes([bytes[2], bytes[3]]);
        let group = u16::from_be_bytes([bytes[4], bytes[5]]);
        let seq = bytes[6];
        let id = bytes[7];
        return Ok(MessageHeader {
            op,
            flags,
            length,
            seq,
            message_id: MessageGroupId::from_raw(group, id),
        });
    }
    pub fn to_bytes(&self) -> [u8; 8] {
        let length_bytes = self.length.to_be_bytes();
        let group_bytes = self.message_id.group_id_u16().to_be_bytes();

        return [
            self.op.to_u8().unwrap(),
            self.flags,
            length_bytes[0],
            length_bytes[1],
            group_bytes[0],
            group_bytes[1],
            self.seq,
            self.message_id.message_id_u8(),
        ];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_from_bytes() {
        let header = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02];
        assert_eq!(
            MessageHeader::from_bytes(&header).unwrap(),
            MessageHeader {
                op: MessageOptions::Read,
                flags: 0,
                length: 0,
                group: MessageGroup::ManagementGroup(ManagementGroup::Os),
                seq: 0,
                id: 2,
            }
        )
    }

    #[test]
    fn test_header_to_bytes() {
        let header: [u8; 8] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02];
        assert_eq!(
            MessageHeader {
                op: MessageOptions::Read,
                flags: 0,
                length: 0,
                group: MessageGroup::ManagementGroup(ManagementGroup::Os),
                seq: 0,
                id: 2,
            }
            .to_bytes(),
            header
        );
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
