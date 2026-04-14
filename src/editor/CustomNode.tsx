import { Handle, Position, type NodeProps } from "@xyflow/react";
import { NODE_SPECS } from "./nodeSpecs";
import type { PatchNode } from "./types";

export function CustomNode({ data, selected, id }: NodeProps<PatchNode>) {
  const spec = NODE_SPECS[data.kind];
  if (!spec) return <div className="patch-node">{data.kind}</div>;

  const live =
    data.liveValue !== undefined && data.liveValue !== null
      ? formatLive(data.liveValue)
      : null;

  return (
    <div
      className={`patch-node patch-node-${spec.category} ${selected ? "selected" : ""}`}
      data-node-id={id}
    >
      <div className="patch-node-header">
        <span className="patch-node-kind">{spec.kind}</span>
        <span className="patch-node-label">{data.label}</span>
      </div>
      <div className="patch-node-body">
        {spec.kind === "tracker_signal" ? (
          <div className="patch-node-caption">
            {String(data.config.signal_path ?? "–")}
          </div>
        ) : null}
        {spec.kind === "constant" ? (
          <div className="patch-node-caption">
            = {String(data.config.value ?? 0)}
          </div>
        ) : null}
        {spec.kind === "osc_out" ? (
          <div className="patch-node-caption">
            {String(data.config.address ?? "/?")}{" "}
            <span className="muted small">
              {String(data.config.host ?? "127.0.0.1")}:
              {String(data.config.port ?? "—")}
            </span>
          </div>
        ) : null}
        {live !== null ? <div className="patch-node-live">{live}</div> : null}
      </div>
      {spec.inputs.map((port, idx) => {
        const spacing = spec.inputs.length > 1 ? 100 / (spec.inputs.length + 1) : 50;
        const top = `${spacing * (idx + 1)}%`;
        return (
          <Handle
            key={port.id}
            type="target"
            position={Position.Left}
            id={port.id}
            style={{ top }}
            className={`handle handle-${port.type}`}
          >
            <span className="handle-label">{port.label}</span>
          </Handle>
        );
      })}
      {spec.outputs.map((port, idx) => {
        const spacing = spec.outputs.length > 1 ? 100 / (spec.outputs.length + 1) : 50;
        const top = `${spacing * (idx + 1)}%`;
        return (
          <Handle
            key={port.id}
            type="source"
            position={Position.Right}
            id={port.id}
            style={{ top }}
            className={`handle handle-${port.type}`}
          />
        );
      })}
    </div>
  );
}

function formatLive(v: unknown): string {
  if (typeof v === "number") return v.toFixed(4);
  if (typeof v === "boolean") return v ? "true" : "false";
  if (typeof v === "string") return v;
  if (v === null) return "null";
  return JSON.stringify(v);
}
