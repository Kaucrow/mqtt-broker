use crate::config::get_broker_config;
use rumqttd::Broker;
use std::thread;

pub fn spawn_background_thread() -> anyhow::Result<()> {
    thread::spawn(|| {
        let broker_config = get_broker_config().expect("Failed to get broker config");
        let mut broker = Broker::new(broker_config);

        // Blocking call
        broker.start().expect("MQTT Broker crashed");
    });

    Ok(())
}