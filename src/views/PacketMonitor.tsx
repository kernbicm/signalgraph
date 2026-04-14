import { useMemo, useState } from "react";
import { useAppStore } from "../lib/store";
import type { PacketLogEntry } from "../lib/types";

export function PacketMonitor() {
  const packetLog = useAppStore((s) => s.packetLog);
  const snapshot = useAppStore((s) => s.snapshot);
  const [filter, setFilter] = useState("");
  const [onlyErrors, setOnlyErrors] = useState(false);

  const filtered = useMemo(() => {
    return packetLog
      .slice()
      .reverse()
      .filter((e) => {
        if (onlyErrors && e.sent) return false;
        if (!filter) return true;
        const f = filter.toLowerCase();
        return (
          e.address.toLowerCase().includes(f) ||
          e.host.toLowerCase().includes(f) ||
          (e.source_label?.toLowerCase().includes(f) ?? false)
        );
      });
  }, [packetLog, filter, onlyErrors]);

  const sent = packetLog.filter((e) => e.sent).length;
  const failed = packetLog.length - sent;

  return (
    <div className="packet-monitor">
      <section className="panel packet-sinks">
        <h2>Active OSC sinks</h2>
        {snapshot?.sinks.length ? (
          <ul className="sink-list">
            {snapshot.sinks.map((s) => (
              <li key={s.node_id} className={`sink-row ${s.enabled ? "" : "disabled"}`}>
                <span className={`badge ${s.enabled ? "ok" : "muted"}`}>
                  {s.enabled ? "on" : "off"}
                </span>
                <span className="sink-label">{s.label}</span>
                <span className="sink-addr">
                  {s.host}:{s.port}
                  <span className="muted"> {s.address}</span>
                </span>
                <span className="sink-value">
                  {formatSinkValue(s.last_value)}
                </span>
              </li>
            ))}
          </ul>
        ) : (
          <p className="muted small">
            No patch loaded, or no OSC sink nodes in the current patch.
          </p>
        )}
      </section>

      <section className="panel packet-stream">
        <div className="packet-header">
          <h2>Packet stream</h2>
          <div className="packet-summary small muted">
            {packetLog.length} total · {sent} sent ·{" "}
            <span className={failed > 0 ? "err-row" : ""}>{failed} failed</span>
          </div>
        </div>
        <div className="row">
          <input
            className="packet-filter"
            placeholder="filter address/host/label"
            value={filter}
            onChange={(e) => setFilter(e.target.value)}
          />
          <label className="small">
            <input
              type="checkbox"
              checked={onlyErrors}
              onChange={(e) => setOnlyErrors(e.target.checked)}
            />
            only failures
          </label>
        </div>
        <div className="packet-list">
          {filtered.length === 0 ? (
            <p className="muted empty">No packets match.</p>
          ) : (
            filtered.map((entry, i) => <PacketRow key={i} entry={entry} />)
          )}
        </div>
      </section>
    </div>
  );
}

function PacketRow({ entry }: { entry: PacketLogEntry }) {
  return (
    <div className={`packet-row ${entry.sent ? "packet-ok" : "packet-err"}`}>
      <span className="packet-time">{entry.monotonic_ms}</span>
      <span className="packet-host">
        {entry.host}:{entry.port}
      </span>
      <span className="packet-addr">{entry.address}</span>
      <span className="packet-payload">{entry.payload_preview}</span>
      <span className="packet-source muted small">
        {entry.source_label ?? "–"}
      </span>
      {entry.error ? (
        <span className="packet-error err-row small">{entry.error}</span>
      ) : null}
    </div>
  );
}

function formatSinkValue(v: unknown): string {
  if (v === null || v === undefined) return "–";
  if (typeof v === "number") return v.toFixed(4);
  if (typeof v === "boolean") return v ? "true" : "false";
  if (typeof v === "string") return v;
  return JSON.stringify(v);
}
