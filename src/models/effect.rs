use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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
