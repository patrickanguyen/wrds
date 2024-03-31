//! Decoder for RDS Messages.

use self::{psaf::{decode_0a, decode_0b}, rt::{decode_2a, decode_2b}, shared::Shared};
use crate::{error::Error, types::{Block1, Block2, Block3, Block4, GroupType, Message, Payload}};

mod psaf;
mod rt;
mod shared;

fn parse_payload(
    group_type: &GroupType,
    block2: &Block2,
    block3: &Block3,
    block4: &Block4,
) -> Result<Payload, Error> {
    match group_type {
        GroupType::ZeroA => Ok(Payload::ZeroA(decode_0a(block2, block3, block4))),
        GroupType::ZeroB => Ok(Payload::ZeroB(decode_0b(block2, block3, block4))),
        GroupType::TwoA => Ok(Payload::TwoA(decode_2a(block2, block3, block4))),
        GroupType::TwoB => Ok(Payload::TwoB(decode_2b(block2, block3, block4))),
        _ => Err(Error::Unimplemented(*group_type)),
    }
}

/// Decode RDS/RBDS Message from RDS Blocks
pub fn from_blocks(
    block1: &Block1,
    block2: &Block2,
    block3: &Block3,
    block4: &Block4,
) -> Result<Message, Error> {
    let shared = Shared::new(&block1, &block2);

    Ok(Message {
        pi: shared.pi,
        group_type: shared.group_type,
        tp: shared.tp,
        pty: shared.pty,
        payload: parse_payload(&shared.group_type, &block2, &block3, &block4)?,
    })
}
