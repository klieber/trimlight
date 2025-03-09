use clap::{Parser, Subcommand};
use std::env;
use trimlight::TrimlightClient;

#[derive(Parser)]
#[command(name = "trimlight")]
#[command(about = "Control Trimlight LED devices", long_about = None)]
struct Cli {
    /// Output raw JSON response instead of formatted text
    #[arg(long, global = true)]
    json: bool,

    #[command(subcommand)]
    command: Commands,
}

// First, add the effect mode data structure
#[derive(Debug)]
struct EffectMode {
    id: i32,
    name: &'static str,
    category: &'static str,
}

// Define the built-in effects
const BUILT_IN_EFFECTS: &[EffectMode] = &[
    EffectMode { id: 0, name: "Rainbow Gradual Chase", category: "Built-in" },
    EffectMode { id: 1, name: "Rainbow Comet", category: "Built-in" },
    EffectMode { id: 2, name: "Rainbow Segment", category: "Built-in" },
    EffectMode { id: 3, name: "Rainbow Wave", category: "Built-in" },
    EffectMode { id: 4, name: "Rainbow Meteor", category: "Built-in" },
    EffectMode { id: 5, name: "Rainbow Gradual", category: "Built-in" },
    EffectMode { id: 6, name: "Rainbow Jump", category: "Built-in" },
    EffectMode { id: 7, name: "Rainbow Stars", category: "Built-in" },
    EffectMode { id: 8, name: "Rainbow Fade In Out", category: "Built-in" },
    EffectMode { id: 9, name: "Rainbow Spin", category: "Built-in" },
    EffectMode { id: 10, name: "Red Stacking", category: "Built-in" },
    EffectMode { id: 11, name: "Green Stacking", category: "Built-in" },
    EffectMode { id: 12, name: "Blue Stacking", category: "Built-in" },
    EffectMode { id: 13, name: "Yellow Stacking", category: "Built-in" },
    EffectMode { id: 14, name: "Cyan Stacking", category: "Built-in" },
    EffectMode { id: 15, name: "Purple Stacking", category: "Built-in" },
    EffectMode { id: 16, name: "White Stacking", category: "Built-in" },
    EffectMode { id: 17, name: "Full Color Stack", category: "Built-in" },
    EffectMode { id: 18, name: "Red to Green Stack", category: "Built-in" },
    EffectMode { id: 19, name: "Green to Blue Stack", category: "Built-in" },
    EffectMode { id: 20, name: "Blue to Yellow Stack", category: "Built-in" },
    EffectMode { id: 21, name: "Yellow to Cyan Stack", category: "Built-in" },
    EffectMode { id: 22, name: "Cyan to Purple Stack", category: "Built-in" },
    EffectMode { id: 23, name: "Purple to White Stack", category: "Built-in" },
    EffectMode { id: 24, name: "Red Comet", category: "Built-in" },
    EffectMode { id: 25, name: "Green Comet", category: "Built-in" },
    EffectMode { id: 26, name: "Blue Comet", category: "Built-in" },
    EffectMode { id: 27, name: "Yellow Comet", category: "Built-in" },
    EffectMode { id: 28, name: "Cyan Comet", category: "Built-in" },
    EffectMode { id: 29, name: "Purple Comet", category: "Built-in" },
    EffectMode { id: 30, name: "White Comet", category: "Built-in" },
    EffectMode { id: 31, name: "Red Meteor", category: "Built-in" },
    EffectMode { id: 32, name: "Green Meteor", category: "Built-in" },
    EffectMode { id: 33, name: "Blue Meteor", category: "Built-in" },
    EffectMode { id: 34, name: "Yellow Meteor", category: "Built-in" },
    EffectMode { id: 35, name: "Cyan Meteor", category: "Built-in" },
    EffectMode { id: 36, name: "Purple Meteor", category: "Built-in" },
    EffectMode { id: 37, name: "White Meteor", category: "Built-in" },
    EffectMode { id: 38, name: "Red Wave", category: "Built-in" },
    EffectMode { id: 39, name: "Green Wave", category: "Built-in" },
    EffectMode { id: 40, name: "Blue Wave", category: "Built-in" },
    EffectMode { id: 41, name: "Yellow Wave", category: "Built-in" },
    EffectMode { id: 42, name: "Cyan Wave", category: "Built-in" },
    EffectMode { id: 43, name: "Purple Wave", category: "Built-in" },
    EffectMode { id: 44, name: "White Wave", category: "Built-in" },
    EffectMode { id: 45, name: "Red Green Wave", category: "Built-in" },
    EffectMode { id: 46, name: "Red Blue Wave", category: "Built-in" },
    EffectMode { id: 47, name: "Red Yellow Wave", category: "Built-in" },
    EffectMode { id: 48, name: "Red Cyan Wave", category: "Built-in" },
    EffectMode { id: 49, name: "Red Purple Wave", category: "Built-in" },
    EffectMode { id: 50, name: "Red White Wave", category: "Built-in" },
    EffectMode { id: 51, name: "Green Blue Wave", category: "Built-in" },
    EffectMode { id: 52, name: "Green Yellow Wave", category: "Built-in" },
    EffectMode { id: 53, name: "Green Cyan Wave", category: "Built-in" },
    EffectMode { id: 54, name: "Green Purple Wave", category: "Built-in" },
    EffectMode { id: 55, name: "Green White Wave", category: "Built-in" },
    EffectMode { id: 56, name: "Blue Yellow Wave", category: "Built-in" },
    EffectMode { id: 57, name: "Blue Cyan Wave", category: "Built-in" },
    EffectMode { id: 58, name: "Blue Purple Wave", category: "Built-in" },
    EffectMode { id: 59, name: "Blue White Wave", category: "Built-in" },
    EffectMode { id: 60, name: "Yellow Cyan Wave", category: "Built-in" },
    EffectMode { id: 61, name: "Yellow Purple Wave", category: "Built-in" },
    EffectMode { id: 62, name: "Yellow White Wave", category: "Built-in" },
    EffectMode { id: 63, name: "Cyan Purple Wave", category: "Built-in" },
    EffectMode { id: 64, name: "Cyan White Wave", category: "Built-in" },
    EffectMode { id: 65, name: "Purple White Wave", category: "Built-in" },
    EffectMode { id: 66, name: "Red Dot Pulse", category: "Built-in" },
    EffectMode { id: 67, name: "Green Dot Pulse", category: "Built-in" },
    EffectMode { id: 68, name: "Blue Dot Pulse", category: "Built-in" },
    EffectMode { id: 69, name: "Yellow Dot Pulse", category: "Built-in" },
    EffectMode { id: 70, name: "Cyan Dot Pulse", category: "Built-in" },
    EffectMode { id: 71, name: "Purple Dot Pulse", category: "Built-in" },
    EffectMode { id: 72, name: "White Dot Pulse", category: "Built-in" },
    EffectMode { id: 73, name: "Red Green Blank Pulse", category: "Built-in" },
    EffectMode { id: 74, name: "Green Blue Blank Pulse", category: "Built-in" },
    EffectMode { id: 75, name: "Blue Yellow Blank Pulse", category: "Built-in" },
    EffectMode { id: 76, name: "Yellow Cyan Blank Pulse", category: "Built-in" },
    EffectMode { id: 77, name: "Cyan Purple Blank Pulse", category: "Built-in" },
    EffectMode { id: 78, name: "Purple White Blank Pulse", category: "Built-in" },
    EffectMode { id: 79, name: "Red with Purple Pulse", category: "Built-in" },
    EffectMode { id: 80, name: "Green with Cyan Pulse", category: "Built-in" },
    EffectMode { id: 81, name: "Blue with Yellow Pulse", category: "Built-in" },
    EffectMode { id: 82, name: "Yellow with Blue Pulse", category: "Built-in" },
    EffectMode { id: 83, name: "Cyan with Green Pulse", category: "Built-in" },
    EffectMode { id: 84, name: "Purple with Purple Pulse", category: "Built-in" },
    EffectMode { id: 85, name: "Red Comet Spin", category: "Built-in" },
    EffectMode { id: 86, name: "Green Comet Spin", category: "Built-in" },
    EffectMode { id: 87, name: "Blue Comet Spin", category: "Built-in" },
    EffectMode { id: 88, name: "Yellow Comet Spin", category: "Built-in" },
    EffectMode { id: 89, name: "Cyan Comet Spin", category: "Built-in" },
    EffectMode { id: 90, name: "Purple Comet Spin", category: "Built-in" },
    EffectMode { id: 91, name: "White Comet Spin", category: "Built-in" },
    EffectMode { id: 92, name: "Red Dot Spin", category: "Built-in" },
    EffectMode { id: 93, name: "Green Dot Spin", category: "Built-in" },
    EffectMode { id: 94, name: "Blue Dot Spin", category: "Built-in" },
    EffectMode { id: 95, name: "Yellow Dot Spin", category: "Built-in" },
    EffectMode { id: 96, name: "Cyan Dot Spin", category: "Built-in" },
    EffectMode { id: 97, name: "Purple Dot Spin", category: "Built-in" },
    EffectMode { id: 98, name: "White Dot Spin", category: "Built-in" },
    EffectMode { id: 99, name: "Red Segment Spin", category: "Built-in" },
    EffectMode { id: 100, name: "Green Segment Spin", category: "Built-in" },
    EffectMode { id: 101, name: "Blue Segment Spin", category: "Built-in" },
    EffectMode { id: 102, name: "Yellow Segment Spin", category: "Built-in" },
    EffectMode { id: 103, name: "Cyan Segment Spin", category: "Built-in" },
    EffectMode { id: 104, name: "Purple Segment Spin", category: "Built-in" },
    EffectMode { id: 105, name: "White Segment Spin", category: "Built-in" },
    EffectMode { id: 106, name: "Red Green Gradual Snake", category: "Built-in" },
    EffectMode { id: 107, name: "Red Blue Gradual Snake", category: "Built-in" },
    EffectMode { id: 108, name: "Red Yellow Gradual Snake", category: "Built-in" },
    EffectMode { id: 109, name: "Red Cyan Gradual Snake", category: "Built-in" },
    EffectMode { id: 110, name: "Red Purple Gradual Snake", category: "Built-in" },
    EffectMode { id: 111, name: "Red White Gradual Snake", category: "Built-in" },
    EffectMode { id: 112, name: "Green Blue Gradual Snake", category: "Built-in" },
    EffectMode { id: 113, name: "Green Yellow Gradual Snake", category: "Built-in" },
    EffectMode { id: 114, name: "Green Cyan Gradual Snake", category: "Built-in" },
    EffectMode { id: 115, name: "Green Purple Gradual Snake", category: "Built-in" },
    EffectMode { id: 116, name: "Green White Gradual Snake", category: "Built-in" },
    EffectMode { id: 117, name: "Blue Yellow Gradual Snake", category: "Built-in" },
    EffectMode { id: 118, name: "Blue Cyan Gradual Snake", category: "Built-in" },
    EffectMode { id: 119, name: "Blue Purple Gradual Snake", category: "Built-in" },
    EffectMode { id: 120, name: "Blue White Gradual Snake", category: "Built-in" },
    EffectMode { id: 121, name: "Yellow Cyan Gradual Snake", category: "Built-in" },
    EffectMode { id: 122, name: "Yellow Purple Gradual Snake", category: "Built-in" },
    EffectMode { id: 123, name: "Yellow White Gradual Snake", category: "Built-in" },
    EffectMode { id: 124, name: "Cyan Purple Gradual Snake", category: "Built-in" },
    EffectMode { id: 125, name: "Cyan White Gradual Snake", category: "Built-in" },
    EffectMode { id: 126, name: "Purple White Gradual Snake", category: "Built-in" },
    EffectMode { id: 127, name: "Red White Blank Snake", category: "Built-in" },
    EffectMode { id: 128, name: "Green White Blank Snake", category: "Built-in" },
    EffectMode { id: 129, name: "Blue White Blank Snake", category: "Built-in" },
    EffectMode { id: 130, name: "Yellow White Blank Snake", category: "Built-in" },
    EffectMode { id: 131, name: "Cyan White Blank Snake", category: "Built-in" },
    EffectMode { id: 132, name: "Purple White Blank Snake", category: "Built-in" },
    EffectMode { id: 133, name: "Green Yellow White Snake", category: "Built-in" },
    EffectMode { id: 134, name: "Red Green White Snake", category: "Built-in" },
    EffectMode { id: 135, name: "Red Yellow Snake", category: "Built-in" },
    EffectMode { id: 136, name: "Red White Snake", category: "Built-in" },
    EffectMode { id: 137, name: "Green White Snake", category: "Built-in" },
    EffectMode { id: 138, name: "Red Stars", category: "Built-in" },
    EffectMode { id: 139, name: "Green Stars", category: "Built-in" },
    EffectMode { id: 140, name: "Blue Stars", category: "Built-in" },
    EffectMode { id: 141, name: "Yellow Stars", category: "Built-in" },
    EffectMode { id: 142, name: "Cyan Stars", category: "Built-in" },
    EffectMode { id: 143, name: "Purple Stars", category: "Built-in" },
    EffectMode { id: 144, name: "White Stars", category: "Built-in" },
    EffectMode { id: 145, name: "Red Background Stars", category: "Built-in" },
    EffectMode { id: 146, name: "Green Background Stars", category: "Built-in" },
    EffectMode { id: 147, name: "Blue Background Stars", category: "Built-in" },
    EffectMode { id: 148, name: "Yellow Background Stars", category: "Built-in" },
    EffectMode { id: 149, name: "Cyan Background Stars", category: "Built-in" },
    EffectMode { id: 150, name: "Purple Background Stars", category: "Built-in" },
    EffectMode { id: 151, name: "Red White Background Stars", category: "Built-in" },
    EffectMode { id: 152, name: "Green White Background Stars", category: "Built-in" },
    EffectMode { id: 153, name: "Blue White Background Stars", category: "Built-in" },
    EffectMode { id: 154, name: "Yellow White Background Stars", category: "Built-in" },
    EffectMode { id: 155, name: "Cyan White Background Stars", category: "Built-in" },
    EffectMode { id: 156, name: "Purple White Background Stars", category: "Built-in" },
    EffectMode { id: 157, name: "White White Background Stars", category: "Built-in" },
    EffectMode { id: 158, name: "Red Breath", category: "Built-in" },
    EffectMode { id: 159, name: "Green Breath", category: "Built-in" },
    EffectMode { id: 160, name: "Blue Breath", category: "Built-in" },
    EffectMode { id: 161, name: "Yellow Breath", category: "Built-in" },
    EffectMode { id: 162, name: "Cyan Breath", category: "Built-in" },
    EffectMode { id: 163, name: "Purple Breath", category: "Built-in" },
    EffectMode { id: 164, name: "White Breath", category: "Built-in" },
    EffectMode { id: 165, name: "Red Yellow Fire", category: "Built-in" },
    EffectMode { id: 166, name: "Red Purple Fire", category: "Built-in" },
    EffectMode { id: 167, name: "Green Yellow Fire", category: "Built-in" },
    EffectMode { id: 168, name: "Green Cyan Fire", category: "Built-in" },
    EffectMode { id: 169, name: "Blue Purple Fire", category: "Built-in" },
    EffectMode { id: 170, name: "Blue Cyan Fire", category: "Built-in" },
    EffectMode { id: 171, name: "Red Strobe", category: "Built-in" },
    EffectMode { id: 172, name: "Green Strobe", category: "Built-in" },
    EffectMode { id: 173, name: "Blue Strobe", category: "Built-in" },
    EffectMode { id: 174, name: "Yellow Strobe", category: "Built-in" },
    EffectMode { id: 175, name: "Cyan Strobe", category: "Built-in" },
    EffectMode { id: 176, name: "Purple Strobe", category: "Built-in" },
    EffectMode { id: 177, name: "White Strobe", category: "Built-in" },
    EffectMode { id: 178, name: "Red Blue White Strobe", category: "Built-in" },
    EffectMode { id: 179, name: "Full Color Strobe", category: "Built-in" },
];

