use crate::prelude::*;
use strum::Display;
use local_ip_address::local_ip;

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub debug: bool,
    pub api: ApiConfig,
    #[serde(default)]
    pub broker: BrokerConfig,
    #[serde(rename = "database")]
    pub db: DbConfig,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ApiConfig {
    pub host: String,
    pub port: u16,
    #[serde(default)]
    pub protocol: ApiProtocol,
    #[serde(default)]
    pub domain: String,
    pub docs_endpoint: String,
}

impl ApiConfig {
    pub fn url(&self) -> String {
        match self.protocol {
            ApiProtocol::Https => format!("https://{}", self.domain),
            ApiProtocol::Http => format!("http://{}:{}", self.domain, self.port),
        }
    }
}

#[derive(Deserialize, Debug, Display, Default)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ApiProtocol {
    #[default]
    Http,
    Https,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct BrokerConfig {
    #[serde(default)]
    pub port: u16,
    #[serde(default)]
    pub domain: String,
}

impl BrokerConfig {
    pub fn addr(&self) -> String {
        format!("{}:{}", self.domain, self.port)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DbConfig {
    pub name: String,
}

pub fn get_config(broker_port: u16) -> Result<Config, config::ConfigError> {
    let base_path = get_base_path();

    let environment: String = std::env::var("RAILWAY_ENVIRONMENT_NAME")
        .unwrap_or_else(|_| "development".into());

    let config_directory = base_path.join(format!("config/{}", environment));

    let filename = "base.toml";

    let settings = config::Config::builder()
        .add_source(config::File::from(
            config_directory.join(filename),
        ))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    let mut app_config = settings.try_deserialize::<Config>()?;

    if environment == "production" {
        app_config.api.protocol = ApiProtocol::Https;
        app_config.api.domain = std::env::var("RAILWAY_PUBLIC_DOMAIN").expect("Failed to get Railway public domain.");

        app_config.broker.domain = std::env::var("RAILWAY_TCP_PROXY_DOMAIN").expect("Failed to get Railway TCP proxy domain.");
        app_config.broker.port = std::env::var("RAILWAY_TCP_PROXY_PORT").expect("Failed to get Railway TCP proxy port.").parse().unwrap();
    } else {
        let local_ip = local_ip().unwrap_or("127.0.0.1".parse().unwrap()).to_string();

        app_config.api.protocol = ApiProtocol::Http;
        app_config.api.domain = local_ip.clone();

        app_config.broker.domain = local_ip;
        app_config.broker.port = broker_port;
    }

    Ok(app_config)
}