use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use hmac::{Hmac, Mac};
use reqwest::{Client as ReqwestClient, header::{HeaderMap, HeaderValue}};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

const API_BASE_URL: &str = "https://trimlight.ledhue.com/trimlight";

#[derive(Error, Debug)]
pub enum TrimlightError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Authentication error: {0}")]
    AuthError(String),
    #[error("API error: {code} - {message}")]
    ApiError { code: i32, message: String },
}

#[derive(Debug, Clone)]
pub struct TrimlightClient {
    client: ReqwestClient,
    client_id: String,
    client_secret: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ApiResponse<T> {
    code: i32,
    desc: String,
    #[serde(default)]
    payload: Option<T>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BasicResponse {
    #[serde(default)]
    pub code: i32,
    #[serde(default)]
    pub desc: String,
}

// Device List Types
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

// Device Detail Types
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
pub struct Effect {
    #[serde(default)]
    pub id: i32,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub category: i32,
    #[serde(default)]
    pub mode: i32,
    #[serde(default)]
    pub speed: i32,
    #[serde(default)]
    pub brightness: i32,
    #[serde(default)]
    pub pixel_len: Option<i32>,
    #[serde(default)]
    pub reverse: Option<bool>,
    #[serde(default)]
    pub pixels: Option<Vec<Pixel>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Pixel {
    #[serde(default)]
    pub index: i32,
    #[serde(default)]
    pub count: i32,
    #[serde(default)]
    pub color: i32,
    #[serde(default)]
    pub disable: bool,
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

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CombinedEffect {
    #[serde(default)]
    #[serde(rename = "effectIds")]
    pub effect_ids: Vec<i32>,
    #[serde(default)]
    pub interval: i32,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OverlayEffect {
    #[serde(default)]
    #[serde(rename = "overlayType")]
    pub overlay_type: i32,
    #[serde(default)]
    #[serde(rename = "targetEffect")]
    pub target_effect: i32,
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
    pub combined_effect: Option<CombinedEffect>,
    #[serde(default)]
    pub daily: Vec<DailySchedule>,
    #[serde(default)]
    pub calendar: Vec<CalendarSchedule>,
    #[serde(default)]
    #[serde(rename = "currentEffect")]
    pub current_effect: Option<Effect>,
    #[serde(default)]
    #[serde(rename = "overlayEffects")]
    pub overlay_effects: Vec<OverlayEffect>,
    #[serde(default)]
    #[serde(rename = "currentDatetime")]
    pub current_datetime: DeviceDateTime,
}

impl TrimlightClient {
    pub fn new(client_id: impl Into<String>, client_secret: impl Into<String>) -> Self {
        Self {
            client: ReqwestClient::new(),
            client_id: client_id.into(),
            client_secret: client_secret.into(),
        }
    }

    fn generate_auth_headers(&self) -> HeaderMap {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string();

        let auth_string = format!("Trimlight|{}|{}", self.client_id, timestamp);

        let mut mac = Hmac::<Sha256>::new_from_slice(self.client_secret.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(auth_string.as_bytes());
        let access_token = BASE64.encode(mac.finalize().into_bytes());

        let mut headers = HeaderMap::new();
        headers.insert("authorization", HeaderValue::from_str(&access_token).unwrap());
        headers.insert("S-ClientId", HeaderValue::from_str(&self.client_id).unwrap());
        headers.insert("S-Timestamp", HeaderValue::from_str(&timestamp).unwrap());

        headers
    }

    async fn request<T, U>(&self, method: reqwest::Method, endpoint: &str, body: Option<&T>) -> Result<U, TrimlightError>
    where
        T: Serialize + ?Sized,
        U: for<'de> Deserialize<'de> + Default,
    {
        let url = format!("{}{}", API_BASE_URL, endpoint);
        let mut req = self.client.request(method, &url);

        // Add authentication headers
        for (key, value) in self.generate_auth_headers() {
            req = req.header(key.unwrap(), value);
        }

        if let Some(body) = body {
            req = req.json(body);
        }

        let response = req.send().await?;
        let api_response: ApiResponse<U> = response.json().await?;

        if api_response.code != 0 {
            return Err(TrimlightError::ApiError {
                code: api_response.code,
                message: api_response.desc,
            });
        }

        // For operations that don't return a payload (like updates), return the default value
        Ok(api_response.payload.unwrap_or_default())
    }

    // Get device list
    pub async fn get_device_list(&self, page: Option<i32>) -> Result<DeviceListResponse, TrimlightError> {
        let body = serde_json::json!({
            "page": page
        });

        self.request(
            reqwest::Method::GET,
            "/v1/oauth/resources/devices",
            Some(&body),
        ).await
    }

    // Set device switch state
    pub async fn set_device_switch_state(&self, device_id: &str, switch_state: i32) -> Result<BasicResponse, TrimlightError> {
        let body = serde_json::json!({
            "deviceId": device_id,
            "payload": {
                "switchState": switch_state
            }
        });

        let url = format!("{}{}", API_BASE_URL, "/v1/oauth/resources/device/update");
        let mut req = self.client.request(reqwest::Method::POST, &url);

        // Add authentication headers
        for (key, value) in self.generate_auth_headers() {
            req = req.header(key.unwrap(), value);
        }

        req = req.json(&body);

        let response = req.send().await?;
        let api_response = response.json().await?;

        Ok(api_response)
    }

    // Set device name
    pub async fn set_device_name(&self, device_id: &str, name: &str) -> Result<BasicResponse, TrimlightError> {
        let body = serde_json::json!({
            "deviceId": device_id,
            "payload": {
                "name": name
            }
        });

        let url = format!("{}{}", API_BASE_URL, "/v1/oauth/resources/device/update");
        let mut req = self.client.request(reqwest::Method::POST, &url);

        // Add authentication headers
        for (key, value) in self.generate_auth_headers() {
            req = req.header(key.unwrap(), value);
        }

        req = req.json(&body);

        let response = req.send().await?;
        let api_response = response.json().await?;

        Ok(api_response)
    }

    // Preview build-in effect
    pub async fn preview_builtin_effect(
        &self,
        device_id: &str,
        mode: i32,
        speed: i32,
        brightness: i32,
        pixel_len: i32,
        reverse: bool,
    ) -> Result<BasicResponse, TrimlightError> {
        let body = serde_json::json!({
            "deviceId": device_id,
            "payload": {
                "category": 1,  // 1 for built-in effects
                "mode": mode,
                "speed": speed,
                "brightness": brightness,
                "pixelLen": pixel_len,
                "reverse": reverse
            }
        });

        let url = format!("{}{}", API_BASE_URL, "/v1/oauth/resources/device/effect/preview");
        let mut req = self.client.request(reqwest::Method::POST, &url);

        // Add authentication headers
        for (key, value) in self.generate_auth_headers() {
            req = req.header(key.unwrap(), value);
        }

        req = req.json(&body);

        let response = req.send().await?;
        let api_response = response.json().await?;

        Ok(api_response)
    }

    /// Get detailed information about a device
    pub async fn get_device_details(&self, device_id: &str) -> Result<DeviceDetails, TrimlightError> {
        let body = serde_json::json!({
            "deviceId": device_id,
            "currentDate": {
                "year": 24,  // 2024
                "month": 1,
                "day": 1,
                "weekday": 1,
                "hours": 1,
                "minutes": 1,
                "seconds": 1
            }
        });

        self.request(
            reqwest::Method::POST,
            "/v1/oauth/resources/device/get",
            Some(&body),
        ).await
    }

    /// Get device schedules
    pub async fn get_device_schedules(&self, device_id: &str) -> Result<DeviceSchedules, TrimlightError> {
        let details = self.get_device_details(device_id).await?;
        Ok(DeviceSchedules {
            daily: details.daily,
            calendar: details.calendar,
        })
    }

    /// Add a daily schedule
    pub async fn add_daily_schedule(
        &self,
        device_id: &str,
        effect_id: i32,
        start: String,
        end: String,
        repetition: i32,
    ) -> Result<BasicResponse, TrimlightError> {
        // Parse start time
        let (start_hours, start_minutes) = parse_time(&start)?;
        // Parse end time
        let (end_hours, end_minutes) = parse_time(&end)?;

        let body = serde_json::json!({
            "deviceId": device_id,
            "payload": {
                "id": -1,  // Server will assign the actual ID
                "enable": true,
                "effectId": effect_id,
                "repetition": repetition,
                "startTime": {
                    "hours": start_hours,
                    "minutes": start_minutes
                },
                "endTime": {
                    "hours": end_hours,
                    "minutes": end_minutes
                }
            }
        });

        self.request(
            reqwest::Method::POST,
            "/v1/oauth/resources/device/schedule/daily/add",
            Some(&body),
        ).await
    }

    /// Add a calendar schedule
    pub async fn add_calendar_schedule(
        &self,
        device_id: &str,
        effect_id: i32,
        start_date: String,
        end_date: String,
        start_time: String,
        end_time: String,
    ) -> Result<BasicResponse, TrimlightError> {
        // Parse dates
        let (start_month, start_day) = parse_date(&start_date)?;
        let (end_month, end_day) = parse_date(&end_date)?;
        // Parse times
        let (start_hours, start_minutes) = parse_time(&start_time)?;
        let (end_hours, end_minutes) = parse_time(&end_time)?;

        let body = serde_json::json!({
            "deviceId": device_id,
            "payload": {
                "id": 0,
                "effectId": effect_id,
                "startDate": {
                    "month": start_month,
                    "day": start_day
                },
                "endDate": {
                    "month": end_month,
                    "day": end_day
                },
                "startTime": {
                    "hours": start_hours,
                    "minutes": start_minutes
                },
                "endTime": {
                    "hours": end_hours,
                    "minutes": end_minutes
                }
            }
        });

        self.request(
            reqwest::Method::POST,
            "/v1/oauth/resources/device/schedule/calendar/add",
            Some(&body),
        ).await
    }

    /// Delete a schedule
    pub async fn delete_schedule(
        &self,
        device_id: &str,
        schedule_id: i32,
        schedule_type: &str,
    ) -> Result<BasicResponse, TrimlightError> {
        let endpoint = match schedule_type.to_lowercase().as_str() {
            "daily" => "/v1/oauth/resources/device/schedule/daily/delete",
            "calendar" => "/v1/oauth/resources/device/schedule/calendar/delete",
            _ => return Err(TrimlightError::ApiError {
                code: 400,
                message: "Invalid schedule type. Must be 'daily' or 'calendar'".to_string(),
            }),
        };

        let body = serde_json::json!({
            "deviceId": device_id,
            "payload": {
                "id": schedule_id
            }
        });

        self.request(reqwest::Method::POST, endpoint, Some(&body)).await
    }

    /// Toggle a daily schedule on/off
    pub async fn toggle_schedule(
        &self,
        device_id: &str,
        schedule_id: i32,
        enable: bool,
    ) -> Result<BasicResponse, TrimlightError> {
        let body = serde_json::json!({
            "deviceId": device_id,
            "payload": {
                "id": schedule_id,
                "enable": enable
            }
        });

        self.request(
            reqwest::Method::POST,
            "/v1/oauth/resources/device/schedule/daily/update",
            Some(&body),
        ).await
    }

    /// Modify an existing schedule
    pub async fn modify_schedule(
        &self,
        device_id: &str,
        schedule_id: i32,
        schedule_type: &str,
        effect_id: Option<i32>,
        start: Option<String>,
        end: String,
        repetition: Option<i32>,
    ) -> Result<BasicResponse, TrimlightError> {
        let endpoint = match schedule_type.to_lowercase().as_str() {
            "daily" => "/v1/oauth/resources/device/schedule/daily/update",
            "calendar" => "/v1/oauth/resources/device/schedule/calendar/update",
            _ => return Err(TrimlightError::ApiError {
                code: 400,
                message: "Invalid schedule type. Must be 'daily' or 'calendar'".to_string(),
            }),
        };

        // Get current schedule details
        let schedules = self.get_device_schedules(device_id).await?;
        let schedule = if schedule_type == "daily" {
            schedules.daily.iter().find(|s| s.id == schedule_id)
        } else {
            None
        };

        let schedule = schedule.ok_or_else(|| TrimlightError::ApiError {
            code: 404,
            message: format!("Schedule {} not found", schedule_id),
        })?;

        // Parse times if provided
        let (start_hours, start_minutes) = if let Some(start_time) = start {
            parse_time(&start_time)?
        } else {
            (schedule.start_time.hours, schedule.start_time.minutes)
        };
        let (end_hours, end_minutes) = parse_time(&end)?;

        let mut payload = serde_json::json!({
            "id": schedule_id,
            "enable": schedule.enable,
            "effectId": effect_id.unwrap_or(schedule.effect_id),
            "startTime": {
                "hours": start_hours,
                "minutes": start_minutes
            },
            "endTime": {
                "hours": end_hours,
                "minutes": end_minutes
            }
        });

        if schedule_type == "daily" {
            if let Some(rep) = repetition {
                payload["repetition"] = serde_json::json!(rep);
            } else {
                payload["repetition"] = serde_json::json!(schedule.repetition);
            }
        }

        let body = serde_json::json!({
            "deviceId": device_id,
            "payload": payload
        });

        self.request(reqwest::Method::POST, endpoint, Some(&body)).await
    }

    /// Check for schedule conflicts
    pub async fn check_schedule_conflicts(&self, device_id: &str) -> Result<BasicResponse, TrimlightError> {
        let schedules = self.get_device_schedules(device_id).await?;
        let mut conflicts = Vec::new();

        // Check daily schedule conflicts
        for (i, schedule1) in schedules.daily.iter().enumerate() {
            if !schedule1.enable {
                continue;
            }

            for schedule2 in schedules.daily.iter().skip(i + 1) {
                if !schedule2.enable {
                    continue;
                }

                // Check if schedules have overlapping repetition patterns
                let overlapping_days = match (schedule1.repetition, schedule2.repetition) {
                    (0, _) | (_, 0) => false, // Today only doesn't conflict
                    (1, _) | (_, 1) => true,  // Everyday conflicts with everything
                    (2, 2) => true,  // Weekdays overlap with weekdays
                    (3, 3) => true,  // Weekend overlaps with weekend
                    (2, 3) | (3, 2) => false, // Weekdays don't overlap with weekend
                    _ => false,
                };

                if overlapping_days {
                    // Check time overlap
                    let start1 = schedule1.start_time.hours * 60 + schedule1.start_time.minutes;
                    let end1 = schedule1.end_time.hours * 60 + schedule1.end_time.minutes;
                    let start2 = schedule2.start_time.hours * 60 + schedule2.start_time.minutes;
                    let end2 = schedule2.end_time.hours * 60 + schedule2.end_time.minutes;

                    if (start1 <= end2 && end1 >= start2) || (start2 <= end1 && end2 >= start1) {
                        conflicts.push(format!(
                            "Daily schedules {} and {} have overlapping times",
                            schedule1.id,
                            schedule2.id
                        ));
                    }
                }
            }
        }

        // Check calendar schedule conflicts
        for (i, schedule1) in schedules.calendar.iter().enumerate() {
            for schedule2 in schedules.calendar.iter().skip(i + 1) {
                // Check date overlap
                let start1 = schedule1.start_date.month * 31 + schedule1.start_date.day;
                let end1 = schedule1.end_date.month * 31 + schedule1.end_date.day;
                let start2 = schedule2.start_date.month * 31 + schedule2.start_date.day;
                let end2 = schedule2.end_date.month * 31 + schedule2.end_date.day;

                if (start1 <= end2 && end1 >= start2) || (start2 <= end1 && end2 >= start1) {
                    // Check time overlap
                    let time_start1 = schedule1.start_time.hours * 60 + schedule1.start_time.minutes;
                    let time_end1 = schedule1.end_time.hours * 60 + schedule1.end_time.minutes;
                    let time_start2 = schedule2.start_time.hours * 60 + schedule2.start_time.minutes;
                    let time_end2 = schedule2.end_time.hours * 60 + schedule2.end_time.minutes;

                    if (time_start1 <= time_end2 && time_end1 >= time_start2) ||
                       (time_start2 <= time_end1 && time_end2 >= time_start1) {
                        conflicts.push(format!(
                            "Calendar schedules {} and {} have overlapping dates and times",
                            schedule1.id,
                            schedule2.id
                        ));
                    }
                }
            }
        }

        if conflicts.is_empty() {
            Ok(BasicResponse {
                code: 0,
                desc: "No conflicts found".to_string(),
            })
        } else {
            Ok(BasicResponse {
                code: 1,
                desc: conflicts.join("\n"),
            })
        }
    }

    /// Set a combined effect sequence
    pub async fn set_combined_effect(
        &self,
        device_id: &str,
        effect_ids: &[i32],
        interval: i32,
    ) -> Result<BasicResponse, TrimlightError> {
        let body = serde_json::json!({
            "deviceId": device_id,
            "payload": {
                "effectIds": effect_ids,
                "interval": interval
            }
        });

        self.request(
            reqwest::Method::POST,
            "/v1/oauth/resources/device/effect/combined/set",
            Some(&body),
        ).await
    }

    /// Clear the combined effect sequence
    pub async fn clear_combined_effect(
        &self,
        device_id: &str,
    ) -> Result<BasicResponse, TrimlightError> {
        let body = serde_json::json!({
            "deviceId": device_id,
            "payload": {
                "effectIds": [],
                "interval": 0
            }
        });

        self.request(
            reqwest::Method::POST,
            "/v1/oauth/resources/device/effect/combined/set",
            Some(&body),
        ).await
    }
}

/// Parse time string in HH:MM format
fn parse_time(time: &str) -> Result<(i32, i32), TrimlightError> {
    let parts: Vec<&str> = time.split(':').collect();
    if parts.len() != 2 {
        return Err(TrimlightError::ApiError {
            code: 400,
            message: "Invalid time format. Use HH:MM".to_string(),
        });
    }

    let hours = parts[0].parse::<i32>().map_err(|_| TrimlightError::ApiError {
        code: 400,
        message: "Invalid hours".to_string(),
    })?;

    let minutes = parts[1].parse::<i32>().map_err(|_| TrimlightError::ApiError {
        code: 400,
        message: "Invalid minutes".to_string(),
    })?;

    if hours < 0 || hours > 23 || minutes < 0 || minutes > 59 {
        return Err(TrimlightError::ApiError {
            code: 400,
            message: "Invalid time values".to_string(),
        });
    }

    Ok((hours, minutes))
}

/// Parse date string in MM-DD format
fn parse_date(date: &str) -> Result<(i32, i32), TrimlightError> {
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() != 2 {
        return Err(TrimlightError::ApiError {
            code: 400,
            message: "Invalid date format. Use MM-DD".to_string(),
        });
    }

    let month = parts[0].parse::<i32>().map_err(|_| TrimlightError::ApiError {
        code: 400,
        message: "Invalid month".to_string(),
    })?;

    let day = parts[1].parse::<i32>().map_err(|_| TrimlightError::ApiError {
        code: 400,
        message: "Invalid day".to_string(),
    })?;

    if month < 1 || month > 12 || day < 1 || day > 31 {
        return Err(TrimlightError::ApiError {
            code: 400,
            message: "Invalid date values".to_string(),
        });
    }

    Ok((month, day))
}
