use super::effect::Effect;
use super::schedule::{CalendarSchedule, DailySchedule};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Device {
    #[serde(default)]
    #[serde(rename = "deviceId")]
    pub device_id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "switchState")]
    pub switch_state: i32,
    #[serde(default)]
    pub connectivity: i32,
    #[serde(default)]
    pub state: i32,
    #[serde(default)]
    #[serde(rename = "fwVersionName")]
    pub fw_version_name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeviceListResponse {
    #[serde(default)]
    pub total: i32,
    #[serde(default)]
    pub current: i32,
    #[serde(default)]
    pub data: Vec<Device>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Port {
    #[serde(default)]
    pub id: i32,
    #[serde(default)]
    pub start: i32,
    #[serde(default)]
    pub end: i32,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeviceDateTime {
    #[serde(default)]
    pub year: i32,
    #[serde(default)]
    pub month: i32,
    #[serde(default)]
    pub day: i32,
    #[serde(default)]
    pub weekday: i32,
    #[serde(default)]
    pub hours: i32,
    #[serde(default)]
    pub minutes: i32,
    #[serde(default)]
    pub seconds: i32,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeviceDetails {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "switchState")]
    pub switch_state: i32,
    #[serde(default)]
    pub connectivity: i32,
    #[serde(default)]
    pub state: i32,
    #[serde(default)]
    #[serde(rename = "colorOrder")]
    pub color_order: i32,
    #[serde(default)]
    pub ic: i32,
    #[serde(default)]
    pub ports: Vec<Port>,
    #[serde(default)]
    #[serde(rename = "fwVersionName")]
    pub fw_version_name: String,
    #[serde(default)]
    pub effects: Vec<Effect>,
    #[serde(default)]
    #[serde(rename = "combinedEffect")]
    pub combined_effect: Option<super::effect::CombinedEffect>,
    #[serde(default)]
    pub daily: Vec<DailySchedule>,
    #[serde(default)]
    pub calendar: Vec<CalendarSchedule>,
    #[serde(default)]
    #[serde(rename = "currentEffect")]
    pub current_effect: Option<Effect>,
    #[serde(default)]
    #[serde(rename = "overlayEffects")]
    pub overlay_effects: Vec<super::effect::OverlayEffect>,
    #[serde(default)]
    #[serde(rename = "currentDatetime")]
    pub current_datetime: DeviceDateTime,
}
