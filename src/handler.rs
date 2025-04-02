use azalea::prelude::*;
use crate::types::State;

pub async fn handle(bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    match event {
        Event::Spawn => {

        }
        Event::Chat(msg) => {
            if msg.username() == Some(bot.username()) {
                return Ok(());
            }

            println!("{}", msg.message().to_ansi());
        }

        _ => {}
    }

    Ok(())
}
