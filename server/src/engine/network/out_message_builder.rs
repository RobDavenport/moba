use super::protobuf::{ClientMessage::*, ServerMessage::*};
use protobuf::Message as Message_imported_for_functions;

use crate::engine::messaging::messages::OutMessage;

pub fn build_out_message(out: OutMessage) -> Vec<u8> {
    match out {
        OutMessage::UpdateTick {
            frame,
            x,
            y,
            entity,
        } => update_tick(frame, x, y, entity),
        OutMessage::VerifyUuid(uuid) => verify_uuid(uuid),
        OutMessage::VerifiedUuid => verified_uuid(),
    }
}

fn update_tick(frame: u32, x: f32, y: f32, entity: u32) -> Vec<u8> {
    let mut output = ServerMessage::new();
    output.set_msgType(ServerMessage_ServerMessageType::UPDATETICK);

    let mut inner = ServerMessage_UpdateTick::new();
    inner.set_frame(frame);
    inner.set_entity(entity);
    inner.set_x(x);
    inner.set_y(y);

    output.set_updateTick(inner);
    output.write_to_bytes().unwrap()
}

fn verify_uuid(uuid: String) -> Vec<u8> {
    let mut output = ServerMessage::new();
    output.set_msgType(ServerMessage_ServerMessageType::VERIFYUUID);

    let mut inner = ServerMessage_VerifyUuid::new();
    inner.set_uuid(uuid);

    output.set_verifyUuid(inner);
    output.write_to_bytes().unwrap()
}

fn verified_uuid() -> Vec<u8> {
    let mut output = ServerMessage::new();
    output.set_msgType(ServerMessage_ServerMessageType::VERIFIEDUUID);
    output.set_verifiedUuid(ServerMessage_VerifiedUuid::new());
    output.write_to_bytes().unwrap()
}
