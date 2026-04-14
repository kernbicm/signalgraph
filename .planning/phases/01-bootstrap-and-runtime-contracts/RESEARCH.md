# RESEARCH

## Why this phase exists
The project has three runtimes that can drift apart fast:
- Tauri frontend
- Rust core
- Python worker

If the contracts are vague, every later phase wastes time translating assumptions instead of shipping features.

## Design choices to lock early
- Rust is the authoritative evaluator and sender.
- The worker emits compact telemetry, not app-specific OSC packets.
- The graph document must be serializable and versioned.
- The app needs a fake source so graphing and OSC work can continue without live inference.

## Suggested initial node taxonomy
- Source nodes: `tracker_signal`, `constant`, `manual_slider`
- Transform nodes: `map_range`, `clamp`, `smooth`
- Output nodes: `osc_out`, `debug_meter`

## Acceptance mindset
By the end of this phase, the app can be ugly. It cannot be vague.
