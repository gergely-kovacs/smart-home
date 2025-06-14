use anyhow::Result;
use chrono::{Duration, Utc};
use log::debug;
use rand::Rng;
use sqlx::sqlite::SqlitePool;

use crate::models::{
    ControlSetpoint, ControlSetpointInput, Device, DeviceInput, DeviceType, Room, RoomInput,
    SensorReading, SensorReadingInput, SensorUnit, SetpointType, SetpointUnit, Site, SiteInput,
};

// TODO: implement extending existing site
pub async fn seed_db(pool: &SqlitePool, _should_extend: bool) -> Result<()> {
    let mut tx = pool.begin().await?;

    let now = Utc::now();

    debug!("Starting database seeding...");

    let site_input = SiteInput {
        name: "Main Office Site".to_string(),
        address: Some("123 Main St, Anytown, USA".to_string()),
    };
    let site = sqlx::query_as::<_, Site>(
        r#"
        INSERT INTO Site (name, address, created_at, updated_at)
        VALUES (?, ?, ?, ?)
        RETURNING id, name, address, created_at, updated_at
        "#,
    )
    .bind(site_input.name)
    .bind(site_input.address)
    .bind(now)
    .bind(now)
    .fetch_one(&mut *tx)
    .await?;
    debug!("Created Site: {:?}", site);

    let room_input = RoomInput {
        site_id: site.id,
        name: "Server Room".to_string(),
    };
    let room = sqlx::query_as::<_, Room>(
        r#"
        INSERT INTO Room (site_id, name, created_at, updated_at)
        VALUES (?, ?, ?, ?)
        RETURNING id, site_id, name, created_at, updated_at
        "#,
    )
    .bind(room_input.site_id)
    .bind(room_input.name)
    .bind(now)
    .bind(now)
    .fetch_one(&mut *tx)
    .await?;
    debug!("Created Room: {:?}", room);

    let device_input = DeviceInput {
        room_id: room.id,
        name: "Server Room Temperature Sensor".to_string(),
        device_type: DeviceType::TemperatureSensor,
        unique_identifier: None,
    };
    let device = sqlx::query_as::<_, Device>(
        r#"
        INSERT INTO Device (room_id, name, device_type, unique_identifier, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?)
        RETURNING id, room_id, name, device_type, unique_identifier, created_at, updated_at
        "#,
    )
    .bind(device_input.room_id)
    .bind(device_input.name)
    .bind(device_input.device_type as DeviceType)
    .bind(device_input.unique_identifier)
    .bind(now)
    .bind(now)
    .fetch_one(&mut *tx)
    .await?;
    debug!("Created Device: {:?}", device);

    let control_setpoint_input = ControlSetpointInput {
        device_id: device.id,
        setpoint_type: SetpointType::Temperature,
        value: "22.5".to_string(),
        unit: Some(SetpointUnit::Celsius),
    };
    let control_setpoint = sqlx::query_as::<_, ControlSetpoint>(
        r#"
        INSERT INTO ControlSetpoint (device_id, setpoint_type, value, unit, timestamp, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        RETURNING id, device_id, setpoint_type, value, unit, timestamp, created_at, updated_at
        "#
    )
    .bind(control_setpoint_input.device_id)
    .bind(control_setpoint_input.setpoint_type as SetpointType)
    .bind(control_setpoint_input.value)
    .bind(control_setpoint_input.unit as Option<SetpointUnit>)
    .bind(now)
    .bind(now)
    .bind(now)
    .fetch_one(&mut *tx)
    .await?;
    debug!("Created ControlSetpoint: {:?}", control_setpoint);

    let mut current_timestamp = now - Duration::minutes(100 * 5);
    // TODO: error handling when parsing fails
    let base_temperature: f64 = control_setpoint.value.parse().unwrap_or(22.5);
    for i in 0..100 {
        let mut rng = rand::rng();
        let random_offset: f64 = rng.random_range(-1.0..=1.0);
        let sensor_reading_value = format!("{:.1}", base_temperature + random_offset);
        let sensor_reading_input = SensorReadingInput {
            device_id: device.id,
            value: sensor_reading_value,
            unit: Some(SensorUnit::Celsius),
        };

        sqlx::query_as::<_, SensorReading>(
            r#"
            INSERT INTO SensorReading (device_id, value, unit, timestamp, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?)
            RETURNING id, device_id, value, unit, timestamp, created_at, updated_at
            "#,
        )
        .bind(sensor_reading_input.device_id)
        .bind(sensor_reading_input.value)
        .bind(sensor_reading_input.unit as Option<SensorUnit>)
        .bind(now)
        .bind(now)
        .bind(now)
        .fetch_one(&mut *tx)
        .await?;

        current_timestamp += Duration::minutes(5);

        if i % 10 == 0 && i != 0 {
            debug!("Created {} sensor readings...", i);
        }
    }

    tx.commit().await?;
    debug!("Database seeding completed successfully!");

    Ok(())
}
