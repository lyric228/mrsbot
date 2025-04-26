use std::sync::Arc;

use azalea::{prelude::*, protocol::packets::game::ClientboundGamePacket};
use crate::types::State;

pub fn packet_parser(bot: Client, state: State, packet: Arc<ClientboundGamePacket>) {
    match packet.as_ref() {
        ClientboundGamePacket::PlayerPosition(position) => {
            let warp = state.config.bot.warp.clone();
            let cmd = format!("/warp {warp}");
            bot.chat(cmd.as_str());
        }

        _ => {}
    }
}
