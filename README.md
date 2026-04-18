# MQTT Broker & REST API SCADA bridge

This project provides the backend infrastructure for a demo SCADA system. It bridges two microcontrollers (a Raspberry Pi and an ESP32) with a real-time dashboard and an SQLite database for data logs. The backend consists of a Rust-based MQTT broker for real-time telemetry/commands and a REST API for dashboard integration.

## System Architecture

* **REST API:** Built with Rust (Actix-Web) to handle HTTP commands from the real-time dashboard. OpenAPI documentation is included and visualized with Scalar.
* **MQTT Broker:** Facilitates bi-directional communication between the microcontrollers and the backend.
* **Microcontrollers:** A Raspberry Pi and an ESP32 equipped with sensors and buzzers.
* **Database:** SQLite is used to persist sensor telemetry and system state logs.
* **Real-Time Dashboard:** A frontend interface (not included in this repo) that monitors sensor values and issues manual override commands via the REST API.

## Core System Logic & Behavior

The system features a response mechanism between the two microcontrollers based on their sensor readings and the current command state.

**Cross-Linked Mode (Default)**
By default (or when "resumed"), the microcontrollers operate in a **Cross-Linked Mode**:
* The **ESP32** plays a melody based on the sensor values of the **Raspberry Pi**.
* The **Raspberry Pi** plays a melody based on the sensor values of the **ESP32**.

**Manual Overrides**
This automatic behavior can be manually overridden via the REST API/Dashboard. The system responds to three specific state commands:
* `resume` (Payload: `""`): Microcontrollers return to listening to each other's sensors.
* `play` (Payload: `"true"`): Manual override **ON**. The target device plays its melody continuously, disregarding sensor values entirely.
* `stop` (Payload: `"false"`): Manual override **OFF**. The target device stops playing entirely, disregarding sensor values.

**Device Synchronization & Presence**
To ensure the system stays in sync, devices use a presence-tracking mechanism:
* **Connection Status**: Each microcontroller broadcasts its status to `presence/{device}/status`.
* **Last Will and Testament (LWT)**: If a device loses its connection unexpectedly, the MQTT broker automatically publishes an `offline` message to this topic.
* **Automatic Reset**: When a new microcontroller connects (`online`), the other active devices receive this update and **restart their audio playback state**. This ensures that the "Cross-Linked" logic begins fresh whenever the network topology changes.

## MQTT Topic Structure

### Telemetry (Sensors)
Devices publish their current sensor readings to these topics:
* `sensors/raspberry`
* `sensors/esp32`

### Commands (Actuators)
Devices subscribe to these topics to listen for override commands from the API:
* `commands/raspberry/play`
* `commands/esp32/play`

### Presence (Synchronization)
Used for tracking the online/offline status of the devices:
* `presence/raspberry/status`
* `presence/esp32/status`

## Documentation

When the project is running, you can access the REST API and MQTT broker documentation from a browser.

### REST API (Scalar)
For the REST API, there's OpenAPI documentation powered by **Scalar**. You can test endpoints & view schemas at `http://localhost:8080/api/docs`.

### MQTT Broker (AsyncAPI)
You can view topics & message payloads using the **AsyncAPI** documentation for the MQTT broker at `http://localhost:8080/mqtt/docs`.

## Getting Started

### Prerequisites
* Rust toolchain
* SQLite3

### Usage
1. Clone this repository.
2. Compile and run the application with `cargo run`.