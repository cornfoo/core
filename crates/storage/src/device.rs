use chrono::NaiveDateTime;
use primitives::{Device, PlatformStore};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use tokio_postgres::{types::ToSql, Row};

use crate::{builder::InsertBuilder, error::DbPoolError, pool::PoolBuilder, QueryResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceRow {
    pub id: i32,
    pub device_id: String,
    pub platform_store: Option<String>,
    pub token: String,
    pub locale: String,
    pub currency: String,
    pub is_push_enabled: bool,
    pub is_price_alerts_enabled: bool,
    pub version: String,
    pub subscriptions_version: i32,
    pub os: Option<String>,
    pub model: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDeviceRow {
    pub device_id: String,
    pub platform_store: Option<String>,
    pub token: String,
    pub locale: String,
    pub currency: String,
    pub is_push_enabled: bool,
    pub is_price_alerts_enabled: bool,
    pub version: String,
    pub subscriptions_version: i32,
    pub os: Option<String>,
    pub model: Option<String>,
}

impl DeviceRow {
    pub fn from_row(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            device_id: row.get("device_id"),
            platform_store: row.try_get("platform_store").ok(),
            token: row.get("token"),
            locale: row.get("locale"),
            currency: row.get("currency"),
            is_push_enabled: row.get("is_push_enabled"),
            is_price_alerts_enabled: row.get("is_price_alerts_enabled"),
            version: row.get("version"),
            subscriptions_version: row.get("subscriptions_version"),
            os: row.try_get("os").ok(),
            model: row.try_get("model").ok(),
            created_at: row.get("created_at"),
        }
    }

    pub fn as_primitive(&self) -> Device {
        let platform_store =
            PlatformStore::from_str(self.platform_store.clone().unwrap_or_default().as_str()).ok();

        Device {
            id: self.device_id.clone(),
            os: self.os.clone(),
            model: self.model.clone(),
            platform_store,
            token: self.token.clone(),
            locale: self.locale.clone(),
            currency: self.currency.clone(),
            is_push_enabled: self.is_push_enabled,
            is_price_alerts_enabled: Some(self.is_price_alerts_enabled),
            version: self.version.clone(),
            subscriptions_version: self.subscriptions_version,
        }
    }
}

impl UpdateDeviceRow {
    pub fn from_primitive(device: Device) -> Self {
        Self {
            device_id: device.id,
            os: device.os,
            model: device.model,
            platform_store: device.platform_store.map(|x| x.as_ref().to_string()),
            token: device.token,
            locale: device.locale,
            currency: device.currency,
            is_push_enabled: device.is_push_enabled,
            is_price_alerts_enabled: device.is_price_alerts_enabled.unwrap_or(false),
            version: device.version,
            subscriptions_version: device.subscriptions_version,
        }
    }
}

pub struct DeviceClient {
    pool: PoolBuilder,
}

impl DeviceClient {
    pub fn new(pool: &PoolBuilder) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn add_device(&self, device: UpdateDeviceRow) -> QueryResult<DeviceRow> {
        let columns = vec![
            "device_id",
            "platform_store",
            "token",
            "locale",
            "currency",
            "is_push_enabled",
            "is_price_alerts_enabled",
            "version",
            "subscriptions_version",
            "os",
            "model",
        ];

        let query = InsertBuilder::new("devices")
            .columns(&columns)
            .on_conflict_update(
                "device_id",
                &[
                    "token",
                    "locale",
                    "currency",
                    "is_push_enabled",
                    "is_price_alerts_enabled",
                    "version",
                    "subscriptions_version",
                    "os",
                    "model",
                    "platform_store",
                ],
            )
            .returning("id, device_id, platform_store, token, locale, currency, is_push_enabled, is_price_alerts_enabled, version, subscriptions_version, os, model, created_at")
            .build();

        let params: Vec<&(dyn ToSql + Sync)> = vec![
            &device.device_id,
            &device.platform_store,
            &device.token,
            &device.locale,
            &device.currency,
            &device.is_push_enabled,
            &device.is_price_alerts_enabled,
            &device.version,
            &device.subscriptions_version,
            &device.os,
            &device.model,
        ];

        let client = self
            .pool
            .inner
            .get()
            .await
            .map_err(DbPoolError::GetConnectionError)?;

        let stmt = client
            .prepare(&query)
            .await
            .map_err(DbPoolError::QueryError)?;

        let row = client
            .query_one(&stmt, &params)
            .await
            .map_err(DbPoolError::QueryError)?;

        Ok(DeviceRow::from_row(&row))
    }
}
