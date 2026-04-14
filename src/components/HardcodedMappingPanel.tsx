import { useEffect, useRef, useState } from "react";
import { api } from "../lib/tauri";
import type { HardcodedMapping, OscPayloadType } from "../lib/types";
import { useAppStore } from "../lib/store";

export function HardcodedMappingPanel() {
  const signals = useAppStore((s) => s.signals);
  const [mapping, setMapping] = useState<HardcodedMapping | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [status, setStatus] = useState<string | null>(null);
  const sendTimer = useRef<number | null>(null);

  useEffect(() => {
    void api.getHardcodedMapping().then(setMapping).catch(console.error);
  }, []);

  useEffect(() => {
    if (!mapping?.enabled) {
      if (sendTimer.current) {
        window.clearInterval(sendTimer.current);
        sendTimer.current = null;
      }
      return;
    }
    sendTimer.current = window.setInterval(async () => {
      try {
        const msg = await api.sendHardcodedMapping();
        setStatus(msg);
        setError(null);
      } catch (e) {
        setError(String(e));
      }
    }, 66);
    return () => {
      if (sendTimer.current) {
        window.clearInterval(sendTimer.current);
        sendTimer.current = null;
      }
    };
  }, [mapping?.enabled]);

  if (!mapping) {
    return <div className="muted">loading mapping…</div>;
  }

  const update = (patch: Partial<HardcodedMapping>) => {
    const next = { ...mapping, ...patch };
    setMapping(next);
    void api.setHardcodedMapping(next).catch((e) => setError(String(e)));
  };
  const updateTarget = (patch: Partial<HardcodedMapping["target"]>) => {
    update({ target: { ...mapping.target, ...patch } });
  };

  const numericSources = signals
    .filter((s) => s.kind === "float" || s.kind === "int")
    .map((s) => s.path);

  const currentValue = signals.find((s) => s.path === mapping.source)?.last_value;

  return (
    <div className="hardcoded-panel">
      <div className="row">
        <label>
          <input
            type="checkbox"
            checked={mapping.enabled}
            onChange={(e) => update({ enabled: e.target.checked })}
          />
          enabled
        </label>
        <span className="badge muted">regression path</span>
      </div>

      <label className="field">
        <span>source signal</span>
        <select
          value={mapping.source}
          onChange={(e) => update({ source: e.target.value })}
        >
          {numericSources.length === 0 ? (
            <option value={mapping.source}>{mapping.source}</option>
          ) : null}
          {numericSources.map((p) => (
            <option key={p} value={p}>
              {p}
            </option>
          ))}
        </select>
      </label>
      <div className="row">
        <span className="label">live value</span>
        <span className="value">
          {typeof currentValue === "number"
            ? currentValue.toFixed(4)
            : String(currentValue ?? "–")}
        </span>
      </div>

      <div className="grid-2">
        <label className="field">
          <span>in min</span>
          <input
            type="number"
            step="0.01"
            value={mapping.in_min}
            onChange={(e) => update({ in_min: Number(e.target.value) })}
          />
        </label>
        <label className="field">
          <span>in max</span>
          <input
            type="number"
            step="0.01"
            value={mapping.in_max}
            onChange={(e) => update({ in_max: Number(e.target.value) })}
          />
        </label>
        <label className="field">
          <span>out min</span>
          <input
            type="number"
            step="0.01"
            value={mapping.out_min}
            onChange={(e) => update({ out_min: Number(e.target.value) })}
          />
        </label>
        <label className="field">
          <span>out max</span>
          <input
            type="number"
            step="0.01"
            value={mapping.out_max}
            onChange={(e) => update({ out_max: Number(e.target.value) })}
          />
        </label>
      </div>

      <label>
        <input
          type="checkbox"
          checked={mapping.invert}
          onChange={(e) => update({ invert: e.target.checked })}
        />
        invert
      </label>

      <div className="grid-2">
        <label className="field">
          <span>host</span>
          <input
            value={mapping.target.host}
            onChange={(e) => updateTarget({ host: e.target.value })}
          />
        </label>
        <label className="field">
          <span>port</span>
          <input
            type="number"
            value={mapping.target.port}
            onChange={(e) =>
              updateTarget({ port: Number(e.target.value) || 0 })
            }
          />
        </label>
      </div>
      <label className="field">
        <span>osc address</span>
        <input
          value={mapping.target.address}
          onChange={(e) => updateTarget({ address: e.target.value })}
        />
      </label>
      <label className="field">
        <span>payload type</span>
        <select
          value={mapping.payload_type}
          onChange={(e) =>
            update({ payload_type: e.target.value as OscPayloadType })
          }
        >
          <option value="float">float</option>
          <option value="int">int</option>
          <option value="bool">bool</option>
        </select>
      </label>

      {status ? <div className="ok-row small">{status}</div> : null}
      {error ? <div className="err-row small">{error}</div> : null}
    </div>
  );
}
