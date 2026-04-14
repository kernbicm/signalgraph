import { useEffect, useRef, useState } from "react";
import { useAppStore } from "../lib/store";
import { useEditorStore } from "./editorStore";
import { api } from "../lib/tauri";

interface CaptureState {
  active: boolean;
  signalPath: string | null;
  min: number;
  max: number;
  samples: number;
}

const initial: CaptureState = {
  active: false,
  signalPath: null,
  min: Infinity,
  max: -Infinity,
  samples: 0,
};

export function CalibrationPanel() {
  const nodes = useEditorStore((s) => s.nodes);
  const selectedNodeId = useEditorStore((s) => s.selectedNodeId);
  const updateNodeConfig = useEditorStore((s) => s.updateNodeConfig);
  const signals = useAppStore((s) => s.signals);

  const [capture, setCapture] = useState<CaptureState>(initial);
  const pollRef = useRef<number | null>(null);

  const node = nodes.find((n) => n.id === selectedNodeId);
  const targetPath =
    node?.data.kind === "tracker_signal"
      ? (node.data.config.signal_path as string | undefined)
      : node?.data.kind === "map_range"
        ? findUpstreamSignal(nodes, useEditorStore.getState().edges, node.id)
        : undefined;

  const livePath = targetPath ?? null;
  const liveValue = livePath
    ? (signals.find((s) => s.path === livePath)?.last_value as number | undefined)
    : undefined;

  useEffect(() => {
    if (!capture.active || !capture.signalPath) {
      if (pollRef.current) {
        window.clearInterval(pollRef.current);
        pollRef.current = null;
      }
      return;
    }
    pollRef.current = window.setInterval(() => {
      const descriptor = useAppStore
        .getState()
        .signals.find((s) => s.path === capture.signalPath);
      const v = descriptor?.last_value;
      if (typeof v === "number") {
        setCapture((prev) => ({
          ...prev,
          min: Math.min(prev.min, v),
          max: Math.max(prev.max, v),
          samples: prev.samples + 1,
        }));
      }
    }, 33);
    return () => {
      if (pollRef.current) {
        window.clearInterval(pollRef.current);
        pollRef.current = null;
      }
    };
  }, [capture.active, capture.signalPath]);

  const start = () => {
    if (!livePath) return;
    setCapture({
      active: true,
      signalPath: livePath,
      min: Infinity,
      max: -Infinity,
      samples: 0,
    });
    void api.calibrateStart(livePath).catch(() => {});
  };

  const stop = () => {
    setCapture((prev) => ({ ...prev, active: false }));
    void api.calibrateStop().catch(() => {});
  };

  const reset = () => setCapture(initial);

  const apply = () => {
    if (!node || node.data.kind !== "map_range") return;
    if (!Number.isFinite(capture.min) || !Number.isFinite(capture.max)) return;
    updateNodeConfig(node.id, {
      in_min: round(capture.min),
      in_max: round(capture.max),
    });
  };

  if (!node) return null;
  if (node.data.kind !== "tracker_signal" && node.data.kind !== "map_range") {
    return null;
  }

  return (
    <div className="calibration-panel">
      <div className="calibration-header">
        <span className="inspector-kind">calibration</span>
      </div>
      <div className="row small">
        <span className="label">source signal</span>
        <span className="value">{livePath ?? "–"}</span>
      </div>
      <div className="row small">
        <span className="label">live</span>
        <span className="value">
          {typeof liveValue === "number" ? liveValue.toFixed(4) : "–"}
        </span>
      </div>
      <div className="row small">
        <span className="label">captured</span>
        <span className="value">
          {Number.isFinite(capture.min) ? capture.min.toFixed(4) : "–"} →{" "}
          {Number.isFinite(capture.max) ? capture.max.toFixed(4) : "–"}
        </span>
      </div>
      <div className="row small">
        <span className="label">samples</span>
        <span className="value">{capture.samples}</span>
      </div>
      <div className="row row-actions">
        {capture.active ? (
          <button className="danger" onClick={stop}>
            stop capture
          </button>
        ) : (
          <button onClick={start} disabled={!livePath}>
            start capture
          </button>
        )}
        <button onClick={reset}>reset</button>
        {node.data.kind === "map_range" ? (
          <button
            className="primary"
            onClick={apply}
            disabled={
              !Number.isFinite(capture.min) || !Number.isFinite(capture.max)
            }
          >
            apply to map range
          </button>
        ) : null}
      </div>
    </div>
  );
}

function round(v: number): number {
  return Math.round(v * 10000) / 10000;
}

function findUpstreamSignal(
  nodes: { id: string; data: { kind: string; config: Record<string, unknown> } }[],
  edges: { source: string; target: string }[],
  nodeId: string,
  depth = 0,
): string | undefined {
  if (depth > 8) return undefined;
  const incoming = edges.filter((e) => e.target === nodeId);
  for (const edge of incoming) {
    const parent = nodes.find((n) => n.id === edge.source);
    if (!parent) continue;
    if (parent.data.kind === "tracker_signal") {
      return parent.data.config.signal_path as string | undefined;
    }
    const upstream = findUpstreamSignal(nodes, edges, parent.id, depth + 1);
    if (upstream) return upstream;
  }
  return undefined;
}
