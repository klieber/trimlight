# Trimlight CLI

A command-line interface for controlling Trimlight LED devices. This tool provides easy access to device management and effect control features.

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/trimlight
cd trimlight

# Build the project
cargo build --release

# Optional: Install globally
cargo install --path .
```

## Configuration

The CLI requires API credentials to authenticate with the Trimlight service. Create a `.env` file in the project directory:

```env
TRIMLIGHT_CLIENT_ID=your_client_id
TRIMLIGHT_CLIENT_SECRET=your_client_secret
```

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

Preview an effect:
```bash
# Basic usage
trimlight-cli effect --mode 1

# Full customization
trimlight-cli effect --mode 1 --speed 150 --brightness 200 --pixel-len 45 --reverse

# Specify a particular device
trimlight-cli effect --device ID --mode 1
```

List and search effects:
```bash
# List all effects
trimlight-cli modes

# Search for effects
trimlight-cli modes --search rainbow

# Filter by category
trimlight-cli modes --built-in  # Show only built-in effects
trimlight-cli modes --custom    # Show only custom effects
```

### JSON Output

Add the `--json` flag to any command to get the raw JSON response:
```bash
trimlight-cli details --json
trimlight-cli switch --manual --json
```

## Effect Categories

### Built-in Effects (modes 0-179)
- Various pre-programmed effects including:
  - Rainbow patterns
  - Color stacking
  - Comets and meteors
  - Wave effects
  - Stars and pulses
  - Breathing effects
  - Fire effects
  - Strobes

### Custom Effects (modes 0-16)
- Pixel-by-pixel control effects:
  - Static colors
  - Chase patterns
  - Comet effects
  - Wave patterns
  - Stars
  - Breathing
  - Strobes
  - Solid fades

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

For detailed API documentation, see [API Client](docs/api-client.md).
