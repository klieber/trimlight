use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ScheduleTime {
    #[serde(default)]
    pub hours: i32,
    #[serde(default)]
    pub minutes: i32,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ScheduleDate {
    #[serde(default)]
    pub month: i32,
    #[serde(default)]
    pub day: i32,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DailySchedule {
    #[serde(default)]
    pub id: i32,
    #[serde(default)]
    pub enable: bool,
    #[serde(default)]
    #[serde(rename = "effectId")]
    pub effect_id: i32,
    #[serde(default)]
    pub repetition: i32,
    #[serde(default)]
    #[serde(rename = "startTime")]
    pub start_time: ScheduleTime,
    #[serde(default)]
    #[serde(rename = "endTime")]
    pub end_time: ScheduleTime,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CalendarSchedule {
    #[serde(default)]
    pub id: i32,
    #[serde(default)]
    #[serde(rename = "effectId")]
    pub effect_id: i32,
    #[serde(default)]
    #[serde(rename = "startDate")]
    pub start_date: ScheduleDate,
    #[serde(default)]
    #[serde(rename = "endDate")]
    pub end_date: ScheduleDate,
    #[serde(default)]
    #[serde(rename = "startTime")]
    pub start_time: ScheduleTime,
    #[serde(default)]
    #[serde(rename = "endTime")]
    pub end_time: ScheduleTime,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeviceSchedules {
    #[serde(default)]
    pub daily: Vec<DailySchedule>,
    #[serde(default)]
    pub calendar: Vec<CalendarSchedule>,
}
