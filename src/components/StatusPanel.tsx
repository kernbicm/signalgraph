import { useAppStore } from "../lib/store";
import { api } from "../lib/tauri";

export function StatusPanel() {
  const status = useAppStore((s) => s.status);
  const tick = useAppStore((s) => s.tick);

  if (!status) {
    return <div className="muted">connecting…</div>;
  }

  const badgeClass = {
    stopped: "badge muted",
    starting: "badge warn",
    running: "badge ok",
    error: "badge err",
  }[status.worker_state];

  const sourceBadge = status.source_mode === "fake" ? "badge warn" : "badge ok";

  return (
    <div className="status-grid">
      <div className="row">
        <span className="label">worker</span>
        <span className={badgeClass}>{status.worker_state}</span>
      </div>
      <div className="row">
        <span className="label">source</span>
        <span className={sourceBadge}>{status.source_mode}</span>
      </div>
      <div className="row">
        <span className="label">signals</span>
        <span>{status.signals_count}</span>
      </div>
      <div className="row">
        <span className="label">last frame ms</span>
        <span>{status.last_frame_monotonic_ms ?? "–"}</span>
      </div>
      <div className="row">
        <span className="label">last osc ms</span>
        <span>{status.osc_last_send_monotonic_ms ?? "–"}</span>
      </div>
      <div className="row">
        <span className="label">patch</span>
        <span>{status.loaded_patch ?? "–"}</span>
      </div>
      {status.last_error ? (
        <div className="row err-row">
          <span className="label">error</span>
          <span className="error-text">{status.last_error}</span>
        </div>
      ) : null}

      <div className="row row-actions">
        <button
          onClick={async () => {
            await api.setSourceMode("fake");
            await tick();
          }}
        >
          use fake source
        </button>
        <button
          onClick={async () => {
            try {
              await api.startWorker(null);
            } catch (e) {
              console.warn("start worker failed", e);
            }
            await tick();
          }}
        >
          start worker
        </button>
        <button
          onClick={async () => {
            await api.stopWorker();
            await tick();
          }}
        >
          stop worker
        </button>
      </div>
    </div>
  );
}
