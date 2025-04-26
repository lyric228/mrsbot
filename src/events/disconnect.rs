use azalea::FormattedText;
use sysx::io::log::*;
use crate::types::*;

pub fn disconnect_handler(state: State, reason: Option<FormattedText>) {
    let portal = state.config.bot.portal.clone();
    let text = reason.unwrap_or_default().to_ansi();

    log!(INFO, "[{}] Disconnected: {}", portal, text);
}