// Define the custom effects
const CUSTOM_EFFECTS: &[EffectMode] = &[
    EffectMode { id: 0, name: "Static", category: "Custom" },
    EffectMode { id: 1, name: "Chase Forward", category: "Custom" },
    EffectMode { id: 2, name: "Chase Backward", category: "Custom" },
    EffectMode { id: 3, name: "Chase Middle to Out", category: "Custom" },
    EffectMode { id: 4, name: "Chase Out to Middle", category: "Custom" },
    EffectMode { id: 5, name: "Stars", category: "Custom" },
    EffectMode { id: 6, name: "Breath", category: "Custom" },
    EffectMode { id: 7, name: "Comet Forward", category: "Custom" },
    EffectMode { id: 8, name: "Comet Backward", category: "Custom" },
    EffectMode { id: 9, name: "Comet Middle to Out", category: "Custom" },
    EffectMode { id: 10, name: "Comet Out to Middle", category: "Custom" },
    EffectMode { id: 11, name: "Wave Forward", category: "Custom" },
    EffectMode { id: 12, name: "Wave Backward", category: "Custom" },
    EffectMode { id: 13, name: "Wave Middle to Out", category: "Custom" },
    EffectMode { id: 14, name: "Wave Out to Middle", category: "Custom" },
    EffectMode { id: 15, name: "Strobe", category: "Custom" },
    EffectMode { id: 16, name: "Solid Fade", category: "Custom" },
];

