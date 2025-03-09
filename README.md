[![CI](https://github.com/klieber/trimlight/actions/workflows/ci.yml/badge.svg)](https://github.com/klieber/trimlight/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/klieber/trimlight/branch/main/graph/badge.svg)](https://codecov.io/gh/klieber/trimlight)

# Trimlight CLI

A command-line interface for controlling Trimlight LED devices. This tool provides easy access to device management and effect control features.

## Installation

```bash
# Clone the repository
git clone https://github.com/klieber/trimlight
cd trimlight

# Build the project
cargo build --release

# Optional: Install globally
cargo install --path .
```

## Configuration

The CLI requires API credentials to authenticate with the Trimlight service. There are several ways to provide these credentials:

### Option 1: Environment Variables
Set the following environment variables in your shell's configuration file (e.g., `~/.bashrc`, `~/.zshrc`):
```bash
export TRIMLIGHT_CLIENT_ID=your_client_id
export TRIMLIGHT_CLIENT_SECRET=your_client_secret
```
After adding these lines, restart your terminal or run `source ~/.bashrc` (or `source ~/.zshrc`).

### Option 2: Project-specific .env File
If you're running the CLI from the project directory, you can create a `.env` file:
```env
TRIMLIGHT_CLIENT_ID=your_client_id
TRIMLIGHT_CLIENT_SECRET=your_client_secret
```

### Option 3: User-specific .env File
For a global installation, you can create a `.env` file in your home directory:
```bash
echo "TRIMLIGHT_CLIENT_ID=your_client_id" >> ~/.trimlight.env
echo "TRIMLIGHT_CLIENT_SECRET=your_client_secret" >> ~/.trimlight.env
```

The CLI will check these locations in the following order:
1. Environment variables
2. `.env` file in the current directory
3. `.trimlight.env` file in your home directory

## Usage

### Device Management

List all devices:
```bash
trimlight-cli list
```

Get device details:
```bash
trimlight-cli details              # Uses first available device
trimlight-cli details --device ID  # Specify a particular device
```

Change device state:
```bash
trimlight-cli switch --off     # Turn off
trimlight-cli switch --manual  # Turn on manual mode
trimlight-cli switch --timer   # Turn on timer mode
```

Rename a device:
```bash
trimlight-cli rename --name "New Name"              # Uses first available device
trimlight-cli rename --device ID --name "New Name"  # Specify a particular device
```

### Effect Control

List saved effects:
```bash
trimlight-cli effects list
```

View a saved effect:
```bash
# Load a saved effect by ID
trimlight-cli effects view --id 1

# Specify a particular device
trimlight-cli effects view --device ID --id 1
```

Preview an effect:
```bash
# Basic usage
trimlight-cli effects preview --built-in 1

# Full customization of a built-in effect
trimlight-cli effects preview --built-in 1 --speed 150 --brightness 200 --pixel-len 45 --reverse

# Preview a custom pattern with pixel data
trimlight-cli effects preview --pattern 1 --speed 150 --brightness 200 --pixels "255,0,0:1;0,255,0:2"

# Specify a particular device
trimlight-cli effects preview --device ID --built-in 1
```

List and search available effect modes:
```bash
# List all modes
trimlight-cli effects modes

# Search for effects
trimlight-cli effects modes --search rainbow

# Filter by category
trimlight-cli effects modes --built-in  # Show only built-in effects
trimlight-cli effects modes --pattern   # Show only custom patterns
```

Manage custom effects:
```bash
# Add a new effect
trimlight-cli effects add --name "My Effect" --pattern 1

# Add a custom effect with pixel data
trimlight-cli effects add --name "Custom Pattern" --pattern 1 --pixels "255,0,0:1;0,255,0:2"

# Update an existing effect
trimlight-cli effects update --id 1 --name "New Name"

# Update an effect with new pixel data
trimlight-cli effects update --id 1 --pixels "255,0,0:1;0,255,0:2"

# Delete an effect
trimlight-cli effects delete --id 1
```

Manage combined effects:
```bash
# Set a combined effect sequence
trimlight-cli effects combined set --effects 1,2,3 --interval 60

# Clear the combined effect sequence
trimlight-cli effects combined clear
```

Manage overlay effects:
```bash
# Add a lightning effect to an existing effect
trimlight-cli effects overlay add --lightning --target 1

# Add a snow effect to an existing effect
trimlight-cli effects overlay add --snow --target 2

# Clear all overlay effects
trimlight-cli effects overlay clear

# Specify a particular device
trimlight-cli effects overlay add --device ID --lightning --target 1
trimlight-cli effects overlay clear --device ID
```

### JSON Output

Add the `--json` flag to any command to get the raw JSON response:
```bash
trimlight-cli details --json
trimlight-cli switch --manual --json
```

## Effect Modes

The Trimlight system supports two types of effects:
- Built-in Effects (modes 0-179): Pre-programmed patterns including rainbows, comets, waves, and more
- Custom Effects (modes 0-16): Configurable effects with pixel-by-pixel control

### Pixel Data Format

When specifying pixel data for custom effects, use the following format:
```
R,G,B:count[:disabled];R,G,B:count[:disabled];...
```

Where:
- R,G,B: RGB color values (0-255)
- count: Number of consecutive pixels with this color (default: 1)
- disabled: Optional, 0=enabled, 1=disabled (default: 0)

Example:
```bash
# 1 red pixel, followed by 2 green pixels, followed by 1 disabled blue pixel
trimlight-cli effects add --name "Custom" --pattern 1 --pixels "255,0,0:1;0,255,0:2;0,0,255:1:1"
```

For a complete list of available effects and their descriptions, see [Effect Documentation](docs/effects.md).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

For detailed API documentation, see [API Client](docs/api-client.md).
