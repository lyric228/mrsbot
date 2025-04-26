use azalea::prelude::*;
use crate::types::*;

pub fn login_handler(bot: Client, mut state: State) {
    state.flags.login = true;
    let password = state.config.bot.password.clone();

    bot.chat(format!("/reg {password}").as_str());
    bot.chat(format!("/login {password}").as_str());
}
