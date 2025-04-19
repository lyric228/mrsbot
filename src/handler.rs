use azalea::prelude::*;
use crate::types::State;

pub async fn handle(bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    match event {
        Event::Spawn => {
            bot.chat(format!("/reg {}", state.runtime_config.bot.password).as_str());
            bot.chat(format!("/login {}", state.runtime_config.bot.password).as_str());
            bot.chat(format!("/s{}", state.runtime_config.bot.password).as_str());
        }

        Event::Chat(msg) => {
            if msg.sender() == Some(bot.username()) {
                // return Ok(());
            }

            println!("{}", msg.message().to_ansi());
        }

        Event::Disconnect(text) => {
            
        }

        _ => {}
    }

    Ok(())
}
/* 
TODO --------------------------------------------------------------
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
