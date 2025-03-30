use std::sync::Arc;

use azalea::prelude::*;
use parking_lot::Mutex;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let account = Account::offline("bot");

    ClientBuilder::new()
        .set_handler(handle)
        .start(account, "localhost")
        .await
        .unwrap();
    Ok(())
}

#[derive(Default, Clone, Component)]
pub struct State {}

async fn handle(bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    match event {
        Event::Chat(m) => {
            println!("{}", m.message().to_ansi());
        }
        _ => {}
    }

    Ok(())
}
