use mqtt_broker::{
    prelude::*,
    config::get_config,
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

    // Initialize Database
    let db_path = "sensor_data.db";
    db::init(db_path)?;

    // Start MQTT Broker in a background thread
    broker::spawn_background_thread()?;

    // Give the broker half a second to bind to the port
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Start Local DB Client
    info!("MQTT Broker running on port 1883.");
    info!("Database listener active. Waiting for sensor data...");

    // This will loop indefinitely
    client::run_worker(db_path).await?;

    Ok(())
}