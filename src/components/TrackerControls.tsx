import { useState } from "react";
import { useAppStore } from "../lib/store";
import { api } from "../lib/tauri";

export function TrackerControls() {
  const status = useAppStore((s) => s.status);
  const tick = useAppStore((s) => s.tick);
  const [busy, setBusy] = useState(false);
  const [msg, setMsg] = useState<string | null>(null);
  const [err, setErr] = useState<string | null>(null);

  const run = async (fn: () => Promise<unknown>, label: string) => {
    setBusy(true);
    setMsg(null);
    setErr(null);
    try {
      await fn();
      setMsg(label);
    } catch (e) {
      setErr(String(e));
    } finally {
      setBusy(false);
      await tick();
    }
  };

  return (
    <div className="tracker-controls">
      <div className="row">
        <label>
          tracker profile
          <select defaultValue="hand" disabled>
            <option value="hand">MediaPipe Hands</option>
          </select>
        </label>
      </div>
      <p className="muted small">
        v1 ships one tracker profile well. Pose and face land in v1.5.
      </p>
      <div className="row row-actions">
        <button
          disabled={busy}
          onClick={() => void run(() => api.setSourceMode("fake"), "fake source")}
        >
          use fake source
        </button>
        <button
          disabled={busy}
          onClick={() =>
            void run(() => api.startWorker(null), "worker started")
          }
        >
          start tracker worker
        </button>
        <button
          disabled={busy}
          onClick={() => void run(() => api.stopWorker(), "worker stopped")}
        >
          stop tracker
        </button>
      </div>
      <div className="row small">
        {msg ? <span className="ok-row">✓ {msg}</span> : null}
        {err ? <span className="err-row">✗ {err}</span> : null}
      </div>
      <div className="row">
        <span className="label">worker state</span>
        <span>{status?.worker_state ?? "–"}</span>
      </div>
      <div className="row">
        <span className="label">source mode</span>
        <span>{status?.source_mode ?? "–"}</span>
      </div>
      <div className="row">
        <span className="label">last frame ms</span>
        <span>{status?.last_frame_monotonic_ms ?? "–"}</span>
      </div>
    </div>
  );
}
