use azalea::{chat::ChatPacket, prelude::*};
use crate::consts::*;
use crate::types::State;

pub fn chat_parser(bot: Client, state: State, msg: ChatPacket) {
    let portal = state.config.bot.portal.clone();
    let text = msg.content();

    if msg.sender() == Some(bot.username()) {
        // return Ok(());
    }
    if text.contains(JOIN_PORTAL_MSG1) || text.contains(JOIN_PORTAL_MSG2) {
        bot.send_command_packet(&portal);
    }
    if text.contains("/spam") && text.contains("zxclyric") {
        for _ in 0..10 {
            bot.chat("lol");
        }
    }

    println!("{}", msg.message().to_ansi());
}