// Add helper function to get default device
async fn get_default_device(client: &TrimlightClient) -> Result<String, Box<dyn std::error::Error>> {
    let devices = client.get_device_list(None).await?;
    if devices.data.is_empty() {
        return Err("No devices found. Please specify a device ID using --device".into());
    }
    Ok(devices.data[0].device_id.clone())
}

#[derive(Subcommand)]
enum Commands {
    /// List all devices
    List {
        /// Page number (optional)
        #[arg(short, long)]
        page: Option<i32>,
    },
    /// Get detailed information about a device
    Details {
        /// Device ID (optional, uses first device if not specified)
        #[arg(short, long)]
        device: Option<String>,
    },
    /// Turn a device on or off
    #[command(after_help = "Examples:\n\
    # Turn device off\n\
    trimlight-cli switch --off\n\
    \n\
    # Turn on manual mode\n\
    trimlight-cli switch --manual\n\
    \n\
    # Turn on timer mode\n\
    trimlight-cli switch --timer\n\
    \n\
    # Specify a particular device\n\
    trimlight-cli switch --device abc123 --off")]
    Switch {
        /// Device ID (optional, uses first device if not specified)
        #[arg(short, long)]
        device: Option<String>,
        /// Turn device off
        #[arg(long)]
        off: bool,
        /// Turn on manual mode
        #[arg(long)]
        manual: bool,
        /// Turn on timer mode
        #[arg(long)]
        timer: bool,
    },
    /// Rename a device
    Rename {
        /// Device ID (optional, uses first device if not specified)
        #[arg(short, long)]
        device: Option<String>,
        /// New name
        #[arg(short, long)]
        name: String,
    },
    /// List and search available effect modes
    #[command(after_help = "Examples:\n\
    # List all modes\n\
    trimlight-cli modes\n\
    \n\
    # Search for modes containing 'rainbow'\n\
    trimlight-cli modes --search rainbow\n\
    \n\
    # List only built-in modes\n\
    trimlight-cli modes --built-in\n\
    \n\
    # List only custom modes\n\
    trimlight-cli modes --custom\n\
    \n\
    Categories:\n\
    - Built-in effects (modes 0-179)\n\
    - Custom effects for pixel-by-pixel control (modes 0-16)")]
    Modes {
        /// Search for modes containing this text (case-insensitive)
        #[arg(short, long)]
        search: Option<String>,
        /// Show only built-in effects
        #[arg(long)]
        built_in: bool,
        /// Show only custom effects
        #[arg(long)]
        custom: bool,
    },
    /// Manage schedules
    #[command(subcommand)]
    Schedule(ScheduleCommands),
    /// Manage combined effects (multiple effects running in sequence)
    #[command(subcommand)]
    Combined(CombinedCommands),
    /// Manage and control effects
    #[command(subcommand)]
    Effects(EffectCommands),
}

