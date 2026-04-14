# SignalGraph worker

Python sidecar that emits `TelemetryFrame` records on stdout.

## Dev

```bash
python -m venv .venv
.venv\Scripts\activate
pip install -r requirements.txt

# fake mode (deterministic, no camera)
python main.py

# live MediaPipe mode
python main.py --live --camera 0
```

## Protocol

Each line on stdout is one JSON object:

```json
{
  "schema_version": 1,
  "source": "worker.hand",
  "monotonic_ms": 12345,
  "signals": {
    "mp.hand.left.landmark.index_tip.x": {"kind": "float", "value": 0.42}
  }
}
```

stdout = frames only. stderr = human logs.
