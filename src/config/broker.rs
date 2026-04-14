use crate::prelude::*;
use rumqttd::Config as RumqttdConfig;

pub fn get_broker_config() -> Result<RumqttdConfig, config::ConfigError> {
    let base_path = get_base_path();

    let environment: String = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "development".into());

    let config_directory = base_path.join(format!("config/{}", environment));

    let filename = "broker.toml";

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

    settings.try_deserialize::<RumqttdConfig>()
}

pub fn get_broker_addr(broker_config: &RumqttdConfig) -> String {
    let listen_addr = broker_config
        .v4
        .as_ref()
        .and_then(|servers| servers.values().next())
        .map(|server_settings| server_settings.listen.to_string())
        // Fallback to v5 if v4 isn't configured, or "unknown" if neither are
        .or_else(|| {
            broker_config
                .v5
                .as_ref()
                .and_then(|servers| servers.values().next())
                .map(|server_settings| server_settings.listen.to_string())
        })
        .unwrap_or_else(|| "unknown address".to_string());

    listen_addr
}