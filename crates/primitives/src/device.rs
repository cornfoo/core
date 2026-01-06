use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter, EnumString};
use typeshare::typeshare;

#[derive(
    Copy, Clone, Debug, Serialize, Deserialize, EnumIter, AsRefStr, EnumString, PartialEq, Eq, Hash,
)]
#[typeshare(swift = "Equatable, CaseIterable, Sendable")]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "camelCase")]
pub enum PlatformStore {
    AppStore,
    Local,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[typeshare(swift = "Equatable, Sendable")]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub id: String,
    pub os: Option<String>,
    pub model: Option<String>,
    pub platform_store: Option<PlatformStore>,
    pub token: String,
    pub locale: String,
    pub version: String,
    pub currency: String,
    pub is_push_enabled: bool,
    pub is_price_alerts_enabled: Option<bool>,
    pub subscriptions_version: i32,
}
