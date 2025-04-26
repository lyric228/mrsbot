use azalea::Vec3;
use mrsbot::*;
use anyhow::{anyhow, Result};
use config::load_cfg;
use handler::handle;
use std::env;
use std::net::SocketAddr;
use std::path::Path;
use types::*;
use deadlock::deadlock_detection;
use azalea::JoinOpts;
use azalea::prelude::*;
use azalea::protocol::connect::Proxy;
use azalea_viaversion::ViaVersionPlugin;

#[tokio::main]
async fn main() -> Result<()> {
    deadlock_detection();
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <portal_config_path>", args[0]);
        return Err(anyhow!("Invalid arguments: expected <portal_config_path>"));
    }

    let portal_path = Path::new(&args[1]);
    let (_config, runtime_config, server_config, proxy_config) = load_cfg(portal_path)?;

    let host = server_config
        .host
        .ok_or_else(|| anyhow!("Server host is missing in config"))?;

    let address = if let Some(port) = server_config.port {
        format!("{host}:{port}")
    } else {
        host
    };

    let account = Account::offline(&runtime_config.bot.nickname);
    let options = if let (Some(proxy_host), Some(proxy_port)) =
        (proxy_config.host.as_deref(), proxy_config.port)
    {
        let proxy_addr = tokio::net::lookup_host(format!("{proxy_host}:{proxy_port}"))
            .await?
            .find(|addr| addr.is_ipv4())
            .ok_or_else(|| anyhow!("Could not resolve proxy host to an IPv4 address: {}", proxy_host))?;

        let proxy_socket_addr = match proxy_addr {
             SocketAddr::V4(addr) => addr,
             SocketAddr::V6(_) => return Err(anyhow!("IPv6 proxies are not supported yet")),
        };

        let proxy = Proxy::new(SocketAddr::V4(proxy_socket_addr), None);
        JoinOpts::new().proxy(proxy)
    } else {
        JoinOpts::new()
    };

    let version = server_config.version.unwrap_or_else(|| "AUTO".to_string());

    let initial_state = State {
        config: runtime_config,
        prev_pos: Vec3::ZERO,
        counters: Counters { 
            spawn: 0,
        },
    };
    let mut client_builder = ClientBuilder::new();

    if version.as_str() != "AUTO" {
        let via_version_plugin = ViaVersionPlugin::start(version).await;
        client_builder = client_builder.add_plugins(via_version_plugin);
    }

    client_builder
        .set_handler(handle)
        .set_state(initial_state)
        .start_with_opts(account, address, options)
        .await?
}
