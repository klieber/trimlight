# Trimlight API Client Library

The Trimlight API client library provides a Rust interface for interacting with Trimlight LED devices through their API.

For detailed API endpoint documentation, see [API Documentation](trimlight-api.md).

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
trimlight = { git = "https://github.com/yourusername/trimlight" }
```

## Authentication

The client requires API credentials to authenticate with the Trimlight service:

```rust
use trimlight::TrimlightClient;

let client = TrimlightClient::new(
    "your_client_id".to_string(),
    "your_client_secret".to_string()
);
```

## Usage

### Device Management

```rust
// List all devices
let devices = client.get_device_list(None).await?;

// Get details for a specific device
let details = client.get_device_details("device_id").await?;

// Rename a device
let response = client.set_device_name("device_id", "New Name").await?;

// Change device state (0=off, 1=manual, 2=timer)
let response = client.set_device_switch_state("device_id", 1).await?;
```

### Effect Control

```rust
// Preview a built-in effect
let response = client.preview_builtin_effect(
    "device_id",  // Device ID
    1,            // Effect mode (0-179)
    100,          // Speed (0-255)
    100,          // Brightness (0-255)
    30,           // Pixel length (1-90)
    false         // Reverse direction
).await?;

// View (load) a saved effect
let response = client.view_effect(
    "device_id",  // Device ID
    1,            // Effect ID
).await?;

// Add a custom effect
let response = client.add_effect(
    "device_id",
    "My Effect",  // Effect name
    1,            // Effect mode (0-16)
    100,          // Speed
    100,          // Brightness
    Some(30),     // Optional pixel length
    Some(false),  // Optional reverse direction
    None,         // Optional pixel data
).await?;

// Update an existing effect
let response = client.update_effect(
    "device_id",
    1,                    // Effect ID
    Some("New Name"),    // Optional new name
    Some(2),            // Optional new mode
    Some(150),          // Optional new speed
    Some(200),          // Optional new brightness
    Some(45),           // Optional new pixel length
    Some(true),         // Optional new reverse direction
    None,               // Optional new pixel data
).await?;

// Delete an effect
let response = client.delete_effect("device_id", 1).await?;

// Set a combined effect sequence
let effect_ids = vec![1, 2, 3];
let response = client.set_combined_effect(
    "device_id",
    &effect_ids,
    60,  // Interval in seconds
).await?;

// Clear a combined effect sequence
let response = client.clear_combined_effect("device_id").await?;

// Add an overlay effect (lightning or snow)
let response = client.add_overlay_effect(
    "device_id",
    0,            // Effect type (0=lightning, 1=snow)
    1,            // Target effect ID
).await?;

// Clear all overlay effects
let response = client.clear_overlay_effects("device_id").await?;
```

## Response Types

### DeviceList

```rust
pub struct DeviceList {
    pub total: i32,
    pub data: Vec<Device>,
}

pub struct Device {
    pub device_id: String,
    pub name: String,
    pub connectivity: i32,  // 0=offline, 1=online
    pub switch_state: i32,  // 0=off, 1=manual, 2=timer
    pub fw_version_name: String,
}
```

### DeviceDetails

```rust
pub struct DeviceDetails {
    pub device_id: String,
    pub name: String,
    pub connectivity: i32,
    pub switch_state: i32,
    pub fw_version_name: String,
    pub color_order: String,
    pub ic: String,
    pub ports: Vec<Port>,
    pub effects: Vec<Effect>,
    pub current_effect: Option<CurrentEffect>,
    pub daily: Vec<DailySchedule>,
    pub calendar: Vec<CalendarSchedule>,
}
```

### BasicResponse

```rust
pub struct BasicResponse {
    pub code: i32,     // 0 indicates success
    pub desc: String,  // Description or error message
}
```

## Error Handling

The library uses a custom error type `TrimlightError` that wraps various error cases:

```rust
pub enum TrimlightError {
    RequestError(reqwest::Error),
    ApiError { code: i32, message: String },
    JsonError(serde_json::Error),
}
```

All API methods return `Result<T, Box<dyn std::error::Error>>` where `T` is the appropriate response type.

## Built-in Effects

The API supports 180 built-in effects (modes 0-179) and 17 custom effects (modes 0-16). See the [effect documentation](effects.md) for a complete list of available effects.
