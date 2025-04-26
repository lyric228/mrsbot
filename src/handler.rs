use crate::events::chat::parser::chat_parser;
use crate::events::init::init_handler;
use crate::events::login::login_handler;
use crate::events::packet::parser::packet_parser;
use crate::{events::spawn::spawn_handler, types::*};
use azalea::prelude::*;
use crate::events::disconnect::disconnect_handler;

pub async fn handle(bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    match event {
        Event::Init => init_handler(state),
        Event::Login => login_handler(bot, state),
        Event::Spawn => spawn_handler(state),
        Event::Chat(msg) => chat_parser(bot, state, msg),
        Event::Disconnect(reason) => disconnect_handler(state, reason),
        Event::Packet(packet) => packet_parser(bot, state, packet),
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

