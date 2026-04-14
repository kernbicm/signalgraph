# SignalGraph setup

This page walks you from a fresh clone to a running SignalGraph dev build
on Windows, macOS, or Linux.

## Prerequisites

| Tool                           | Version     | Notes                                           |
| ------------------------------ | ----------- | ----------------------------------------------- |
| Rust (stable)                  | 1.82+       | install via <https://rustup.rs/>                |
| Node                           | 20+         | ships pnpm via `corepack enable`                |
| pnpm                           | 9+          | or `npm`/`yarn`, adjust commands accordingly    |
| Python                         | 3.10+       | MediaPipe currently ships wheels for 3.10–3.13  |
| Tauri platform prerequisites   | see below   | C toolchain, WebView2 on Windows                |

### Platform prerequisites for Tauri

- **Windows**: the Microsoft C++ Build Tools and WebView2 runtime. See
  <https://tauri.app/start/prerequisites/>.
- **macOS**: Xcode command line tools (`xcode-select --install`).
- **Linux**: `webkit2gtk` and friends; follow the Tauri distro-specific
  instructions.

## Clone + install

```bash
git clone https://github.com/kernbicm/signalgraph.git
cd signalgraph
pnpm install
```

The Rust side fetches and builds on first run of `pnpm tauri dev`. On a
cold cache expect 5–10 minutes for the first Tauri compile.

## Python worker

The worker runs as a sidecar process in dev mode. Create a virtualenv
and install the two dependencies:

```bash
cd worker
python -m venv .venv
.venv\Scripts\activate            # macOS/Linux: source .venv/bin/activate
pip install -r requirements.txt
```

A quick smoke test to make sure the worker emits valid JSON frames on
stdout:

```bash
python main.py --fps 5
# stop with Ctrl-C
```

## Run the app (dev)

From the repo root:

```bash
pnpm tauri dev
```

The first invocation compiles the entire Rust workspace; subsequent
runs hot-reload frontend changes and recompile only touched Rust files.

### First five minutes inside the app

1. Open the **Lab** tab.
2. Grant camera permission when prompted.
3. Pick a camera from the dropdown (the last one is remembered).
4. Click **start tracker worker** to kick off the Python sidecar.
5. In the source explorer on the right, confirm that
   `mp.hand.left.landmark.index_tip.x` starts updating.
6. In the **hardcoded mapping** panel: enable the mapping, point your
   hand at the camera, and verify the packet monitor (Monitor tab) is
   filling up.
7. Start a local OSC receiver to observe the packets, e.g.
   [Protokol](https://hexler.net/protokol) or
   `oscdump 9000` from liblo.

## Build a release bundle

```bash
pnpm tauri build
```

The bundle lands in `src-tauri/target/release/bundle/`. See
`docs/PACKAGING.md` for the Python worker sidecar story - you must
produce a standalone binary (e.g. via PyInstaller) and wire it through
`tauri.conf.json` → `bundle.externalBin` before shipping.

## Running the tests

```bash
# Rust side (runtime math + OSC loopback + replay)
cargo test --manifest-path src-tauri/Cargo.toml

# Frontend typecheck + build
pnpm typecheck
pnpm build
```
