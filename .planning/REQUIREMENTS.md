# REQUIREMENTS

## Scope policy
- `V1` requirements are mandatory for the first releasable build.
- `V1.5` requirements are stretch items that may ship if earlier phases land cleanly.
- `V2` items are intentionally deferred.

## V1 Requirements

### Runtime and platform
- **REQ-V1-001**: The project must ship as a desktop app built on Tauri with a Rust core and an external tracking worker.
- **REQ-V1-002**: The app must run on at least one primary desktop platform end-to-end before cross-platform polish begins.
- **REQ-V1-003**: The app must expose structured logs and surface fatal worker/runtime failures in the UI.

### Camera and tracking
- **REQ-V1-004**: The app must provide a live camera preview.
- **REQ-V1-005**: The user must be able to select a camera device.
- **REQ-V1-006**: The worker must run at least one live MediaPipe tracker profile.
- **REQ-V1-007**: Tracker results must be adapted into stable, named internal signals.

### Visual graphing
- **REQ-V1-008**: The app must include a node-based editor using React Flow.
- **REQ-V1-009**: The editor must support at least these node families:
  - Source
  - Transform
  - Output
  - Debug / Monitor
- **REQ-V1-010**: The graph must be persisted as a serializable patch document.
- **REQ-V1-011**: Invalid graphs must be blocked before runtime evaluation.

### Mapping and signal processing
- **REQ-V1-012**: The runtime must support at least these transform nodes:
  - map_range
  - clamp
  - invert
  - add
  - multiply
  - smooth
  - deadzone
  - threshold
- **REQ-V1-013**: The runtime must evaluate the graph in deterministic topological order.
- **REQ-V1-014**: The runtime must expose live node output values for debugging.

### OSC output
- **REQ-V1-015**: The app must send OSC over UDP to a configurable host and port.
- **REQ-V1-016**: The user must be able to configure a specific OSC address per output node.
- **REQ-V1-017**: The app must support at least float, int, and bool OSC payload types.
- **REQ-V1-018**: The app must include a packet monitor or equivalent debug surface.

### Persistence and usability
- **REQ-V1-019**: The user must be able to save, load, duplicate, rename, import, and export patches.
- **REQ-V1-020**: The app must provide calibration support for learned input ranges or manual input ranges.
- **REQ-V1-021**: The app must ship with at least one starter patch template.

### Quality and release
- **REQ-V1-022**: The project must include automated tests for mapping logic and OSC encoding/output.
- **REQ-V1-023**: The project must include replayable test fixtures for tracker data.
- **REQ-V1-024**: The packaged app must bundle the tracker worker as a sidecar-compatible executable.
- **REQ-V1-025**: The release must include setup docs and a troubleshooting guide.

## V1.5 Requirements
- **REQ-V1.5-001**: Add a second tracker profile behind the same signal adapter contract.
- **REQ-V1.5-002**: Add additional transform nodes:
  - hysteresis
  - quantize
  - curve/ease
  - gate
  - hold
- **REQ-V1.5-003**: Add target presets for TouchDesigner and at least one additional OSC consumer.
- **REQ-V1.5-004**: Add runtime rate limiting and bundle controls per OSC output node.

## V2 Requirements
- **REQ-V2-001**: Multi-camera support
- **REQ-V2-002**: Face blendshape source nodes
- **REQ-V2-003**: Custom node/plugin SDK
- **REQ-V2-004**: Cyclic graphs or controlled feedback loops
- **REQ-V2-005**: Network target discovery
- **REQ-V2-006**: Collaborative patch editing
