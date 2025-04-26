use azalea::prelude::*;
use crate::types::State;

pub fn tick_handler(bot: Client, mut state: State) {
    /*
    let pos = bot.position();

    if pos.distance_to(&state.prev_pos) > 1.0 {
        let warp = state.config.bot.warp.clone();
        let cmd = format!("/warp {warp:?}");
        bot.chat(cmd.as_str());
    }

    state.prev_pos = pos;
    */
}
