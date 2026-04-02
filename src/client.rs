use crate::prelude::*;
use crate::db;
use rumqttc::{AsyncClient, Event, Incoming, MqttOptions, QoS};
use std::time::Duration;

pub async fn run_worker(db_path: &str) -> anyhow::Result<()> {
    let mut mqttoptions = MqttOptions::new("server-db-worker", "127.0.0.1", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    // Subscribe to all topics under 'sensors/'
    client.subscribe("sensors/#", QoS::AtLeastOnce).await?;

    // Process incoming messages and save to DB
    loop {
        match eventloop.poll().await {
            Ok(notification) => {
                if let Event::Incoming(Incoming::Publish(publish)) = notification {
                    let topic = publish.topic;
                    let payload = String::from_utf8_lossy(&publish.payload).to_string();

                    debug!("Received -> Topic: {}, Payload: {}", topic, payload);

                    if let Err(e) = db::insert_reading(db_path, &topic, &payload) {
                        error!("Database error: {}", e);
                    }
                }
            }
            Err(e) => {
                error!("MQTT connection error: {:?}", e);
                // Wait briefly before reconnecting/polling to avoid CPU spam on errors
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
}