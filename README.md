# SignalGraph

Desktop creative-coding tool that turns live human-tracking data into OSC.

- **Tauri** shell with **React Flow** editor (TypeScript)
- **Rust** core: orchestration, graph validation, graph runtime, OSC transport
- **Python** worker: MediaPipe tracker, compact telemetry over stdout

## Loop

`camera -> MediaPipe -> normalized signal -> graph transform -> OSC packet`

## Status

Active development. See `.planning/ROADMAP.md` for phase structure.

## Layout

```
src/          React + TypeScript frontend (React Flow editor)
src-tauri/    Rust core (orchestration, runtime, OSC)
worker/       Python MediaPipe worker
shared/       JSON schemas shared between Rust and TypeScript
fixtures/     Replay fixtures for tests
.planning/    GSD project plans, phase docs
```

## Quick start (dev)

```bash
pnpm install
cd worker && python -m venv .venv && .venv/Scripts/activate && pip install -r requirements.txt
pnpm tauri dev
```

## Run tests

```bash
cargo test --manifest-path src-tauri/Cargo.toml
pnpm test
```

## License

Not yet declared.
