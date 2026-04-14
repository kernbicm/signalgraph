# CONTEXT

## Goal
Create the repo, wire the Tauri shell to a sidecar-friendly worker model, define the internal data contracts, and add enough fake data + loopback infrastructure that future phases can move without needing a camera.

## Non-negotiables
- Keep the runtime authority in Rust.
- Keep the visual editor optional until the contracts are stable.
- Keep the worker replaceable; the UI and graph should not know MediaPipe internals.
- Add observability on day one: logs, status badges, and explicit worker state transitions.

## Preferred repo shape
- `src/` or `app/` for frontend
- `src-tauri/` for Rust core
- `worker/` for Python tracker process
- `shared/` or a Rust/TS schema generation strategy for shared contracts
- `fixtures/` for fake telemetry and replay packets

## Internal contracts to define now
- `TelemetryFrame`
- `TelemetrySource`
- `SignalPath`
- `SignalValue`
- `GraphDocument`
- `GraphNode`
- `GraphEdge`
- `OscTarget`
- `OscMessageDraft`
- `RuntimeStatus`

## Strong preference
Use a **fake telemetry producer** in this phase. If every later task depends on a webcam and MediaPipe, the plan will age like milk.
