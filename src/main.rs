use std::net::SocketAddr;
use azalea::prelude::*;
use ::mrsbot::handler::handle;
use ::mrsbot::cfg::load_cfg;
use sysx::io::env::get_args;
use azalea::JoinOpts;
use sysx::Result;
use anyhow::anyhow;
use sysx::net::ipv4::create_ipv4_socket;
use azalea::protocol::connect::Proxy;

#[tokio::main]
async fn main() -> Result<()> {
    let args = get_args();
    let path = args.get(0).ok_or_else(|| anyhow!("Не указан путь к конфигу!"))?;

    let config = load_cfg(path.as_str()).map_err(|e| anyhow!("Не удалось загрузить конфиг '{}': {:?}", path, e))?;

    let server_config = config.server.ok_or_else(|| anyhow!("Конфигурация сервера не найдена в конфиге"))?;
    let bot_config = config.bot.ok_or_else(|| anyhow!("Конфигурация бота не найдена в конфиге"))?;
    let proxy_config = config.proxy;

    let host = server_config.host.ok_or_else(|| anyhow!("Хост сервера не указан в конфиге"))?;
    let address = match server_config.port {
        Some(port) => format!("{}:{}", host, port),
        None => host,
    };

    let nickname = bot_config.nickname.ok_or_else(|| anyhow!("Никнейм бота не указан в конфиге"))?;
    let account = Account::offline(&nickname);

    let options = if let Some(proxy_conf) = proxy_config {
        if let (Some(proxy_host), Some(proxy_port)) = (proxy_conf.host, proxy_conf.port) {
            let socket = create_ipv4_socket(&proxy_host, proxy_port)
                .expect("Не удалось использовать прокси");
            let proxy = Proxy::new(SocketAddr::V4(socket), None);
            JoinOpts::new().proxy(proxy)
        } else {
            JoinOpts::new()
        }
    } else {
        JoinOpts::new()
    };

    ClientBuilder::new()
        .set_handler(handle)
        .start_with_opts(account, address, options)
        .await
        .map_err(|e| anyhow!("Ошибка при запуске клиента: {:?}", e))?;
}
