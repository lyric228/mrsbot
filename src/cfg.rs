use std::path::Path;
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Config {
    pub bot: Option<BotConfig>,
    pub server: Option<ServerConfig>,
    pub proxy: Option<ProxyConfig>,
    pub delay: Option<DelayConfig>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct BotConfig {
    pub nickname: Option<String>,
    pub password: Option<String>,
    pub warp: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct ServerConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub version: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct ProxyConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct DelayConfig {
    pub min: Option<Delay>,
    pub max: Option<Delay>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Delay {
    pub global: Option<i32>,
    pub discord: Option<i32>,
    pub invite: Option<i32>,
}

#[derive(Debug, Clone, Default)]
pub struct RuntimeConfig {
    pub bot: BotConfigResolved,
    pub delay: DelayResolved,
}

#[derive(Debug, Clone, Default)]
pub struct BotConfigResolved {
    pub nickname: String,
    pub password: String,
    pub warp: String,
}

#[derive(Debug, Clone, Default)]
pub struct DelayResolved {
    pub min: DelayValues,
    pub max: DelayValues,
}

#[derive(Debug, Clone, Default)]
pub struct DelayValues {
    pub global: i32,
    pub discord: i32,
    pub invite: i32,
}

impl Config {
    pub fn resolve(&self) -> Result<RuntimeConfig> {
        let bot = self.bot.as_ref().ok_or_else(|| anyhow!("Bot config is missing"))?;
        let nickname = bot.nickname.as_ref().ok_or_else(|| anyhow!("Bot nickname is missing"))?.clone();
        let password = bot.password.as_ref().ok_or_else(|| anyhow!("Bot password is missing"))?.clone();
        let warp = bot.warp.as_ref().ok_or_else(|| anyhow!("Bot warp is missing"))?.clone();

        let delay = self.delay.as_ref().ok_or_else(|| anyhow!("Delay config is missing"))?;
        let min_delay = delay.min.as_ref().unwrap_or_else(|| delay.max.as_ref().unwrap_or_else(|| {
            panic!("Either min or max delay should be present")
        }));
        let max_delay = delay.max.as_ref().unwrap_or(min_delay);

        let min = DelayValues {
            global: min_delay.global.ok_or_else(|| anyhow!("Min global delay is missing"))?,
            discord: min_delay.discord.ok_or_else(|| anyhow!("Min discord delay is missing"))?,
            invite: min_delay.invite.ok_or_else(|| anyhow!("Min invite delay is missing"))?,
        };

        let max = DelayValues {
            global: max_delay.global.ok_or_else(|| anyhow!("Max global delay is missing"))?,
            discord: max_delay.discord.ok_or_else(|| anyhow!("Max discord delay is missing"))?,
            invite: max_delay.invite.ok_or_else(|| anyhow!("Max invite delay is missing"))?,
        };

        Ok(RuntimeConfig {
            bot: BotConfigResolved { nickname, password, warp },
            delay: DelayResolved { min, max },
        })
    }
}

fn load_toml_config(path: &Path) -> Result<Config> {
    if !path.exists() {
        return Ok(Config::default());
    }
    let content = std::fs::read_to_string(path)
        .context(format!("Failed to read config file: {}", path.display()))?;
    let config: Config = toml::from_str(&content)
        .context(format!("Failed to deserialize TOML file: {}", path.display()))?;
    Ok(config)
}

pub fn load_cfg(portal_path: &Path) -> Result<(Config, RuntimeConfig, ServerConfig, ProxyConfig)> {
    let server_path = portal_path.parent().unwrap().join("all.toml");
    let default_path = portal_path.parent().unwrap().parent().unwrap().join("default.toml");

    let default_config = load_toml_config(&default_path)?;
    let server_config = load_toml_config(&server_path)?;
    let portal_config = load_toml_config(portal_path)?;

    let merged_config = merge_configs(&default_config, &server_config, &portal_config);

    let runtime_config = merged_config.resolve()
        .context("Failed to resolve merged configuration")?;

    let server_config_to_return = merged_config.server.clone()
        .ok_or_else(|| anyhow!("Merged configuration is missing 'server' section"))?;
    let proxy_config_to_return = merged_config.proxy.clone().unwrap_or_default();
    Ok((merged_config, runtime_config, server_config_to_return, proxy_config_to_return))
}

fn merge_configs(default: &Config, server: &Config, portal: &Config) -> Config {
    // --- BotConfig ---
    let bot_nickname = portal.bot.as_ref().and_then(|b| b.nickname.clone())
        .or_else(|| server.bot.as_ref().and_then(|b| b.nickname.clone()))
        .or_else(|| default.bot.as_ref().and_then(|b| b.nickname.clone()));

    let bot_password = portal.bot.as_ref().and_then(|b| b.password.clone())
        .or_else(|| server.bot.as_ref().and_then(|b| b.password.clone()))
        .or_else(|| default.bot.as_ref().and_then(|b| b.password.clone()));

    let bot_warp = portal.bot.as_ref().and_then(|b| b.warp.clone())
        .or_else(|| server.bot.as_ref().and_then(|b| b.warp.clone()))
        .or_else(|| default.bot.as_ref().and_then(|b| b.warp.clone()));

    let merged_bot = if bot_nickname.is_some() || bot_password.is_some() || bot_warp.is_some() {
        Some(BotConfig {
            nickname: bot_nickname,
            password: bot_password,
            warp: bot_warp,
        })
    } else {
        None
    };

    // --- ServerConfig ---
    let server_host = portal.server.as_ref().and_then(|s| s.host.clone())
        .or_else(|| server.server.as_ref().and_then(|s| s.host.clone()))
        .or_else(|| default.server.as_ref().and_then(|s| s.host.clone()));

    let server_port = portal.server.as_ref().and_then(|s| s.port)
        .or_else(|| server.server.as_ref().and_then(|s| s.port))
        .or_else(|| default.server.as_ref().and_then(|s| s.port));

    let server_version = portal.server.as_ref().and_then(|s| s.version.clone())
        .or_else(|| server.server.as_ref().and_then(|s| s.version.clone()))
        .or_else(|| default.server.as_ref().and_then(|s| s.version.clone()));

    let merged_server = if server_host.is_some() || server_port.is_some() || server_version.is_some() {
        Some(ServerConfig {
            host: server_host,
            port: server_port,
            version: server_version,
        })
    } else {
        None
    };

    // --- ProxyConfig ---
    let proxy_host = portal.proxy.as_ref().and_then(|p| p.host.clone())
        .or_else(|| server.proxy.as_ref().and_then(|p| p.host.clone()))
        .or_else(|| default.proxy.as_ref().and_then(|p| p.host.clone()));

    let proxy_port = portal.proxy.as_ref().and_then(|p| p.port)
        .or_else(|| server.proxy.as_ref().and_then(|p| p.port))
        .or_else(|| default.proxy.as_ref().and_then(|p| p.port));

    let merged_proxy = if proxy_host.is_some() || proxy_port.is_some() {
        Some(ProxyConfig {
            host: proxy_host,
            port: proxy_port,
        })
    } else {
        None
    };

    // --- DelayConfig ---
    // Сначала объединяем поля для min delay
    let min_delay_global = portal.delay.as_ref().and_then(|d| d.min.as_ref()).and_then(|m| m.global)
        .or_else(|| server.delay.as_ref().and_then(|d| d.min.as_ref()).and_then(|m| m.global))
        .or_else(|| default.delay.as_ref().and_then(|d| d.min.as_ref()).and_then(|m| m.global));
    let min_delay_discord = portal.delay.as_ref().and_then(|d| d.min.as_ref()).and_then(|m| m.discord)
        .or_else(|| server.delay.as_ref().and_then(|d| d.min.as_ref()).and_then(|m| m.discord))
        .or_else(|| default.delay.as_ref().and_then(|d| d.min.as_ref()).and_then(|m| m.discord));
    let min_delay_invite = portal.delay.as_ref().and_then(|d| d.min.as_ref()).and_then(|m| m.invite)
        .or_else(|| server.delay.as_ref().and_then(|d| d.min.as_ref()).and_then(|m| m.invite))
        .or_else(|| default.delay.as_ref().and_then(|d| d.min.as_ref()).and_then(|m| m.invite));

    let merged_min_delay = if min_delay_global.is_some() || min_delay_discord.is_some() || min_delay_invite.is_some() {
        Some(Delay {
            global: min_delay_global,
            discord: min_delay_discord,
            invite: min_delay_invite,
        })
    } else {
        None
    };

    // Теперь объединяем поля для max delay, аналогично min
    let max_delay_global = portal.delay.as_ref().and_then(|d| d.max.as_ref()).and_then(|m| m.global)
        .or_else(|| server.delay.as_ref().and_then(|d| d.max.as_ref()).and_then(|m| m.global))
        .or_else(|| default.delay.as_ref().and_then(|d| d.max.as_ref()).and_then(|m| m.global));
    let max_delay_discord = portal.delay.as_ref().and_then(|d| d.max.as_ref()).and_then(|m| m.discord)
        .or_else(|| server.delay.as_ref().and_then(|d| d.max.as_ref()).and_then(|m| m.discord))
        .or_else(|| default.delay.as_ref().and_then(|d| d.max.as_ref()).and_then(|m| m.discord));
    let max_delay_invite = portal.delay.as_ref().and_then(|d| d.max.as_ref()).and_then(|m| m.invite)
        .or_else(|| server.delay.as_ref().and_then(|d| d.max.as_ref()).and_then(|m| m.invite))
        .or_else(|| default.delay.as_ref().and_then(|d| d.max.as_ref()).and_then(|m| m.invite));

    let merged_max_delay = if max_delay_global.is_some() || max_delay_discord.is_some() || max_delay_invite.is_some() {
        Some(Delay {
            global: max_delay_global,
            discord: max_delay_discord,
            invite: max_delay_invite,
        })
    } else {
        None
    };

    let merged_delay = if merged_min_delay.is_some() || merged_max_delay.is_some() {
        Some(DelayConfig {
            min: merged_min_delay,
            max: merged_max_delay,
        })
    } else {
        None
    };


    Config {
        bot: merged_bot,
        server: merged_server,
        proxy: merged_proxy,
        delay: merged_delay,
    }
}
