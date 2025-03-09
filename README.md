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

List saved effects:
```bash
trimlight-cli effects list
```

Preview an effect:
```bash
# Basic usage
trimlight-cli effects preview --mode 1

# Full customization
trimlight-cli effects preview --mode 1 --speed 150 --brightness 200 --pixel-len 45 --reverse

# Specify a particular device
trimlight-cli effects preview --device ID --mode 1
```

List and search available effect modes:
```bash
# List all modes
trimlight-cli effects modes

# Search for effects
trimlight-cli effects modes --search rainbow

# Filter by category
trimlight-cli effects modes --built-in  # Show only built-in effects
trimlight-cli effects modes --custom    # Show only custom effects
```

Manage custom effects:
```bash
# Add a new effect
trimlight-cli effects add --name "My Effect" --mode 1

# Update an existing effect
trimlight-cli effects update --id 1 --name "New Name"

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
