use crate::QUERIES;

use rusqlite::{params, Connection};

pub fn init(db_path: &str) -> anyhow::Result<()> {
    let conn = Connection::open(db_path)?;
    let queries = QUERIES.get().expect("Queries not initialized");

    conn.execute(&queries.sensor_readings.create, [])?;
    Ok(())
}

pub fn insert_reading(db_path: &str, topic: &str, payload: &str) -> anyhow::Result<()> {
    let conn = Connection::open(db_path)?;
    let queries = QUERIES.get().expect("Queries not initialized");

    conn.execute(
        &queries.sensor_readings.insert,
        params![topic, payload],
    )?;
    Ok(())
}