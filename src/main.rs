use mqtt_rest_bridge::{
    Db, WebServer, MqttBroker, MqttClient,
    prelude::*,
    config::get_config,
    telemetry,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Start MQTT broker in a background thread
    let mqtt_broker = MqttBroker::init()?;

    let config = get_config(mqtt_broker.port)?;

    // Init the tracing subscriber
    let (subscriber, _guard) = telemetry::get_subscriber(&config).await?;
    telemetry::init(subscriber);

    info!(
        "MQTT Broker listening on {}. View the docs at {}{}.",
        config.broker.addr().yellow(),
        format!("{}/", config.api.url()).cyan(),
        config.api.mqtt_docs_endpoint.cyan().bold()
    );

    // Initialize database
    let db_path = &config.db.name;
    let db = Db::init(db_path)?;

    // Start MQTT client
    let mqtt_client_ip = "127.0.0.1";
    let mqtt_client = MqttClient::init(
        db,
        mqtt_client_ip,
        mqtt_broker.port,
    )
    .await
    .expect("MQTT Client failed to start.");
    info!(
        "MQTT Client fetching sensor data from {}...",
        format!("{}:{}", mqtt_client_ip, mqtt_broker.port).yellow()
    );

    // Start MQTT controller API
    info!(
        "MQTT Controller API listening on {}. View the docs at {}{}.",
        config.api.url().yellow(),
        format!("{}/", config.api.url()).cyan(),
        config.api.api_docs_endpoint.cyan().bold()
    );
    WebServer::new(&config, mqtt_client)?.run().await?;

    Ok(())
}