#[derive(Subcommand)]
enum ScheduleCommands {
    /// List all schedules
    List {
        /// Device ID (optional, uses first device if not specified)
        #[arg(short, long)]
        device: Option<String>,
    },
    /// Add a daily schedule
    Daily {
        /// Device ID (optional, uses first device if not specified)
        #[arg(short, long)]
        device: Option<String>,
        /// Effect ID to display
        #[arg(short, long)]
        effect: i32,
        /// Start time (HH:MM format)
        #[arg(long)]
        start: String,
        /// End time (HH:MM format)
        #[arg(long)]
        end: String,
        /// Repetition type (0=today, 1=everyday, 2=weekdays, 3=weekend)
        #[arg(short, long)]
        repeat: i32,
    },
    /// Add a calendar schedule
    Calendar {
        /// Device ID (optional, uses first device if not specified)
        #[arg(short, long)]
        device: Option<String>,
        /// Effect ID to display
        #[arg(short, long)]
        effect: i32,
        /// Start date (MM-DD format)
        #[arg(long)]
        start_date: String,
        /// End date (MM-DD format)
        #[arg(long)]
        end_date: String,
        /// Start time (HH:MM format)
        #[arg(long)]
        start_time: String,
        /// End time (HH:MM format)
        #[arg(long)]
        end_time: String,
    },
    /// Delete a schedule
    Delete {
        /// Device ID (optional, uses first device if not specified)
        #[arg(short, long)]
        device: Option<String>,
        /// Schedule ID to delete
        #[arg(short, long)]
        id: i32,
        /// Schedule type (daily or calendar)
        #[arg(short, long)]
        schedule_type: String,
    },
    /// Enable or disable a daily schedule
    Toggle {
        /// Device ID (optional, uses first device if not specified)
        #[arg(short, long)]
        device: Option<String>,
        /// Schedule ID to toggle
        #[arg(short, long)]
        id: i32,
        /// Enable the schedule (if not specified, will disable)
        #[arg(long)]
        enable: bool,
    },
    /// Modify an existing daily schedule
    Modify {
        /// Device ID (optional, uses first device if not specified)
        #[arg(short, long)]
        device: Option<String>,
        /// Schedule ID to modify
        #[arg(short, long)]
        id: i32,
        /// Schedule type (daily or calendar)
        #[arg(short, long)]
        schedule_type: String,
        /// Effect ID to display (optional)
        #[arg(short, long)]
        effect: Option<i32>,
        /// Start time (HH:MM format, optional)
        #[arg(long)]
        start: Option<String>,
        /// End time (HH:MM format, optional)
        #[arg(long)]
        end: String,
        /// Repetition type (0=today, 1=everyday, 2=weekdays, 3=weekend, optional)
        #[arg(short, long)]
        repeat: Option<i32>,
    },
    /// Check for schedule conflicts
    Check {
        /// Device ID (optional, uses first device if not specified)
        #[arg(short, long)]
        device: Option<String>,
    },
}

