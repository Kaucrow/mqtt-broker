use mqtt_rest_bridge::{
    WebServer,
    prelude::*,
    config::{get_config, get_broker_config, get_broker_addr},
    telemetry,
    queries,
    db,
    broker,
    client,
};
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = get_config()?;

    // Init the tracing subscriber
    let (subscriber, _guard) = telemetry::get_subscriber(&config).await?;
    telemetry::init(subscriber);

    // Read DB queries and set global state
    queries::init()?;

    // Initialize database
    let db_path = "sensor_data.db";
    db::init(db_path)?;

    // Start MQTT broker in a background thread
    let broker_config = get_broker_config().expect("Failed to get broker config.");
    broker::spawn_background_thread(&broker_config)?;

    // Give the broker half a second to bind to the port
    tokio::time::sleep(Duration::from_millis(500)).await;
    info!(
        "MQTT Broker running on {}. Available topics: {}",
        get_broker_addr(&broker_config).yellow(),
        "'sensors/esp32', 'sensors/raspberry', 'commands/esp32/play', 'commands/raspberry/play'".magenta()
    );

    let mqtt_client = client::start_mqtt_client("sensor_data.db".to_string())
        .await
        .expect("Client failed to start.");
    info!("Database listener active. Waiting for sensor data...");

    info!(
        "MQTT Controller API starting on {}. View the docs at {}{}{}.",
        config.server.url().yellow(),
        config.server.url().cyan(),
        "/".cyan(),
        config.server.docs_endpoint.cyan().bold()
    );
    WebServer::new(&config, mqtt_client)?.run().await?;

    Ok(())
}