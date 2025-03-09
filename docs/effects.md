# Trimlight Effects

This document provides a complete list of effects available in the Trimlight API.

## Built-in Effects (0-179)

### Rainbow Effects (0-9)
- 0: Rainbow Gradual Chase
- 1: Rainbow Comet
- 2: Rainbow Segment
- 3: Rainbow Wave
- 4: Rainbow Meteor
- 5: Rainbow Gradual
- 6: Rainbow Jump
- 7: Rainbow Stars
- 8: Rainbow Fade In Out
- 9: Rainbow Spin

### Stacking Effects (10-23)
- 10: Red Stacking
- 11: Green Stacking
- 12: Blue Stacking
- 13: Yellow Stacking
- 14: Cyan Stacking
- 15: Purple Stacking
- 16: White Stacking
- 17: Full Color Stack
- 18: Red to Green Stack
- 19: Green to Blue Stack
- 20: Blue to Yellow Stack
- 21: Yellow to Cyan Stack
- 22: Cyan to Purple Stack
- 23: Purple to White Stack

### Comet Effects (24-30)
- 24: Red Comet
- 25: Green Comet
- 26: Blue Comet
- 27: Yellow Comet
- 28: Cyan Comet
- 29: Purple Comet
- 30: White Comet

### Meteor Effects (31-37)
- 31: Red Meteor
- 32: Green Meteor
- 33: Blue Meteor
- 34: Yellow Meteor
- 35: Cyan Meteor
- 36: Purple Meteor
- 37: White Meteor

### Wave Effects (38-65)
- 38-44: Single Color Waves (Red through White)
- 45-65: Two-Color Wave Combinations

### Dot Pulse Effects (66-84)
- 66-72: Single Color Dot Pulses
- 73-78: Two-Color Blank Pulses
- 79-84: Color with Secondary Pulses

### Comet Spin Effects (85-91)
- 85: Red Comet Spin
- 86: Green Comet Spin
- 87: Blue Comet Spin
- 88: Yellow Comet Spin
- 89: Cyan Comet Spin
- 90: Purple Comet Spin
- 91: White Comet Spin

### Dot Spin Effects (92-98)
- 92: Red Dot Spin
- 93: Green Dot Spin
- 94: Blue Dot Spin
- 95: Yellow Dot Spin
- 96: Cyan Dot Spin
- 97: Purple Dot Spin
- 98: White Dot Spin

### Segment Spin Effects (99-105)
- 99-105: Single Color Segment Spins (Red through White)

### Gradual Snake Effects (106-126)
- 106-126: Two-Color Gradual Snake Combinations

### Snake Effects (127-137)
- 127-132: Color with White Blank Snake
- 133-137: Multi-Color Snake Combinations

### Stars Effects (138-157)
- 138-144: Single Color Stars
- 145-157: Background Stars with Various Color Combinations

### Breath Effects (158-164)
- 158: Red Breath
- 159: Green Breath
- 160: Blue Breath
- 161: Yellow Breath
- 162: Cyan Breath
- 163: Purple Breath
- 164: White Breath

### Fire Effects (165-170)
- 165: Red Yellow Fire
- 166: Red Purple Fire
- 167: Green Yellow Fire
- 168: Green Cyan Fire
- 169: Blue Purple Fire
- 170: Blue Cyan Fire

### Strobe Effects (171-179)
- 171-177: Single Color Strobes
- 178: Red Blue White Strobe
- 179: Full Color Strobe

## Custom Effects (0-16)

These effects provide pixel-by-pixel control for creating custom animations:

- 0: Static
  - Single color, no animation
- 1: Chase Forward
  - Pattern moves forward along the strip
- 2: Chase Backward
  - Pattern moves backward along the strip
- 3: Chase Middle to Out
  - Pattern moves from center outward
- 4: Chase Out to Middle
  - Pattern moves from ends to center
- 5: Stars
  - Random twinkling effect
- 6: Breath
  - Smooth fade in and out
- 7: Comet Forward
  - Comet effect moving forward
- 8: Comet Backward
  - Comet effect moving backward
- 9: Comet Middle to Out
  - Comet effect from center outward
- 10: Comet Out to Middle
  - Comet effect from ends to center
- 11: Wave Forward
  - Wave pattern moving forward
- 12: Wave Backward
  - Wave pattern moving backward
- 13: Wave Middle to Out
  - Wave pattern from center outward
- 14: Wave Out to Middle
  - Wave pattern from ends to center
- 15: Strobe
  - Quick on/off flashing
- 16: Solid Fade
  - Smooth transition between colors

## Effect Parameters

### Common Parameters
All effects (both built-in and custom) support these parameters:
- `speed` (0-255): Animation speed
  - 0: Slowest
  - 255: Fastest
- `brightness` (0-255): LED brightness
  - 0: Off
  - 255: Maximum brightness

### Built-in Effect Parameters
Built-in effects (modes 0-179) support these additional parameters:
- `pixel_len` (1-90): Number of LEDs in the effect
- `reverse` (boolean): Reverse the animation direction

### Custom Effect Parameters
Custom effects (modes 0-16) support this additional parameter:
- `pixels` (array): Array of RGB values for pixel-by-pixel control
  - Each element is an object with `r`, `g`, and `b` values (0-255)
  - Array length must match the number of LEDs in your device
  - Example: `[{"r":255,"g":0,"b":0}, {"r":0,"g":255,"b":0}]`