#[derive(Subcommand)]
enum CombinedCommands {
    /// Set a combined effect sequence
    Set {
        /// Device ID (optional, uses first device if not specified)
        #[arg(short, long)]
        device: Option<String>,
        /// List of effect IDs to run in sequence (comma-separated)
        #[arg(short, long)]
        effects: String,
        /// Interval between effects in seconds (1-3600)
        #[arg(short, long, default_value = "60")]
        interval: i32,
    },
    /// Clear the combined effect sequence
    Clear {
        /// Device ID (optional, uses first device if not specified)
        #[arg(short, long)]
        device: Option<String>,
    },
}

#[derive(Subcommand)]
enum EffectCommands {
    /// List saved effects
    List {
        /// Device ID (optional, uses first device if not specified)
        #[arg(short, long)]
        device: Option<String>,
    },
    /// Preview a built-in or custom effect
    #[command(after_help = "Examples:\n\
    # Basic usage with just mode\n\
    trimlight-cli effects preview --mode 1\n\
    \n\
    # Full customization\n\
    trimlight-cli effects preview --mode 1 --speed 150 --brightness 200 --pixel-len 45 --reverse\n\
    \n\
    # Specify a particular device\n\
    trimlight-cli effects preview --device abc123 --mode 1\n\
    \n\
    Common effects:\n\
    - Mode 0: Rainbow Gradual Chase\n\
    - Mode 1: Rainbow Comet\n\
    - Mode 2: Rainbow Segment\n\
    - Mode 3: Rainbow Wave")]
    Preview {
        /// Device ID (optional, uses first device if not specified)
        #[arg(short, long)]
        device: Option<String>,
        /// Effect mode number (0-179)
        #[arg(short, long)]
        mode: i32,
        /// Effect animation speed (0=slowest, 255=fastest)
        #[arg(short = 's', long, default_value = "100")]
        speed: i32,
        /// LED brightness level (0=off, 255=maximum)
        #[arg(short, long, default_value = "100")]
        brightness: i32,
        /// Number of LEDs to use in the effect (1-90)
        #[arg(short, long, default_value = "30")]
        pixel_len: i32,
        /// Reverse the effect animation direction
        #[arg(short, long)]
        reverse: bool,
    },
    /// Add a new custom effect
    Add {
        /// Device ID (optional, uses first device if not specified)
        #[arg(short, long)]
        device: Option<String>,
        /// Effect name
        #[arg(short, long)]
        name: String,
        /// Effect mode (0-16 for custom effects)
        #[arg(short, long)]
        mode: i32,
        /// Effect speed (0-255)
        #[arg(short, long, default_value = "100")]
        speed: i32,
        /// LED brightness (0-255)
        #[arg(short, long, default_value = "100")]
        brightness: i32,
        /// Number of LEDs to use (1-90)
        #[arg(short, long)]
        pixel_len: Option<i32>,
        /// Reverse the effect direction
        #[arg(short, long)]
        reverse: bool,
    },
    /// Update an existing effect
    Update {
        /// Device ID (optional, uses first device if not specified)
        #[arg(short, long)]
        device: Option<String>,
        /// Effect ID to update
        #[arg(short, long)]
        id: i32,
        /// New effect name
        #[arg(short, long)]
        name: Option<String>,
        /// New effect mode (0-16 for custom effects)
        #[arg(short, long)]
        mode: Option<i32>,
        /// New effect speed (0-255)
        #[arg(short, long)]
        speed: Option<i32>,
        /// New LED brightness (0-255)
        #[arg(short, long)]
        brightness: Option<i32>,
        /// New number of LEDs to use (1-90)
        #[arg(short, long)]
        pixel_len: Option<i32>,
        /// New reverse direction setting
        #[arg(short, long)]
        reverse: Option<bool>,
    },
    /// Delete an effect
    Delete {
        /// Device ID (optional, uses first device if not specified)
        #[arg(short, long)]
        device: Option<String>,
        /// Effect ID to delete
        #[arg(short, long)]
        id: i32,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Get API credentials from environment variables
    let client_id = env::var("TRIMLIGHT_CLIENT_ID")
        .expect("TRIMLIGHT_CLIENT_ID environment variable not set");
    let client_secret = env::var("TRIMLIGHT_CLIENT_SECRET")
        .expect("TRIMLIGHT_CLIENT_SECRET environment variable not set");

    // Create Trimlight client
    let client = TrimlightClient::new(client_id, client_secret);

    // Parse command line arguments
    let cli = Cli::parse();

    // Handle commands
    match cli.command {
        Commands::List { page } => {
            let devices = client.get_device_list(page).await?;
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&devices)?);
            } else {
                println!("Found {} devices:", devices.total);
                for device in devices.data {
                    println!(
                        "- {} (ID: {})",
                        device.name,
                        device.device_id
                    );
                    println!("  Status: {}", match device.connectivity {
                        0 => "Offline",
                        1 => "Online",
                        _ => "Unknown",
                    });
                    println!("  State: {}", match device.switch_state {
                        0 => "Off",
                        1 => "Manual Mode",
                        2 => "Timer Mode",
                        _ => "Unknown",
                    });
                    println!("  Firmware: {}", device.fw_version_name);
                    println!();
                }
            }
        }
        Commands::Details { device } => {
            let device_id = match device {
                Some(id) => id,
                None => get_default_device(&client).await?,
            };
            let details = client.get_device_details(&device_id).await?;
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&details)?);
            } else {
                println!("Device Details for {}:", details.name);
                println!("Status: {}", match details.connectivity {
                    0 => "Offline",
                    1 => "Online",
                    _ => "Unknown",
                });
                println!("State: {}", match details.switch_state {
                    0 => "Off",
                    1 => "Manual Mode",
                    2 => "Timer Mode",
                    _ => "Unknown",
                });
                println!("Firmware: {}", details.fw_version_name);
                println!("Color Order: {}", details.color_order);
                println!("IC Type: {}", details.ic);

                if !details.ports.is_empty() {
                    println!("\nPorts:");
                    for port in details.ports {
                        println!("  Port {}: {} to {}", port.id, port.start, port.end);
                    }
                }

                if !details.effects.is_empty() {
                    println!("\nStored Effects:");
                    for effect in details.effects {
                        println!("  {}: {} (Mode: {})", effect.id, effect.name, effect.mode);
                    }
                }

                if let Some(current) = details.current_effect {
                    println!("\nCurrent Effect:");
                    println!("  Mode: {}", current.mode);
                    println!("  Speed: {}", current.speed);
                    println!("  Brightness: {}", current.brightness);
                    if let Some(len) = current.pixel_len {
                        println!("  Pixel Length: {}", len);
                    }
                    if let Some(rev) = current.reverse {
                        println!("  Reverse: {}", rev);
                    }
                }

                if !details.daily.is_empty() {
                    println!("\nDaily Schedules:");
                    for schedule in details.daily {
                        if schedule.enable {
                            println!("  Schedule {}: Effect {} from {:02}:{:02} to {:02}:{:02}",
                                schedule.id,
                                schedule.effect_id,
                                schedule.start_time.hours,
                                schedule.start_time.minutes,
                                schedule.end_time.hours,
                                schedule.end_time.minutes
                            );
                        }
                    }
                }

                if !details.calendar.is_empty() {
                    println!("\nCalendar Schedules:");
                    for schedule in details.calendar {
                        println!("  Schedule {}: Effect {} from {}/{} to {}/{} ({:02}:{:02} to {:02}:{:02})",
                            schedule.id,
                            schedule.effect_id,
                            schedule.start_date.month,
                            schedule.start_date.day,
                            schedule.end_date.month,
                            schedule.end_date.day,
                            schedule.start_time.hours,
                            schedule.start_time.minutes,
                            schedule.end_time.hours,
                            schedule.end_time.minutes
                        );
                    }
                }
            }
        }
        Commands::Switch { device, off, manual, timer } => {
            // Validate that exactly one state flag is provided
            let state_flags = [off, manual, timer];
            let state_count = state_flags.iter().filter(|&&flag| flag).count();
            if state_count != 1 {
                eprintln!("Error: Exactly one state flag (--off, --manual, or --timer) must be provided");
                std::process::exit(1);
            }

            // Convert flags to state number
            let state = if off {
                0
            } else if manual {
                1
            } else {
                2 // timer
            };

            let device_id = match device {
                Some(id) => id,
                None => get_default_device(&client).await?,
            };
            let response = client.set_device_switch_state(&device_id, state).await?;
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&response)?);
            } else {
                if response.code == 0 {
                    println!("Device state updated successfully");
                } else {
                    println!("Error: {} (code: {})", response.desc, response.code);
                }
            }
        }
        Commands::Rename { device, name } => {
            let device_id = match device {
                Some(id) => id,
                None => get_default_device(&client).await?,
            };
            let response = client.set_device_name(&device_id, &name).await?;
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&response)?);
            } else {
                if response.code == 0 {
                    println!("Device renamed successfully");
                } else {
                    println!("Error: {} (code: {})", response.desc, response.code);
                }
            }
        }
        Commands::Modes { search, built_in, custom } => {
            // Collect modes based on flags
            let mut modes: Vec<&EffectMode> = Vec::new();

            // If neither flag is set, show all modes
            let show_built_in = !custom || built_in;
            let show_custom = !built_in || custom;

            if show_built_in {
                modes.extend(BUILT_IN_EFFECTS.iter());
            }
            if show_custom {
                modes.extend(CUSTOM_EFFECTS.iter());
            }

            // Filter by search term if specified
            if let Some(term) = search {
                let term_lower = term.to_lowercase();
                modes.retain(|mode| mode.name.to_lowercase().contains(&term_lower));
            }

            if cli.json {
                let json_modes: Vec<serde_json::Value> = modes
                    .iter()
                    .map(|mode| serde_json::json!({
                        "id": mode.id,
                        "name": mode.name,
                        "category": mode.category
                    }))
                    .collect();
                println!("{}", serde_json::to_string_pretty(&json_modes)?);
            } else {
                if modes.is_empty() {
                    println!("No modes found matching your criteria.");
                    return Ok(());
                }

                println!("Available Effect Modes:");
                let mut current_category = "";
                for mode in modes {
                    if current_category != mode.category {
                        current_category = mode.category;
                        println!("\n{}:", mode.category);
                        if mode.category == "Custom" {
                            println!("  (For pixel-by-pixel control)");
                        }
                    }
                    println!("  {:3} - {}", mode.id, mode.name);
                }
            }
        }
        Commands::Schedule(schedule_command) => {
            match schedule_command {
                ScheduleCommands::List { device } => {
                    let device_id = match device {
                        Some(id) => id,
                        None => get_default_device(&client).await?,
                    };
                    let schedules = client.get_device_schedules(&device_id).await?;
                    if cli.json {
                        println!("{}", serde_json::to_string_pretty(&schedules)?);
                    } else {
                        println!("Daily Schedules:");
                        for schedule in &schedules.daily {
                            println!("- Schedule {}: Effect {} from {:02}:{:02} to {:02}:{:02}",
                                schedule.id,
                                schedule.effect_id,
                                schedule.start_time.hours,
                                schedule.start_time.minutes,
                                schedule.end_time.hours,
                                schedule.end_time.minutes
                            );
                            println!("  Repetition: {}", match schedule.repetition {
                                0 => "Today Only",
                                1 => "Everyday",
                                2 => "Week Days",
                                3 => "Weekend",
                                _ => "Unknown",
                            });
                            println!("  Status: {}", if schedule.enable { "Enabled" } else { "Disabled" });
                        }

                        println!("\nCalendar Schedules:");
                        for schedule in &schedules.calendar {
                            println!("- Schedule {}: Effect {} from {}-{} to {}-{}",
                                schedule.id,
                                schedule.effect_id,
                                schedule.start_date.month,
                                schedule.start_date.day,
                                schedule.end_date.month,
                                schedule.end_date.day
                            );
                            println!("  Time: {:02}:{:02} to {:02}:{:02}",
                                schedule.start_time.hours,
                                schedule.start_time.minutes,
                                schedule.end_time.hours,
                                schedule.end_time.minutes
                            );
                        }
                    }
                }
                ScheduleCommands::Daily { device, effect, start, end, repeat } => {
                    let device_id = match device {
                        Some(id) => id,
                        None => get_default_device(&client).await?,
                    };
                    let response = client.add_daily_schedule(&device_id, effect, start, end, repeat).await?;
                    if cli.json {
                        println!("{}", serde_json::to_string_pretty(&response)?);
                    } else {
                        if response.code == 0 {
                            println!("Daily schedule added successfully");
                        } else {
                            println!("Error: {} (code: {})", response.desc, response.code);
                        }
                    }
                }
                ScheduleCommands::Calendar { device, effect, start_date, end_date, start_time, end_time } => {
                    let device_id = match device {
                        Some(id) => id,
                        None => get_default_device(&client).await?,
                    };
                    let response = client.add_calendar_schedule(&device_id, effect, start_date, end_date, start_time, end_time).await?;
                    if cli.json {
                        println!("{}", serde_json::to_string_pretty(&response)?);
                    } else {
                        if response.code == 0 {
                            println!("Calendar schedule added successfully");
                        } else {
                            println!("Error: {} (code: {})", response.desc, response.code);
                        }
                    }
                }
                ScheduleCommands::Delete { device, id, schedule_type } => {
                    let device_id = match device {
                        Some(id) => id,
                        None => get_default_device(&client).await?,
                    };
                    let response = client.delete_schedule(&device_id, id, &schedule_type).await?;
                    if cli.json {
                        println!("{}", serde_json::to_string_pretty(&response)?);
                    } else {
                        if response.code == 0 {
                            println!("Schedule deleted successfully");
                        } else {
                            println!("Error: {} (code: {})", response.desc, response.code);
                        }
                    }
                }
                ScheduleCommands::Toggle { device, id, enable } => {
                    let device_id = match device {
                        Some(id) => id,
                        None => get_default_device(&client).await?,
                    };
                    let response = client.toggle_schedule(&device_id, id, enable).await?;
                    if cli.json {
                        println!("{}", serde_json::to_string_pretty(&response)?);
                    } else {
                        if response.code == 0 {
                            println!("Schedule toggled successfully");
                        } else {
                            println!("Error: {} (code: {})", response.desc, response.code);
                        }
                    }
                }
                ScheduleCommands::Modify { device, id, schedule_type, effect, start, end, repeat } => {
                    let device_id = match device {
                        Some(id) => id,
                        None => get_default_device(&client).await?,
                    };
                    let response = client.modify_schedule(&device_id, id, &schedule_type, effect, start, end, repeat).await?;
                    if cli.json {
                        println!("{}", serde_json::to_string_pretty(&response)?);
                    } else {
                        if response.code == 0 {
                            println!("Schedule modified successfully");
                        } else {
                            println!("Error: {} (code: {})", response.desc, response.code);
                        }
                    }
                }
                ScheduleCommands::Check { device } => {
                    let device_id = match device {
                        Some(id) => id,
                        None => get_default_device(&client).await?,
                    };
                    let response = client.check_schedule_conflicts(&device_id).await?;
                    if cli.json {
                        println!("{}", serde_json::to_string_pretty(&response)?);
                    } else {
                        if response.code == 0 {
                            println!("No schedule conflicts found");
                        } else {
                            println!("Error: {} (code: {})", response.desc, response.code);
                        }
                    }
                }
            }
        }
        Commands::Combined(combined_command) => {
            match combined_command {
                CombinedCommands::Set { device, effects, interval } => {
                    let device_id = match device {
                        Some(id) => id,
                        None => get_default_device(&client).await?,
                    };

                    // Parse effect IDs
                    let effect_ids: Vec<i32> = effects
                        .split(',')
                        .map(|s| s.trim().parse::<i32>())
                        .collect::<Result<Vec<i32>, _>>()
                        .map_err(|_| "Invalid effect ID format. Use comma-separated numbers")?;

                    if effect_ids.is_empty() {
                        eprintln!("Error: At least one effect ID must be provided");
                        std::process::exit(1);
                    }

                    if interval < 1 || interval > 3600 {
                        eprintln!("Error: Interval must be between 1 and 3600 seconds");
                        std::process::exit(1);
                    }

                    let response = client.set_combined_effect(&device_id, &effect_ids, interval).await?;
                    if cli.json {
                        println!("{}", serde_json::to_string_pretty(&response)?);
                    } else {
                        if response.code == 0 {
                            println!("Combined effect sequence set successfully");
                        } else {
                            println!("Error: {} (code: {})", response.desc, response.code);
                        }
                    }
                }
                CombinedCommands::Clear { device } => {
                    let device_id = match device {
                        Some(id) => id,
                        None => get_default_device(&client).await?,
                    };

                    let response = client.clear_combined_effect(&device_id).await?;
                    if cli.json {
                        println!("{}", serde_json::to_string_pretty(&response)?);
                    } else {
                        if response.code == 0 {
                            println!("Combined effect sequence cleared successfully");
                        } else {
                            println!("Error: {} (code: {})", response.desc, response.code);
                        }
                    }
                }
            }
        }
        Commands::Effects(effect_command) => {
            match effect_command {
                EffectCommands::List { device } => {
                    let device_id = match device {
                        Some(id) => id,
                        None => get_default_device(&client).await?,
                    };
                    let details = client.get_device_details(&device_id).await?;
                    if cli.json {
                        let effects_json: Vec<serde_json::Value> = details.effects
                            .iter()
                            .map(|effect| serde_json::json!({
                                "id": effect.id,
                                "name": effect.name,
                                "mode": effect.mode,
                                "speed": effect.speed,
                                "brightness": effect.brightness,
                                "pixel_len": effect.pixel_len,
                                "reverse": effect.reverse,
                                "pixels": effect.pixels
                            }))
                            .collect();
                        println!("{}", serde_json::to_string_pretty(&effects_json)?);
                    } else {
                        if details.effects.is_empty() {
                            println!("No saved effects found");
                        } else {
                            println!("Saved Effects:");
                            for effect in details.effects {
                                println!("- Effect {} ({})", effect.id, effect.name);
                                println!("  Mode: {}", effect.mode);
                                println!("  Speed: {}", effect.speed);
                                println!("  Brightness: {}", effect.brightness);
                                if let Some(len) = effect.pixel_len {
                                    println!("  Pixel Length: {}", len);
                                }
                                if let Some(rev) = effect.reverse {
                                    println!("  Reverse: {}", rev);
                                }
                                if let Some(pixels) = effect.pixels {
                                    println!("  Custom Pixels: {} defined", pixels.len());
                                }
                                println!();
                            }
                        }
                    }
                }
                EffectCommands::Preview { device, mode, speed, brightness, pixel_len, reverse } => {
                    if mode < 0 || mode > 179 {
                        eprintln!("Invalid mode. Must be between 0 and 179");
                        std::process::exit(1);
                    }
                    if speed < 0 || speed > 255 {
                        eprintln!("Invalid speed. Must be between 0 and 255");
                        std::process::exit(1);
                    }
                    if brightness < 0 || brightness > 255 {
                        eprintln!("Invalid brightness. Must be between 0 and 255");
                        std::process::exit(1);
                    }
                    if pixel_len < 1 || pixel_len > 90 {
                        eprintln!("Invalid pixel length. Must be between 1 and 90");
                        std::process::exit(1);
                    }

                    let device_id = match device {
                        Some(id) => id,
                        None => get_default_device(&client).await?,
                    };
                    let response = client
                        .preview_builtin_effect(&device_id, mode, speed, brightness, pixel_len, reverse)
                        .await?;
                    if cli.json {
                        println!("{}", serde_json::to_string_pretty(&response)?);
                    } else {
                        if response.code == 0 {
                            println!("Effect preview started successfully");
                        } else {
                            println!("Error: {} (code: {})", response.desc, response.code);
                        }
                    }
                }
                EffectCommands::Add { device, name, mode, speed, brightness, pixel_len, reverse } => {
                    let device_id = match device {
                        Some(id) => id,
                        None => get_default_device(&client).await?,
                    };
                    let response = client.add_effect(
                        &device_id,
                        &name,
                        mode,
                        speed,
                        brightness,
                        pixel_len,
                        Some(reverse),
                        None, // No pixel data for now
                    ).await?;
                    if cli.json {
                        println!("{}", serde_json::to_string_pretty(&response)?);
                    } else {
                        if response.code == 0 {
                            println!("Effect added successfully");
                        } else {
                            println!("Error: {} (code: {})", response.desc, response.code);
                        }
                    }
                }
                EffectCommands::Update { device, id, name, mode, speed, brightness, pixel_len, reverse } => {
                    let device_id = match device {
                        Some(id) => id,
                        None => get_default_device(&client).await?,
                    };
                    let response = client.update_effect(
                        &device_id,
                        id,
                        name.as_deref(),
                        mode,
                        speed,
                        brightness,
                        pixel_len,
                        reverse,
                        None, // No pixel data for now
                    ).await?;
                    if cli.json {
                        println!("{}", serde_json::to_string_pretty(&response)?);
                    } else {
                        if response.code == 0 {
                            println!("Effect updated successfully");
                        } else {
                            println!("Error: {} (code: {})", response.desc, response.code);
                        }
                    }
                }
                EffectCommands::Delete { device, id } => {
                    let device_id = match device {
                        Some(id) => id,
                        None => get_default_device(&client).await?,
                    };
                    let response = client.delete_effect(&device_id, id).await?;
                    if cli.json {
                        println!("{}", serde_json::to_string_pretty(&response)?);
                    } else {
                        if response.code == 0 {
                            println!("Effect deleted successfully");
                        } else {
                            println!("Error: {} (code: {})", response.desc, response.code);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
