use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result, anyhow};

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Config {
    pub server: Option<ServerConfig>,
    pub proxy: Option<ProxyConfig>,
    pub delay: Option<DelayConfig>,
    pub bot: Option<BotConfig>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct BotConfig {
    pub nickname: Option<String>,
    pub password: Option<String>,
    pub version: Option<String>, 
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct ProxyConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct ServerConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct DelayConfig {
    pub min: Option<MinDelayConfig>,
    pub max: Option<MaxDelayConfig>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct MinDelayConfig {
    pub discord: Option<i32>,
    pub global: Option<i32>,
    pub invite: Option<i32>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct MaxDelayConfig {
    pub discord: Option<i32>,
    pub global: Option<i32>,
    pub invite: Option<i32>,
}

fn load_toml_config(path: &Path) -> Result<Config> {
    if !path.exists() {
        return Ok(Config::default());
    }
    let content = fs::read_to_string(path)
        .with_context(|| format!("Не удалось прочитать файл конфигурации: {}", path.display()))?;
    let config: Config = toml::from_str(&content)
        .with_context(|| format!("Не удалось десериализовать TOML файл: {}", path.display()))?;
    Ok(config)
}

fn merge_option_structs<T>(base: Option<T>, overlay: Option<T>) -> Option<T>
where
    T: Mergeable<T> + Default + Clone + serde::de::DeserializeOwned + std::fmt::Debug + 'static,
{
    match (base, overlay) {
        (Some(b), Some(o)) => Some(merge_structs(b, o)),
        (Some(b), None) => Some(b),
        (None, Some(o)) => Some(o),
        (None, None) => None,
    }
}

fn merge_structs<T: Default + Clone>(base: T, overlay: T) -> T
where
    T: Mergeable<T> + Clone + Default + serde::de::DeserializeOwned + std::fmt::Debug + 'static,
{
    overlay.merge_from(base)
}

trait Mergeable<T> {
    fn merge_from(self, base: T) -> Self;
}

impl Mergeable<BotConfig> for BotConfig {
    fn merge_from(self, base: BotConfig) -> Self {
        BotConfig {
            nickname: self.nickname.or(base.nickname),
            password: self.password.or(base.password),
            version: self.version.or(base.version), 
        }
    }
}

impl Mergeable<ProxyConfig> for ProxyConfig {
    fn merge_from(self, base: ProxyConfig) -> Self {
        ProxyConfig {
            host: self.host.or(base.host),
            port: self.port.or(base.port),
        }
    }
}

impl Mergeable<ServerConfig> for ServerConfig {
    fn merge_from(self, base: ServerConfig) -> Self {
        ServerConfig {
            host: self.host.or(base.host),
            port: self.port.or(base.port),
        }
    }
}

impl Mergeable<DelayConfig> for DelayConfig {
    fn merge_from(self, base: DelayConfig) -> Self {
        DelayConfig {
            min: merge_option_structs(base.min, self.min),
            max: merge_option_structs(base.max, self.max),
        }
    }
}

impl Mergeable<MinDelayConfig> for MinDelayConfig {
    fn merge_from(self, base: MinDelayConfig) -> Self {
        MinDelayConfig {
            discord: self.discord.or(base.discord),
            global: self.global.or(base.global),
            invite: self.invite.or(base.invite),
        }
    }
}

impl Mergeable<MaxDelayConfig> for MaxDelayConfig {
    fn merge_from(self, base: MaxDelayConfig) -> Self {
        MaxDelayConfig {
            discord: self.discord.or(base.discord),
            global: self.global.or(base.global),
            invite: self.invite.or(base.invite),
        }
    }
}


impl Mergeable<Config> for Config {
    fn merge_from(self, base: Config) -> Self {
        Config {
            server: merge_option_structs(base.server, self.server),
            proxy: merge_option_structs(base.proxy, self.proxy),
            delay: merge_option_structs(base.delay, self.delay),
            bot: merge_option_structs(base.bot, self.bot),
        }
    }
}

pub fn load_cfg(config_path_str: &str) -> Result<Config> {
    let config_path = PathBuf::from(config_path_str);

    let default_config_path = Path::new("cfg/default.toml");
    let default_config = load_toml_config(&default_config_path)?;

    let server_type = config_path.parent()
        .ok_or_else(|| anyhow!("Некорректный путь к файлу конфигурации: {}", config_path_str))?
        .file_name()
        .ok_or_else(|| anyhow!("Некорректный путь к файлу конфигурации: {}", config_path_str))?;
    let all_config_path = Path::new("cfg").join(server_type).join("all.toml");
    let all_config = load_toml_config(&all_config_path)?;

    let portal_config = load_toml_config(&config_path)?;
    let mut final_config = default_config.merge_from(all_config);
    final_config = final_config.merge_from(portal_config);

    Ok(final_config)
}
