use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use hmac::{Hmac, Mac};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client as ReqwestClient, Method,
};
use serde::Serialize;
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::TrimlightError;
use crate::models::*;
use crate::utils;

const DEFAULT_API_BASE_URL: &str = "https://trimlight.ledhue.com/trimlight";

#[derive(Debug, Clone)]
pub struct TrimlightClient {
    client: ReqwestClient,
    client_id: String,
    client_secret: String,
    api_base_url: String,
}

impl TrimlightClient {
    pub fn new(client_id: impl Into<String>, client_secret: impl Into<String>) -> Self {
        Self {
            client: ReqwestClient::new(),
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            api_base_url: DEFAULT_API_BASE_URL.to_string(),
        }
    }

    #[cfg(test)]
    pub fn with_base_url(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        api_base_url: impl Into<String>,
    ) -> Self {
        Self {
            client: ReqwestClient::new(),
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            api_base_url: api_base_url.into(),
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
        headers.insert(
            "authorization",
            HeaderValue::from_str(&access_token).unwrap(),
        );
        headers.insert(
            "S-ClientId",
            HeaderValue::from_str(&self.client_id).unwrap(),
        );
        headers.insert("S-Timestamp", HeaderValue::from_str(&timestamp).unwrap());

        headers
    }

    async fn request<T, U>(
        &self,
        method: Method,
        endpoint: &str,
        body: Option<&T>,
    ) -> Result<U, TrimlightError>
    where
        T: Serialize + ?Sized,
        U: for<'de> serde::de::Deserialize<'de> + Default,
    {
        let url = format!("{}{}", self.api_base_url, endpoint);
        let mut req = self.client.request(method, &url);

        // Add authentication headers
        for (key, value) in self.generate_auth_headers() {
            req = req.header(key.unwrap(), value);
        }

        if let Some(body) = body {
            req = req.json(body);
        }

        let response = req.send().await?;
        let api_response: crate::models::ApiResponse<U> = response.json().await?;

        if api_response.code != 0 {
            return Err(TrimlightError::ApiError {
                code: api_response.code,
                message: api_response.desc,
            });
        }

        Ok(api_response.payload.unwrap_or_default())
    }

    // Device Management Methods
    pub async fn get_device_list(
        &self,
        page: Option<i32>,
    ) -> Result<DeviceListResponse, TrimlightError> {
        let body = serde_json::json!({
            "page": page
        });

        self.request(Method::GET, "/v1/oauth/resources/devices", Some(&body))
            .await
    }

    pub async fn get_device_details(
        &self,
        device_id: &str,
    ) -> Result<DeviceDetails, TrimlightError> {
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

        self.request(Method::POST, "/v1/oauth/resources/device/get", Some(&body))
            .await
    }

    pub async fn set_device_switch_state(
        &self,
        device_id: &str,
        switch_state: i32,
    ) -> Result<BasicResponse, TrimlightError> {
        let body = serde_json::json!({
            "deviceId": device_id,
            "payload": {
                "switchState": switch_state
            }
        });

        self.request(
            Method::POST,
            "/v1/oauth/resources/device/update",
            Some(&body),
        )
        .await
    }

    pub async fn set_device_name(
        &self,
        device_id: &str,
        name: &str,
    ) -> Result<BasicResponse, TrimlightError> {
        let body = serde_json::json!({
            "deviceId": device_id,
            "payload": {
                "name": name
            }
        });

        self.request(
            Method::POST,
            "/v1/oauth/resources/device/update",
            Some(&body),
        )
        .await
    }

    // Effect Management Methods
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
                "category": 0,  // 0 for built-in effects
                "mode": mode,
                "speed": speed,
                "brightness": brightness,
                "pixelLen": pixel_len,
                "reverse": reverse
            }
        });

        self.request(
            Method::POST,
            "/v1/oauth/resources/device/effect/preview",
            Some(&body),
        )
        .await
    }

    pub async fn preview_custom_effect(
        &self,
        device_id: &str,
        mode: i32,
        speed: i32,
        brightness: i32,
        pixels: Option<Vec<Pixel>>,
    ) -> Result<BasicResponse, TrimlightError> {
        let body = serde_json::json!({
            "deviceId": device_id,
            "payload": {
                "category": 1,  // 1 for custom effects
                "mode": mode,
                "speed": speed,
                "brightness": brightness,
                "pixels": pixels
            }
        });

        self.request(
            Method::POST,
            "/v1/oauth/resources/device/effect/preview",
            Some(&body),
        )
        .await
    }

    pub async fn add_effect(
        &self,
        device_id: &str,
        name: &str,
        mode: i32,
        speed: i32,
        brightness: i32,
        pixel_len: Option<i32>,
        reverse: Option<bool>,
        pixels: Option<Vec<Pixel>>,
    ) -> Result<BasicResponse, TrimlightError> {
        let body = serde_json::json!({
            "deviceId": device_id,
            "payload": {
                "name": name,
                "category": 2,  // 2 for custom effects
                "mode": mode,
                "speed": speed,
                "brightness": brightness,
                "pixelLen": pixel_len,
                "reverse": reverse,
                "pixels": pixels
            }
        });

        self.request(
            Method::POST,
            "/v1/oauth/resources/device/effect/add",
            Some(&body),
        )
        .await
    }

    pub async fn update_effect(
        &self,
        device_id: &str,
        effect_id: i32,
        name: Option<&str>,
        mode: Option<i32>,
        speed: Option<i32>,
        brightness: Option<i32>,
        pixel_len: Option<i32>,
        reverse: Option<bool>,
        pixels: Option<Vec<Pixel>>,
    ) -> Result<BasicResponse, TrimlightError> {
        let details = self.get_device_details(device_id).await?;
        let current_effect = details
            .effects
            .iter()
            .find(|e| e.id == effect_id)
            .ok_or_else(|| TrimlightError::ApiError {
                code: 404,
                message: format!("Effect {} not found", effect_id),
            })?;

        let body = serde_json::json!({
            "deviceId": device_id,
            "payload": {
                "id": effect_id,
                "name": name.unwrap_or(&current_effect.name),
                "category": current_effect.category,
                "mode": mode.unwrap_or(current_effect.mode),
                "speed": speed.unwrap_or(current_effect.speed),
                "brightness": brightness.unwrap_or(current_effect.brightness),
                "pixelLen": pixel_len.or(current_effect.pixel_len),
                "reverse": reverse.or(current_effect.reverse),
                "pixels": pixels.or_else(|| current_effect.pixels.clone())
            }
        });

        self.request(
            Method::POST,
            "/v1/oauth/resources/device/effect/update",
            Some(&body),
        )
        .await
    }

    pub async fn delete_effect(
        &self,
        device_id: &str,
        effect_id: i32,
    ) -> Result<BasicResponse, TrimlightError> {
        let body = serde_json::json!({
            "deviceId": device_id,
            "payload": {
                "id": effect_id
            }
        });

        self.request(
            Method::POST,
            "/v1/oauth/resources/device/effect/delete",
            Some(&body),
        )
        .await
    }

    pub async fn view_effect(
        &self,
        device_id: &str,
        effect_id: i32,
    ) -> Result<BasicResponse, TrimlightError> {
        let details = self.get_device_details(device_id).await?;
        let effect = details
            .effects
            .iter()
            .find(|e| e.id == effect_id)
            .ok_or_else(|| TrimlightError::ApiError {
                code: 404,
                message: format!("Effect {} not found", effect_id),
            })?;

        let body = serde_json::json!({
            "deviceId": device_id,
            "payload": {
                "category": effect.category,
                "mode": effect.mode,
                "speed": effect.speed,
                "brightness": effect.brightness,
                "pixelLen": effect.pixel_len,
                "reverse": effect.reverse,
                "pixels": effect.pixels
            }
        });

        self.request(
            Method::POST,
            "/v1/oauth/resources/device/effect/preview",
            Some(&body),
        )
        .await
    }

    // Schedule Management Methods
    pub async fn get_device_schedules(
        &self,
        device_id: &str,
    ) -> Result<DeviceSchedules, TrimlightError> {
        let details = self.get_device_details(device_id).await?;
        Ok(DeviceSchedules {
            daily: details.daily,
            calendar: details.calendar,
        })
    }

    pub async fn add_daily_schedule(
        &self,
        device_id: &str,
        effect_id: i32,
        start: String,
        end: String,
        repetition: i32,
    ) -> Result<BasicResponse, TrimlightError> {
        let (start_hours, start_minutes) = utils::parse_time(&start)?;
        let (end_hours, end_minutes) = utils::parse_time(&end)?;

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
            Method::POST,
            "/v1/oauth/resources/device/schedule/daily/add",
            Some(&body),
        )
        .await
    }

    pub async fn add_calendar_schedule(
        &self,
        device_id: &str,
        effect_id: i32,
        start_date: String,
        end_date: String,
        start_time: String,
        end_time: String,
    ) -> Result<BasicResponse, TrimlightError> {
        let (start_month, start_day) = utils::parse_date(&start_date)?;
        let (end_month, end_day) = utils::parse_date(&end_date)?;
        let (start_hours, start_minutes) = utils::parse_time(&start_time)?;
        let (end_hours, end_minutes) = utils::parse_time(&end_time)?;

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
            Method::POST,
            "/v1/oauth/resources/device/schedule/calendar/add",
            Some(&body),
        )
        .await
    }

    pub async fn delete_schedule(
        &self,
        device_id: &str,
        schedule_id: i32,
        schedule_type: &str,
    ) -> Result<BasicResponse, TrimlightError> {
        let endpoint = match schedule_type.to_lowercase().as_str() {
            "daily" => "/v1/oauth/resources/device/schedule/daily/delete",
            "calendar" => "/v1/oauth/resources/device/schedule/calendar/delete",
            _ => {
                return Err(TrimlightError::ApiError {
                    code: 400,
                    message: "Invalid schedule type. Must be 'daily' or 'calendar'".to_string(),
                })
            }
        };

        let body = serde_json::json!({
            "deviceId": device_id,
            "payload": {
                "id": schedule_id
            }
        });

        self.request(Method::POST, endpoint, Some(&body)).await
    }

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
            Method::POST,
            "/v1/oauth/resources/device/schedule/daily/update",
            Some(&body),
        )
        .await
    }

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
            _ => {
                return Err(TrimlightError::ApiError {
                    code: 400,
                    message: "Invalid schedule type. Must be 'daily' or 'calendar'".to_string(),
                })
            }
        };

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

        let (start_hours, start_minutes) = if let Some(start_time) = start {
            utils::parse_time(&start_time)?
        } else {
            (schedule.start_time.hours, schedule.start_time.minutes)
        };
        let (end_hours, end_minutes) = utils::parse_time(&end)?;

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

        self.request(Method::POST, endpoint, Some(&body)).await
    }

    pub async fn check_schedule_conflicts(
        &self,
        device_id: &str,
    ) -> Result<BasicResponse, TrimlightError> {
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
                    (2, 2) => true,           // Weekdays overlap with weekdays
                    (3, 3) => true,           // Weekend overlaps with weekend
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
                            schedule1.id, schedule2.id
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
                    let time_start1 =
                        schedule1.start_time.hours * 60 + schedule1.start_time.minutes;
                    let time_end1 = schedule1.end_time.hours * 60 + schedule1.end_time.minutes;
                    let time_start2 =
                        schedule2.start_time.hours * 60 + schedule2.start_time.minutes;
                    let time_end2 = schedule2.end_time.hours * 60 + schedule2.end_time.minutes;

                    if (time_start1 <= time_end2 && time_end1 >= time_start2)
                        || (time_start2 <= time_end1 && time_end2 >= time_start1)
                    {
                        conflicts.push(format!(
                            "Calendar schedules {} and {} have overlapping dates and times",
                            schedule1.id, schedule2.id
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

    // Combined Effect Methods
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
            Method::POST,
            "/v1/oauth/resources/device/effect/combined/set",
            Some(&body),
        )
        .await
    }

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
            Method::POST,
            "/v1/oauth/resources/device/effect/combined/set",
            Some(&body),
        )
        .await
    }

    // Overlay Effect Methods
    pub async fn add_overlay_effect(
        &self,
        device_id: &str,
        overlay_type: i32,
        target_effect_id: i32,
    ) -> Result<BasicResponse, TrimlightError> {
        let body = serde_json::json!({
            "deviceId": device_id,
            "payload": {
                "overlayEffects": [{
                    "overlayType": overlay_type,
                    "targetEffect": target_effect_id
                }]
            }
        });

        self.request(
            Method::POST,
            "/v1/oauth/resources/device/effect/overlay",
            Some(&body),
        )
        .await
    }

    pub async fn clear_overlay_effects(
        &self,
        device_id: &str,
    ) -> Result<BasicResponse, TrimlightError> {
        let body = serde_json::json!({
            "deviceId": device_id,
            "payload": {
                "overlayEffects": []
            }
        });

        self.request(
            Method::POST,
            "/v1/oauth/resources/device/effect/overlay",
            Some(&body),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    async fn create_test_client(server: &Server) -> TrimlightClient {
        TrimlightClient::with_base_url("test_id", "test_secret", server.url())
    }

    #[tokio::test]
    async fn test_get_device_list() {
        let mut server = Server::new_async().await;
        let mock_response = serde_json::json!({
            "code": 0,
            "desc": "Success",
            "payload": {
                "total": 1,
                "current": 1,
                "data": [{
                    "deviceId": "test123",
                    "name": "Test Device",
                    "switchState": 1,
                    "connectivity": 1,
                    "state": 1,
                    "fwVersionName": "1.0.0"
                }]
            }
        });

        let _m = server
            .mock("GET", "/v1/oauth/resources/devices")
            .match_header("authorization", mockito::Matcher::Any)
            .match_header("S-ClientId", mockito::Matcher::Any)
            .match_header("S-Timestamp", mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response.to_string())
            .create_async()
            .await;

        let client = create_test_client(&server).await;
        let result = client.get_device_list(None).await.unwrap();

        assert_eq!(result.total, 1);
        assert_eq!(result.current, 1);
        assert_eq!(result.data.len(), 1);
        assert_eq!(result.data[0].device_id, "test123");
        assert_eq!(result.data[0].name, "Test Device");
    }

    #[tokio::test]
    async fn test_set_device_switch_state() {
        let mut server = Server::new_async().await;
        let mock_response = serde_json::json!({
            "code": 0,
            "desc": "Success",
            "payload": {
                "code": 0,
                "desc": "Success"
            }
        });

        let _m = server
            .mock("POST", "/v1/oauth/resources/device/update")
            .match_header("authorization", mockito::Matcher::Any)
            .match_header("S-ClientId", mockito::Matcher::Any)
            .match_header("S-Timestamp", mockito::Matcher::Any)
            .match_body(mockito::Matcher::JsonString(
                r#"{"deviceId":"test123","payload":{"switchState":1}}"#.to_string(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response.to_string())
            .create_async()
            .await;

        let client = create_test_client(&server).await;
        let result = client.set_device_switch_state("test123", 1).await.unwrap();

        assert_eq!(result.code, 0);
        assert_eq!(result.desc, "Success");
    }

    #[tokio::test]
    async fn test_get_device_details() {
        let mut server = Server::new_async().await;
        let mock_response = serde_json::json!({
            "code": 0,
            "desc": "Success",
            "payload": {
                "name": "Test Device",
                "switchState": 1,
                "connectivity": 1,
                "state": 1,
                "colorOrder": 0,
                "ic": 1,
                "ports": [{
                    "id": 1,
                    "start": 0,
                    "end": 100
                }],
                "fwVersionName": "1.0.0",
                "effects": [],
                "daily": [],
                "calendar": [],
                "currentDatetime": {
                    "year": 24,
                    "month": 3,
                    "day": 9,
                    "weekday": 0,
                    "hours": 12,
                    "minutes": 0,
                    "seconds": 0
                }
            }
        });

        let _m = server
            .mock("POST", "/v1/oauth/resources/device/get")
            .match_header("authorization", mockito::Matcher::Any)
            .match_header("S-ClientId", mockito::Matcher::Any)
            .match_header("S-Timestamp", mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response.to_string())
            .create_async()
            .await;

        let client = create_test_client(&server).await;
        let result = client.get_device_details("test123").await.unwrap();

        assert_eq!(result.name, "Test Device");
        assert_eq!(result.switch_state, 1);
        assert_eq!(result.ports.len(), 1);
        assert_eq!(result.ports[0].id, 1);
    }

    #[tokio::test]
    async fn test_error_handling() {
        let mut server = Server::new_async().await;
        let mock_response = serde_json::json!({
            "code": 1001,
            "desc": "Device not found"
        });

        let _m = server
            .mock("GET", "/v1/oauth/resources/devices")
            .match_header("authorization", mockito::Matcher::Any)
            .match_header("S-ClientId", mockito::Matcher::Any)
            .match_header("S-Timestamp", mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response.to_string())
            .create_async()
            .await;

        let client = create_test_client(&server).await;
        let result = client.get_device_list(None).await;

        assert!(matches!(
            result,
            Err(TrimlightError::ApiError {
                code: 1001,
                message
            }) if message == "Device not found"
        ));
    }

    #[tokio::test]
    async fn test_auth_headers() {
        let mut server = Server::new_async().await;
        let mock_response = serde_json::json!({
            "code": 0,
            "desc": "Success",
            "payload": {
                "total": 0,
                "current": 0,
                "data": []
            }
        });

        let _m = server
            .mock("GET", "/v1/oauth/resources/devices")
            .match_header(
                "authorization",
                mockito::Matcher::Regex(r"^[A-Za-z0-9+/=]+$".to_string()),
            )
            .match_header("S-ClientId", "test_id")
            .match_header("S-Timestamp", mockito::Matcher::Regex(r"^\d+$".to_string()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response.to_string())
            .create_async()
            .await;

        let client = create_test_client(&server).await;
        let result = client.get_device_list(None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_preview_builtin_effect() {
        let mut server = Server::new_async().await;
        let mock_response = serde_json::json!({
            "code": 0,
            "desc": "Success",
            "payload": {
                "code": 0,
                "desc": "Success"
            }
        });

        let _m = server.mock("POST", "/v1/oauth/resources/device/effect/preview")
            .match_header("authorization", mockito::Matcher::Any)
            .match_header("S-ClientId", mockito::Matcher::Any)
            .match_header("S-Timestamp", mockito::Matcher::Any)
            .match_body(mockito::Matcher::JsonString(r#"{"deviceId":"test123","payload":{"category":1,"mode":1,"speed":5,"brightness":100,"pixelLen":50,"reverse":false}}"#.to_string()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response.to_string())
            .create_async()
            .await;

        let client = create_test_client(&server).await;
        let result = client
            .preview_builtin_effect("test123", 1, 5, 100, 50, false)
            .await
            .unwrap();

        assert_eq!(result.code, 0);
        assert_eq!(result.desc, "Success");
    }

    #[tokio::test]
    async fn test_add_effect() {
        let mut server = Server::new_async().await;
        let mock_response = serde_json::json!({
            "code": 0,
            "desc": "Success",
            "payload": {
                "code": 0,
                "desc": "Success"
            }
        });

        let _m = server.mock("POST", "/v1/oauth/resources/device/effect/add")
            .match_header("authorization", mockito::Matcher::Any)
            .match_header("S-ClientId", mockito::Matcher::Any)
            .match_header("S-Timestamp", mockito::Matcher::Any)
            .match_body(mockito::Matcher::JsonString(r#"{"deviceId":"test123","payload":{"name":"Test Effect","category":2,"mode":1,"speed":5,"brightness":100,"pixelLen":50,"reverse":true,"pixels":null}}"#.to_string()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response.to_string())
            .create_async()
            .await;

        let client = create_test_client(&server).await;
        let result = client
            .add_effect(
                "test123",
                "Test Effect",
                1,
                5,
                100,
                Some(50),
                Some(true),
                None,
            )
            .await
            .unwrap();

        assert_eq!(result.code, 0);
        assert_eq!(result.desc, "Success");
    }

    #[tokio::test]
    async fn test_update_effect() {
        let mut server = Server::new_async().await;

        // Mock for get_device_details
        let details_response = serde_json::json!({
            "code": 0,
            "desc": "Success",
            "payload": {
                "effects": [{
                    "id": 1,
                    "name": "Original Effect",
                    "category": 2,
                    "mode": 1,
                    "speed": 5,
                    "brightness": 100,
                    "pixelLen": 50,
                    "reverse": false,
                    "pixels": null
                }]
            }
        });

        let _m1 = server
            .mock("POST", "/v1/oauth/resources/device/get")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(details_response.to_string())
            .expect(1)
            .create_async()
            .await;

        // Mock for update_effect
        let update_response = serde_json::json!({
            "code": 0,
            "desc": "Success",
            "payload": {
                "code": 0,
                "desc": "Success"
            }
        });

        let _m2 = server
            .mock("POST", "/v1/oauth/resources/device/effect/update")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(update_response.to_string())
            .expect(1)
            .create_async()
            .await;

        let client = create_test_client(&server).await;
        let result = client
            .update_effect(
                "test123",
                1,
                Some("Updated Effect"),
                Some(2),
                None,
                None,
                None,
                None,
                None,
            )
            .await;

        match result {
            Ok(response) => {
                assert_eq!(response.code, 0);
                assert_eq!(response.desc, "Success");
            }
            Err(e) => panic!("Failed to update effect: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_delete_effect() {
        let mut server = Server::new_async().await;
        let mock_response = serde_json::json!({
            "code": 0,
            "desc": "Success",
            "payload": {
                "code": 0,
                "desc": "Success"
            }
        });

        let _m = server
            .mock("POST", "/v1/oauth/resources/device/effect/delete")
            .match_header("authorization", mockito::Matcher::Any)
            .match_header("S-ClientId", mockito::Matcher::Any)
            .match_header("S-Timestamp", mockito::Matcher::Any)
            .match_body(mockito::Matcher::JsonString(
                r#"{"deviceId":"test123","payload":{"id":1}}"#.to_string(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response.to_string())
            .create_async()
            .await;

        let client = create_test_client(&server).await;
        let result = client.delete_effect("test123", 1).await.unwrap();

        assert_eq!(result.code, 0);
        assert_eq!(result.desc, "Success");
    }

    #[tokio::test]
    async fn test_view_effect() {
        let mut server = Server::new_async().await;

        // Mock for get_device_details
        let details_response = serde_json::json!({
            "code": 0,
            "desc": "Success",
            "payload": {
                "effects": [{
                    "id": 1,
                    "name": "Test Effect",
                    "category": 2,
                    "mode": 1,
                    "speed": 5,
                    "brightness": 100,
                    "pixelLen": 50,
                    "reverse": false,
                    "pixels": null
                }]
            }
        });

        let _m1 = server
            .mock("POST", "/v1/oauth/resources/device/get")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(details_response.to_string())
            .expect(1)
            .create_async()
            .await;

        // Mock for view_effect
        let view_response = serde_json::json!({
            "code": 0,
            "desc": "Success",
            "payload": {
                "code": 0,
                "desc": "Success"
            }
        });

        let _m2 = server
            .mock("POST", "/v1/oauth/resources/device/effect/preview")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(view_response.to_string())
            .expect(1)
            .create_async()
            .await;

        let client = create_test_client(&server).await;
        let result = client.view_effect("test123", 1).await;

        match result {
            Ok(response) => {
                assert_eq!(response.code, 0);
                assert_eq!(response.desc, "Success");
            }
            Err(e) => panic!("Failed to view effect: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_add_daily_schedule() {
        let mut server = Server::new_async().await;
        let mock_response = serde_json::json!({
            "code": 0,
            "desc": "Success",
            "payload": {
                "code": 0,
                "desc": "Success"
            }
        });

        let _m = server.mock("POST", "/v1/oauth/resources/device/schedule/daily/add")
            .match_header("authorization", mockito::Matcher::Any)
            .match_header("S-ClientId", mockito::Matcher::Any)
            .match_header("S-Timestamp", mockito::Matcher::Any)
            .match_body(mockito::Matcher::JsonString(r#"{"deviceId":"test123","payload":{"id":-1,"enable":true,"effectId":1,"repetition":1,"startTime":{"hours":8,"minutes":0},"endTime":{"hours":20,"minutes":0}}}"#.to_string()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response.to_string())
            .create_async()
            .await;

        let client = create_test_client(&server).await;
        let result = client
            .add_daily_schedule("test123", 1, "08:00".to_string(), "20:00".to_string(), 1)
            .await
            .unwrap();

        assert_eq!(result.code, 0);
        assert_eq!(result.desc, "Success");
    }

    #[tokio::test]
    async fn test_add_calendar_schedule() {
        let mut server = Server::new_async().await;
        let mock_response = serde_json::json!({
            "code": 0,
            "desc": "Success",
            "payload": {
                "code": 0,
                "desc": "Success"
            }
        });

        let _m = server.mock("POST", "/v1/oauth/resources/device/schedule/calendar/add")
            .match_header("authorization", mockito::Matcher::Any)
            .match_header("S-ClientId", mockito::Matcher::Any)
            .match_header("S-Timestamp", mockito::Matcher::Any)
            .match_body(mockito::Matcher::JsonString(r#"{"deviceId":"test123","payload":{"id":0,"effectId":1,"startDate":{"month":12,"day":25},"endDate":{"month":12,"day":31},"startTime":{"hours":17,"minutes":0},"endTime":{"hours":23,"minutes":0}}}"#.to_string()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response.to_string())
            .create_async()
            .await;

        let client = create_test_client(&server).await;
        let result = client
            .add_calendar_schedule(
                "test123",
                1,
                "12-25".to_string(),
                "12-31".to_string(),
                "17:00".to_string(),
                "23:00".to_string(),
            )
            .await
            .unwrap();

        assert_eq!(result.code, 0);
        assert_eq!(result.desc, "Success");
    }

    #[tokio::test]
    async fn test_delete_schedule() {
        let mut server = Server::new_async().await;
        let mock_response = serde_json::json!({
            "code": 0,
            "desc": "Success",
            "payload": {
                "code": 0,
                "desc": "Success"
            }
        });

        let _m = server
            .mock("POST", "/v1/oauth/resources/device/schedule/daily/delete")
            .match_header("authorization", mockito::Matcher::Any)
            .match_header("S-ClientId", mockito::Matcher::Any)
            .match_header("S-Timestamp", mockito::Matcher::Any)
            .match_body(mockito::Matcher::JsonString(
                r#"{"deviceId":"test123","payload":{"id":1}}"#.to_string(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response.to_string())
            .create_async()
            .await;

        let client = create_test_client(&server).await;
        let result = client.delete_schedule("test123", 1, "daily").await.unwrap();

        assert_eq!(result.code, 0);
        assert_eq!(result.desc, "Success");
    }

    #[tokio::test]
    async fn test_toggle_schedule() {
        let mut server = Server::new_async().await;
        let mock_response = serde_json::json!({
            "code": 0,
            "desc": "Success",
            "payload": {
                "code": 0,
                "desc": "Success"
            }
        });

        let _m = server
            .mock("POST", "/v1/oauth/resources/device/schedule/daily/update")
            .match_header("authorization", mockito::Matcher::Any)
            .match_header("S-ClientId", mockito::Matcher::Any)
            .match_header("S-Timestamp", mockito::Matcher::Any)
            .match_body(mockito::Matcher::JsonString(
                r#"{"deviceId":"test123","payload":{"id":1,"enable":true}}"#.to_string(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response.to_string())
            .create_async()
            .await;

        let client = create_test_client(&server).await;
        let result = client.toggle_schedule("test123", 1, true).await.unwrap();

        assert_eq!(result.code, 0);
        assert_eq!(result.desc, "Success");
    }

    #[tokio::test]
    async fn test_modify_schedule() {
        let mut server = Server::new_async().await;

        // Mock for get_device_schedules (which calls get_device_details)
        let details_response = serde_json::json!({
            "code": 0,
            "desc": "Success",
            "payload": {
                "daily": [{
                    "id": 1,
                    "enable": true,
                    "effectId": 1,
                    "repetition": 1,
                    "startTime": {
                        "hours": 8,
                        "minutes": 0
                    },
                    "endTime": {
                        "hours": 20,
                        "minutes": 0
                    }
                }],
                "calendar": []
            }
        });

        let _m1 = server
            .mock("POST", "/v1/oauth/resources/device/get")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(details_response.to_string())
            .create_async()
            .await;

        // Mock for modify_schedule
        let update_response = serde_json::json!({
            "code": 0,
            "desc": "Success",
            "payload": {
                "code": 0,
                "desc": "Success"
            }
        });

        let _m2 = server.mock("POST", "/v1/oauth/resources/device/schedule/daily/update")
            .match_body(mockito::Matcher::JsonString(r#"{"deviceId":"test123","payload":{"id":1,"enable":true,"effectId":2,"startTime":{"hours":9,"minutes":0},"endTime":{"hours":21,"minutes":0},"repetition":1}}"#.to_string()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(update_response.to_string())
            .create_async()
            .await;

        let client = create_test_client(&server).await;
        let result = client
            .modify_schedule(
                "test123",
                1,
                "daily",
                Some(2),
                Some("09:00".to_string()),
                "21:00".to_string(),
                None,
            )
            .await
            .unwrap();

        assert_eq!(result.code, 0);
        assert_eq!(result.desc, "Success");
    }

    #[tokio::test]
    async fn test_check_schedule_conflicts() {
        let mut server = Server::new_async().await;

        // Mock for get_device_schedules (which calls get_device_details)
        let details_response = serde_json::json!({
            "code": 0,
            "desc": "Success",
            "payload": {
                "daily": [
                    {
                        "id": 1,
                        "enable": true,
                        "effectId": 1,
                        "repetition": 1,
                        "startTime": { "hours": 8, "minutes": 0 },
                        "endTime": { "hours": 12, "minutes": 0 }
                    },
                    {
                        "id": 2,
                        "enable": true,
                        "effectId": 2,
                        "repetition": 1,
                        "startTime": { "hours": 10, "minutes": 0 },
                        "endTime": { "hours": 14, "minutes": 0 }
                    }
                ],
                "calendar": []
            }
        });

        let _m = server
            .mock("POST", "/v1/oauth/resources/device/get")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(details_response.to_string())
            .create_async()
            .await;

        let client = create_test_client(&server).await;
        let result = client.check_schedule_conflicts("test123").await.unwrap();

        assert_eq!(result.code, 1);
        assert!(result
            .desc
            .contains("Daily schedules 1 and 2 have overlapping times"));
    }

    #[tokio::test]
    async fn test_set_combined_effect() {
        let mut server = Server::new_async().await;
        let mock_response = serde_json::json!({
            "code": 0,
            "desc": "Success",
            "payload": {
                "code": 0,
                "desc": "Success"
            }
        });

        let _m = server
            .mock("POST", "/v1/oauth/resources/device/effect/combined/set")
            .match_header("authorization", mockito::Matcher::Any)
            .match_header("S-ClientId", mockito::Matcher::Any)
            .match_header("S-Timestamp", mockito::Matcher::Any)
            .match_body(mockito::Matcher::JsonString(
                r#"{"deviceId":"test123","payload":{"effectIds":[1,2,3],"interval":30}}"#
                    .to_string(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response.to_string())
            .create_async()
            .await;

        let client = create_test_client(&server).await;
        let result = client
            .set_combined_effect("test123", &[1, 2, 3], 30)
            .await
            .unwrap();

        assert_eq!(result.code, 0);
        assert_eq!(result.desc, "Success");
    }

    #[tokio::test]
    async fn test_clear_combined_effect() {
        let mut server = Server::new_async().await;
        let mock_response = serde_json::json!({
            "code": 0,
            "desc": "Success",
            "payload": {
                "code": 0,
                "desc": "Success"
            }
        });

        let _m = server
            .mock("POST", "/v1/oauth/resources/device/effect/combined/set")
            .match_header("authorization", mockito::Matcher::Any)
            .match_header("S-ClientId", mockito::Matcher::Any)
            .match_header("S-Timestamp", mockito::Matcher::Any)
            .match_body(mockito::Matcher::JsonString(
                r#"{"deviceId":"test123","payload":{"effectIds":[],"interval":0}}"#.to_string(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response.to_string())
            .create_async()
            .await;

        let client = create_test_client(&server).await;
        let result = client.clear_combined_effect("test123").await.unwrap();

        assert_eq!(result.code, 0);
        assert_eq!(result.desc, "Success");
    }

    #[tokio::test]
    async fn test_add_overlay_effect() {
        let mut server = Server::new_async().await;
        let mock_response = serde_json::json!({
            "code": 0,
            "desc": "Success",
            "payload": {
                "code": 0,
                "desc": "Success"
            }
        });

        let _m = server.mock("POST", "/v1/oauth/resources/device/effect/overlay")
            .match_header("authorization", mockito::Matcher::Any)
            .match_header("S-ClientId", mockito::Matcher::Any)
            .match_header("S-Timestamp", mockito::Matcher::Any)
            .match_body(mockito::Matcher::JsonString(r#"{"deviceId":"test123","payload":{"overlayEffects":[{"overlayType":1,"targetEffect":2}]}}"#.to_string()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response.to_string())
            .create_async()
            .await;

        let client = create_test_client(&server).await;
        let result = client.add_overlay_effect("test123", 1, 2).await.unwrap();

        assert_eq!(result.code, 0);
        assert_eq!(result.desc, "Success");
    }

    #[tokio::test]
    async fn test_clear_overlay_effects() {
        let mut server = Server::new_async().await;
        let mock_response = serde_json::json!({
            "code": 0,
            "desc": "Success",
            "payload": {
                "code": 0,
                "desc": "Success"
            }
        });

        let _m = server
            .mock("POST", "/v1/oauth/resources/device/effect/overlay")
            .match_header("authorization", mockito::Matcher::Any)
            .match_header("S-ClientId", mockito::Matcher::Any)
            .match_header("S-Timestamp", mockito::Matcher::Any)
            .match_body(mockito::Matcher::JsonString(
                r#"{"deviceId":"test123","payload":{"overlayEffects":[]}}"#.to_string(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response.to_string())
            .create_async()
            .await;

        let client = create_test_client(&server).await;
        let result = client.clear_overlay_effects("test123").await.unwrap();

        assert_eq!(result.code, 0);
        assert_eq!(result.desc, "Success");
    }
}
