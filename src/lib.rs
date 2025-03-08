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

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicResponse {
    pub code: i32,
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
}
