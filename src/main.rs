use mqtt_rest_bridge::{
    WebServer,
    prelude::*,
    config::get_config,
    telemetry,
    queries,
    db,
    broker,
    client,
};
use std::time::Duration;
use local_ip_address::local_ip;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Start MQTT broker in a background thread
    let (_, broker_port) = broker::spawn_background_thread()?;

    let config = get_config(broker_port)?;

    // Init the tracing subscriber
    let (subscriber, _guard) = telemetry::get_subscriber(&config).await?;
    telemetry::init(subscriber);

    // Read DB queries and set global state
    queries::init()?;

    // Initialize database
    let db_path = &config.db.name;
    db::init(db_path)?;

    // Give the broker half a second to bind to the port
    tokio::time::sleep(Duration::from_millis(500)).await;
    let local_ip = local_ip().unwrap_or("127.0.0.1".parse().unwrap()).to_string();
    info!(
        "MQTT Broker listening on {}. Available topics: {}.",
        format!("{}:{}", local_ip, broker_port).yellow(),
        "'sensors/esp32', 'sensors/raspberry', 'commands/esp32/play', 'commands/raspberry/play'".magenta()
    );

    // Start MQTT client
    let mqtt_client_ip = "127.0.0.1";
    let mqtt_client = client::start_mqtt_client(
        mqtt_client_ip,
        broker_port,
        config.db.name.clone()
    )
    .await
    .expect("MQTT Client failed to start.");
    info!(
        "MQTT Client fetching sensor data from {}...",
        format!("{}:{}", mqtt_client_ip, broker_port).yellow()
    );

    // Start MQTT controller API
    info!(
        "MQTT Controller API listening on {}. View the docs at {}{}.",
        config.api.url().yellow(),
        format!("{}/", config.api.url()).cyan(),
        config.api.docs_endpoint.cyan().bold()
    );
    WebServer::new(&config, mqtt_client)?.run().await?;

    Ok(())
}