# Packaging the Python worker as a Tauri sidecar

Tauri ships the Rust + frontend as a single installable binary. The
Python worker does not live inside Cargo, so it needs a separate
packaging step to become a true sidecar executable.

## One-time setup

Install PyInstaller inside the worker venv:

```bash
cd worker
python -m venv .venv
.venv\Scripts\activate
pip install -r requirements.txt
pip install pyinstaller
```

## Build the sidecar binary

```bash
cd worker
pyinstaller \
  --onefile \
  --name signalgraph-worker \
  --distpath ../src-tauri/binaries \
  --workpath build \
  --specpath build \
  main.py
```

PyInstaller will produce `src-tauri/binaries/signalgraph-worker`
(or `.exe` on Windows). Tauri expects the binary name to include the
target triple suffix for reproducible cross-building - see
<https://tauri.app/develop/sidecar/> for the exact naming convention.
On Windows the simplest option is to produce
`signalgraph-worker-x86_64-pc-windows-msvc.exe`.

## Wire it into Tauri

Uncomment the `externalBin` block in `src-tauri/tauri.conf.json`:

```jsonc
{
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": ["icons/icon.png"],
    "externalBin": [
      "binaries/signalgraph-worker"
    ]
  }
}
```

Rebuild:

```bash
pnpm tauri build
```

At runtime, `src-tauri/src/worker.rs` looks for
`binaries/signalgraph-worker(.exe)` first and falls back to
`python worker/main.py` if the sidecar is not present, so the same
code works in dev and release.

## Verifying the packaged build

1. Double-click the installer from `src-tauri/target/release/bundle/`.
2. Install. Launch.
3. Run the full acceptance demo in `docs/DEMO.md`.
4. Confirm the status strip shows `worker running` - this means the
   bundled sidecar started, not the dev fallback.
5. Close the app. Re-open. The last selected camera should still be
   there (stored in localStorage inside the webview).

## Known gotchas

- MediaPipe ships large native libraries. Expect the PyInstaller
  bundle to clock in around 200 MB. Use `--strip` and
  `--exclude-module` to trim it if size matters.
- The first `cv2.VideoCapture` call on macOS triggers a camera
  permission dialog. Make sure the bundled app has the required
  Info.plist entitlement.
- On Linux, the bundled Python still needs `libgl1`/`libglib` at runtime
  if the user's system is minimal. Document this in the installer
  README.
