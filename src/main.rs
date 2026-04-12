use mqtt_broker::{
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
    info!("MQTT Broker running on port 1883.");

    let mqtt_client = client::start_mqtt_client("sensor_data.db".to_string())
        .await
        .expect("Client failed to start");
    info!("Database listener active. Waiting for sensor data...");

    info!("Web server starting on http://0.0.0.0:3000");
    WebServer::new(mqtt_client)?.run().await?;

    Ok(())
}