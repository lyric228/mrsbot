use crate::types::*;
use azalea::prelude::*;
use sysx::io::log::*;
use crate::consts::*;

pub async fn handle(bot: Client, event: Event, mut state: State) -> anyhow::Result<()> {
    match event {
        Event::Login => {
            bot.chat(format!("/reg {}", state.config.bot.password).as_str());
            bot.chat(format!("/login {}", state.config.bot.password).as_str());
        }

        Event::Spawn => {
            state.counters.spawn += 1;
            println!("spawn {}", state.counters.spawn);
        }

        Event::Chat(msg) => {
            let text = msg.content();
            if msg.sender() == Some(bot.username()) {
                // return Ok(());
            }
            if text.contains(JOIN_PORTAL_MSG1) || text.contains(JOIN_PORTAL_MSG2) {
                let cmd = format!("/{}", state.config.bot.portal);
                bot.chat(cmd.as_str());
            }

            println!("{}", msg.message().to_ansi());
        }

        Event::Disconnect(reason) => {
            // Auto rejoin
            let text = reason.unwrap_or_default().to_ansi();
            log!(INFO, "[{}] Disconnected: {}", state.config.bot.portal, text);
        }

        _ => {}
    }

    Ok(())
}
/* 
TODO --------------------------------------------------------------
Event::Packet(packet) => {}
many triggers!


Сделать функцию (или что то подобное) чтобы превращать сообщение
в структуру, которая будет содержать информацию о том,
что именно было написано в сообщении, нике, типе сообщения, и тд.
Думаю сделать так чтобы у этой структуры была имплементация new 
которая и получала все эти параметры, принимая к себе на входе
только само сообщение.
После чего эту структуру можно очень удобно использовать везде
и не нужно будет каждый раз писать код для обработки каждого вида 
сообщения отдельно. 

Также для запуска на сервере (и автоматической установки в целом)
нужно написать скрипт для автоматического клонирования репозитория 
github (через скрипт curl), установки всего нужного, удалении 
всего не нужного и тд.

use ru proxy with ru server ip

x .txt 
v .db postgresql

/warp n930iqkfujo2
TODO --------------------------------------------------------------
*/
