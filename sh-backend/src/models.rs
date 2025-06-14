use async_graphql::{ComplexObject, Context, Enum, InputObject, Result, SimpleObject};

use chrono::{DateTime, Utc};
use sqlx::{FromRow, SqlitePool};

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
#[graphql(complex)]
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
#[graphql(complex)]
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
#[graphql(complex)]
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

#[ComplexObject]
impl Site {
    async fn rooms(&self, ctx: &Context<'_>) -> Result<Vec<Room>> {
        let pool = ctx.data::<SqlitePool>()?;
        let rooms = sqlx::query_as::<_, Room>("SELECT * FROM room WHERE site_id = ?")
            .bind(self.id)
            .fetch_all(pool)
            .await?;
        Ok(rooms)
    }
}

#[ComplexObject]
impl Room {
    async fn devices(&self, ctx: &Context<'_>) -> Result<Vec<Device>> {
        let pool = ctx.data::<SqlitePool>()?;
        let devices = sqlx::query_as::<_, Device>("SELECT * FROM device WHERE room_id = ?")
            .bind(self.id)
            .fetch_all(pool)
            .await?;
        Ok(devices)
    }
}

#[ComplexObject]
impl Device {
    async fn sensor_readings(&self, ctx: &Context<'_>) -> Result<Vec<SensorReading>> {
        let pool = ctx.data::<SqlitePool>()?;
        let readings =
            sqlx::query_as::<_, SensorReading>("SELECT * FROM sensor_reading WHERE device_id = ?")
                .bind(self.id)
                .fetch_all(pool)
                .await?;
        Ok(readings)
    }

    async fn control_setpoints(&self, ctx: &Context<'_>) -> Result<Vec<ControlSetpoint>> {
        let pool = ctx.data::<SqlitePool>()?;
        let setpoints = sqlx::query_as::<_, ControlSetpoint>(
            "SELECT * FROM control_setpoint WHERE device_id = ?",
        )
        .bind(self.id)
        .fetch_all(pool)
        .await?;
        Ok(setpoints)
    }
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
