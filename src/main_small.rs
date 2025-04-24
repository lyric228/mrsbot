use azalea::prelude::*;
use azalea::{Account, Client, ClientBuilder, Event};
use azalea::ecs::component::Component;
use anyhow::Result;

#[derive(Default, Clone, Component)]
struct State {
    pub nickname: String,
    pub password: String,
    pub portal_command: String,
}

async fn handle(bot: Client, event: Event, state: State) -> Result<()> {
    match event {
        Event::Login => {
            bot.chat(format!("/reg {}", state.password).as_str());
            bot.chat(format!("/login {}", state.password).as_str());
            println!("Attempted login/registration for {}", state.nickname);
        }
        Event::Chat(msg) => {
            let text = msg.content();
            if text.contains("авторизовались") {
                println!("Login detected, sending portal command: /{}", state.portal_command);
                bot.chat(format!("/{}", state.portal_command).as_str());
                // this command (e.g., /s4) switches the player to the specified portal from the lobby
                // (s4 means survival 4)
            }
        }
        Event::Packet(_) => {}
        Event::Disconnect(reason) => {
             println!("Disconnected: {:?}", reason.map(|r| r.to_ansi()));
        }
        _ => println!("{event:?}"),
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let address = "mc.masedworld.net"; // Server address
    let nickname = "test_93791046";
    let password = "qwerty123"; 
    let portal_command = "s4";  // The command to switch servers (e.g., s4, s3)

    let account = Account::offline(nickname);
    let initial_state = State {
        nickname: nickname.to_string(),
        password: password.to_string(),
        portal_command: portal_command.to_string(),
    };

    println!("Connecting to {address} as {nickname}...");

    ClientBuilder::new()
        .set_handler(handle)
        .set_state(initial_state)
        .start(account, address)
        .await?;
}
