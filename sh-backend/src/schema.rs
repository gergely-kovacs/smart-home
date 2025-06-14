use async_graphql::{Context, EmptySubscription, FieldError, FieldResult, Object, Result, Schema};
use chrono::{DateTime, Utc};
use sqlx::sqlite::SqlitePool;

use crate::models::{
    ControlSetpoint, ControlSetpointInput, Device, DeviceInput, DeviceType, Room, RoomInput,
    SensorReading, SensorReadingInput, SensorUnit, SetpointType, SetpointUnit, Site, SiteInput,
};

pub struct SiteQueryRoot;

#[Object]
impl SiteQueryRoot {
    async fn sites(&self, ctx: &Context<'_>) -> FieldResult<Vec<Site>> {
        let pool = ctx.data::<SqlitePool>()?;
        let sites = sqlx::query_as!(
            Site,
            r#"
            SELECT id, name, address, created_at as "created_at!: DateTime<Utc>", updated_at as "updated_at!: DateTime<Utc>"
            FROM Site
            "#
        )
        .fetch_all(pool)
        .await?;
        Ok(sites)
    }

    async fn site(&self, ctx: &Context<'_>, id: i64) -> Result<Option<Site>> {
        let pool = ctx.data::<SqlitePool>()?;
        let site = sqlx::query_as::<_, Site>("SELECT * FROM site WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?;
        Ok(site)
    }

    async fn room(&self, ctx: &Context<'_>, id: i64) -> FieldResult<Option<Room>> {
        let pool = ctx.data::<SqlitePool>()?;
        let room = sqlx::query_as!(
            Room,
            r#"
            SELECT id, site_id, name, created_at as "created_at!: DateTime<Utc>", updated_at as "updated_at!: DateTime<Utc>"
            FROM Room
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;
        Ok(room)
    }

    async fn devices_in_room(&self, ctx: &Context<'_>, room_id: i64) -> FieldResult<Vec<Device>> {
        let pool = ctx.data::<SqlitePool>()?;
        let devices = sqlx::query_as!(
            Device,
            r#"
            SELECT id, room_id, name, device_type as "device_type: DeviceType", unique_identifier, created_at as "created_at!: DateTime<Utc>", updated_at as "updated_at!: DateTime<Utc>"
            FROM Device
            WHERE room_id = ?
            "#,
            room_id
        )
        .fetch_all(pool)
        .await?;
        Ok(devices)
    }

    async fn latest_sensor_reading(
        &self,
        ctx: &Context<'_>,
        device_id: i64,
    ) -> FieldResult<Option<SensorReading>> {
        let pool = ctx.data::<SqlitePool>()?;
        let reading = sqlx::query_as!(
            SensorReading,
            r#"
            SELECT id, device_id, value, unit as "unit: SensorUnit", timestamp as "timestamp!: DateTime<Utc>", created_at as "created_at!: DateTime<Utc>", updated_at as "updated_at!: DateTime<Utc>"
            FROM SensorReading
            WHERE device_id = ?
            ORDER BY timestamp DESC
            LIMIT 1
            "#,
            device_id
        )
        .fetch_optional(pool)
        .await?;
        Ok(reading)
    }

    async fn latest_control_setpoint(
        &self,
        ctx: &Context<'_>,
        device_id: i64,
    ) -> FieldResult<Option<ControlSetpoint>> {
        let pool = ctx.data::<SqlitePool>()?;
        let setpoint = sqlx::query_as!(
            ControlSetpoint,
            r#"
            SELECT id, device_id, setpoint_type as "setpoint_type: SetpointType", value, unit as "unit: SetpointUnit", timestamp as "timestamp!: DateTime<Utc>", created_at as "created_at!: DateTime<Utc>", updated_at as "updated_at!: DateTime<Utc>"
            FROM ControlSetpoint
            WHERE device_id = ?
            ORDER BY timestamp DESC
            LIMIT 1
            "#,
            device_id
        )
        .fetch_optional(pool)
        .await?;
        Ok(setpoint)
    }
}

pub struct SiteMutationRoot;

#[Object]
impl SiteMutationRoot {
    async fn create_site(&self, ctx: &Context<'_>, input: SiteInput) -> FieldResult<Site> {
        let pool = ctx.data::<SqlitePool>()?;
        let result = sqlx::query_as!(
            Site,
            r#"
            INSERT INTO Site (name, address)
            VALUES (?, ?)
            RETURNING id, name, address, created_at as "created_at!: DateTime<Utc>", updated_at as "updated_at!: DateTime<Utc>"
            "#,
            input.name,
            input.address
        )
        .fetch_one(pool)
        .await?;
        Ok(result)
    }

    async fn create_room(&self, ctx: &Context<'_>, input: RoomInput) -> FieldResult<Room> {
        let pool = ctx.data::<SqlitePool>()?;
        let site_exists: (i64,) =
            sqlx::query_as::<_, (i64,)>("SELECT COUNT(id) FROM Site WHERE id = ?")
                .bind(input.site_id)
                .fetch_one(pool)
                .await?;
        if site_exists.0 == 0 {
            return Err(FieldError::new(format!(
                "Site with ID {} does not exist",
                input.site_id
            )));
        }

        let result = sqlx::query_as::<_, Room>(
            r#"
            INSERT INTO Room (site_id, name)
            VALUES (?, ?)
            RETURNING id, site_id, name, created_at as "created_at!: DateTime<Utc>", updated_at as "updated_at!: DateTime<Utc>"
            "#
        )
        .bind(input.site_id)
        .bind(input.name)
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    async fn create_device(&self, ctx: &Context<'_>, input: DeviceInput) -> FieldResult<Device> {
        let pool = ctx.data::<SqlitePool>()?;
        let room_exists: (i64,) =
            sqlx::query_as::<_, (i64,)>("SELECT COUNT(id) FROM Room WHERE id = ?")
                .bind(input.room_id)
                .fetch_one(pool)
                .await?;
        if room_exists.0 == 0 {
            return Err(FieldError::new(format!(
                "Room with ID {} does not exist",
                input.room_id
            )));
        }

        let result = sqlx::query_as::<_, Device>(
            r#"
            INSERT INTO Device (room_id, name, device_type, unique_identifier)
            VALUES (?, ?, ?, ?)
            RETURNING id, room_id, name, device_type as "device_type: DeviceType", unique_identifier, created_at as "created_at!: DateTime<Utc>", updated_at as "updated_at!: DateTime<Utc>"
            "#
        )
        .bind(input.room_id)
        .bind(input.name)
        .bind(input.device_type as DeviceType)
        .bind(input.unique_identifier)
        .fetch_one(pool)
        .await?;
        Ok(result)
    }

    async fn create_sensor_reading(
        &self,
        ctx: &Context<'_>,
        input: SensorReadingInput,
    ) -> FieldResult<SensorReading> {
        let pool = ctx.data::<SqlitePool>()?;
        let result = sqlx::query_as::<_, SensorReading>(
            r#"
            INSERT INTO SensorReading (device_id, value, unit)
            VALUES (?, ?, ?)
            RETURNING id, device_id, value, unit as "unit: SensorUnit", timestamp as "timestamp!: DateTime<Utc>", created_at as "created_at!: DateTime<Utc>", updated_at as "updated_at!: DateTime<Utc>"
            "#
        )
        .bind(input.device_id)
        .bind(input.value)
        .bind(input.unit as Option<SensorUnit>)
        .fetch_one(pool)
        .await?;
        Ok(result)
    }

    async fn create_control_setpoint(
        &self,
        ctx: &Context<'_>,
        input: ControlSetpointInput,
    ) -> FieldResult<ControlSetpoint> {
        let pool = ctx.data::<SqlitePool>()?;
        let result = sqlx::query_as::<_, ControlSetpoint>(
            r#"
            INSERT INTO ControlSetpoint (device_id, setpoint_type, value, unit)
            VALUES (?, ?, ?, ?)
            RETURNING id, device_id, setpoint_type as "setpoint_type: SetpointType", value, unit as "unit: SetpointUnit", timestamp as "timestamp!: DateTime<Utc>", created_at as "created_at!: DateTime<Utc>", updated_at as "updated_at!: DateTime<Utc>"
            "#
        )
        .bind(input.device_id)
        .bind(input.setpoint_type as SetpointType)
        .bind(input.value)
        .bind(input.unit as Option<SetpointUnit>)
        .fetch_one(pool)
        .await?;
        Ok(result)
    }
}

pub type AppSchema = Schema<SiteQueryRoot, SiteMutationRoot, EmptySubscription>;
