# 02-UI-SPEC

## Screen
**Tracking Lab**

## Layout
- Left: camera preview, as large as the window comfortably allows.
- Right top: device selector, tracker selector, start/stop tracking controls.
- Right middle: live signal explorer with a searchable list of current source values.
- Right bottom: OSC test output panel with last sent address, payload, host/port, packet timestamp.

## Interaction rules
- Device changes require explicit confirmation if tracking is active.
- The UI must always show whether values are live, stale, or simulated.
- When tracking stops, meters freeze with a stale badge rather than instantly blanking out.

## Minimum components
- camera preview
- camera device dropdown
- tracker profile dropdown
- start/stop button
- source signal explorer
- one direct mapping control:
  - source selector
  - manual output range fields
  - OSC address field
  - host/port fields
  - enable/disable toggle

## Visual tone
- utilitarian and low-friction
- optimized for debugging
- no decorative chrome that hides whether packets are actually moving
