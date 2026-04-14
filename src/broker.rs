use rumqttd::{
    Broker,
    Config as RumqttdConfig
};
use std::thread;

pub fn spawn_background_thread(broker_config: &RumqttdConfig) -> anyhow::Result<()> {
    let broker_config = broker_config.clone();
    thread::spawn(|| {
        let mut broker = Broker::new(broker_config);

        // Blocking call
        broker.start().expect("MQTT Broker crashed");
    });

    Ok(())
}