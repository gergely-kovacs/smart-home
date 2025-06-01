use async_graphql::{Enum, InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, sqlx::Type)]
#[sqlx(rename_all = "PascalCase")]
pub enum DeviceType {
    TemperatureSensor,
    ThermostatController,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, sqlx::Type)]
#[sqlx(rename_all = "PascalCase")]
pub enum SensorUnit {
    Celsius,
    Fahrenheit,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, sqlx::Type)]
#[sqlx(rename_all = "PascalCase")]
pub enum SetpointType {
    Temperature,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, sqlx::Type)]
#[sqlx(rename_all = "PascalCase")]
pub enum SetpointUnit {
    Celsius,
    Fahrenheit,
}

#[derive(SimpleObject, Debug, Clone, FromRow)]
pub struct Site {
    pub id: i64,
    pub name: String,
    pub address: Option<String>,
    #[graphql(skip)]
    pub created_at: DateTime<Utc>,
    #[graphql(skip)]
    pub updated_at: DateTime<Utc>,
}

#[derive(SimpleObject, Debug, Clone, FromRow)]
pub struct Room {
    pub id: i64,
    pub site_id: i64,
    pub name: String,
    #[graphql(skip)]
    pub created_at: DateTime<Utc>,
    #[graphql(skip)]
    pub updated_at: DateTime<Utc>,
}

#[derive(SimpleObject, Debug, Clone, FromRow)]
pub struct Device {
    pub id: i64,
    pub room_id: i64,
    pub name: String,
    pub device_type: DeviceType,
    pub unique_identifier: Option<String>,
    #[graphql(skip)]
    pub created_at: DateTime<Utc>,
    #[graphql(skip)]
    pub updated_at: DateTime<Utc>,
}

#[derive(SimpleObject, Debug, Clone, FromRow)]
pub struct SensorReading {
    pub id: i64,
    pub device_id: i64,
    pub value: String,
    pub unit: Option<SensorUnit>,
    pub timestamp: DateTime<Utc>,
    #[graphql(skip)]
    pub created_at: DateTime<Utc>,
    #[graphql(skip)]
    pub updated_at: DateTime<Utc>,
}

#[derive(SimpleObject, Debug, Clone, FromRow)]
pub struct ControlSetpoint {
    pub id: i64,
    pub device_id: i64,
    pub setpoint_type: SetpointType,
    pub value: String,
    pub unit: Option<SetpointUnit>,
    pub timestamp: DateTime<Utc>,
    #[graphql(skip)]
    pub created_at: DateTime<Utc>,
    #[graphql(skip)]
    pub updated_at: DateTime<Utc>,
}

#[derive(InputObject, Debug, Clone)]
pub struct SiteInput {
    pub name: String,
    pub address: Option<String>,
}

#[derive(InputObject, Debug, Clone)]
pub struct RoomInput {
    pub site_id: i64,
    pub name: String,
}

#[derive(InputObject, Debug, Clone)]
pub struct DeviceInput {
    pub room_id: i64,
    pub name: String,
    pub device_type: DeviceType,
    pub unique_identifier: Option<String>,
}

#[derive(InputObject, Debug, Clone)]
pub struct SensorReadingInput {
    pub device_id: i64,
    pub value: String,
    pub unit: Option<SensorUnit>,
}

#[derive(InputObject, Debug, Clone)]
pub struct ControlSetpointInput {
    pub device_id: i64,
    pub setpoint_type: SetpointType,
    pub value: String,
    pub unit: Option<SetpointUnit>,
}
