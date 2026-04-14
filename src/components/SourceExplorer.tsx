import { useMemo, useState } from "react";
import { useAppStore } from "../lib/store";

export function SourceExplorer() {
  const signals = useAppStore((s) => s.signals);
  const [filter, setFilter] = useState("");

  const filtered = useMemo(
    () =>
      signals.filter((s) =>
        s.path.toLowerCase().includes(filter.toLowerCase()),
      ),
    [signals, filter],
  );

  return (
    <div className="source-explorer">
      <input
        type="text"
        placeholder="filter signal paths"
        value={filter}
        onChange={(e) => setFilter(e.target.value)}
      />
      <div className="source-count muted">
        {filtered.length} / {signals.length} signals
      </div>
      <ul className="source-list">
        {filtered.map((s) => (
          <li key={s.path} className="source-row">
            <span className={`kind kind-${s.kind}`}>{s.kind}</span>
            <span className="path">{s.path}</span>
            <span className="value">{formatValue(s.last_value)}</span>
          </li>
        ))}
        {filtered.length === 0 ? (
          <li className="muted empty">no signals match</li>
        ) : null}
      </ul>
    </div>
  );
}

function formatValue(v: unknown): string {
  if (v === null || v === undefined) return "–";
  if (typeof v === "number") return v.toFixed(4);
  if (typeof v === "boolean") return v ? "true" : "false";
  if (typeof v === "string") return v;
  return JSON.stringify(v);
}
