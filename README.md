# SignalGraph

Desktop creative-coding tool that turns live human-tracking data into OSC.

- **Tauri** shell with a **React Flow** editor (TypeScript)
- **Rust** core: orchestration, graph validation, graph runtime, OSC transport
- **Python** worker: MediaPipe tracker, compact telemetry over stdout

## Loop

`camera -> MediaPipe -> normalized signal -> graph transform -> OSC packet`

## Status

v1 is feature-complete across the six phases of `.planning/ROADMAP.md`.
The runtime has per-node math tests and a replay harness, the editor
ships a starter set of nodes, and the packaging story is documented.

## Layout

```
src/          React + TypeScript frontend (React Flow editor)
src-tauri/    Rust core (orchestration, runtime, OSC)
worker/       Python MediaPipe worker
shared/       JSON schemas shared between Rust and TypeScript
fixtures/     Replay fixtures used by runtime tests
scripts/      Packaging scripts for the worker sidecar
docs/         Setup, troubleshooting, demo, packaging
.planning/    GSD project plans, phase docs
```

## Docs

- [docs/SETUP.md](docs/SETUP.md) – dev install
- [docs/DEMO.md](docs/DEMO.md) – the v1 acceptance walkthrough
- [docs/TROUBLESHOOTING.md](docs/TROUBLESHOOTING.md) – fix the three
  dumbest first-run problems fast
- [docs/PACKAGING.md](docs/PACKAGING.md) – bundling the Python worker
  as a Tauri sidecar via PyInstaller

## Quick start

```bash
pnpm install
cd worker && python -m venv .venv && .venv/Scripts/activate \
  && pip install -r requirements.txt
cd ..
pnpm tauri dev
```

## Run the tests

```bash
# Rust: unit + replay + integration (runtime drives real UDP receiver)
cargo test --all --manifest-path src-tauri/Cargo.toml

# Frontend: typecheck + build
pnpm typecheck
pnpm build
```

CI runs the same checks on Linux and Windows - see
[`.github/workflows/ci.yml`](.github/workflows/ci.yml).

## License

Not yet declared.
