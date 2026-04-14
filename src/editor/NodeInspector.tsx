import { useEditorStore } from "./editorStore";
import { NODE_SPECS } from "./nodeSpecs";
import { useAppStore } from "../lib/store";

export function NodeInspector() {
  const selectedId = useEditorStore((s) => s.selectedNodeId);
  const nodes = useEditorStore((s) => s.nodes);
  const updateNodeConfig = useEditorStore((s) => s.updateNodeConfig);
  const updateNodeLabel = useEditorStore((s) => s.updateNodeLabel);
  const deleteNode = useEditorStore((s) => s.deleteNode);
  const signals = useAppStore((s) => s.signals);

  const node = nodes.find((n) => n.id === selectedId);
  if (!node) {
    return (
      <div className="inspector muted small">
        Select a node to edit its configuration.
      </div>
    );
  }
  const spec = NODE_SPECS[node.data.kind];
  const config = node.data.config;

  const set = (key: string, value: unknown) =>
    updateNodeConfig(node.id, { [key]: value });
  const num = (k: string) =>
    typeof config[k] === "number" ? (config[k] as number) : 0;
  const str = (k: string, fallback = "") =>
    typeof config[k] === "string" ? (config[k] as string) : fallback;
  const bool = (k: string) =>
    typeof config[k] === "boolean" ? (config[k] as boolean) : false;

  return (
    <div className="inspector">
      <div className="inspector-header">
        <span className="inspector-kind">{spec.kind}</span>
        <button className="danger" onClick={() => deleteNode(node.id)}>
          delete
        </button>
      </div>

      <label className="field">
        <span>label</span>
        <input
          value={node.data.label}
          onChange={(e) => updateNodeLabel(node.id, e.target.value)}
        />
      </label>

      <p className="muted small">{spec.description}</p>

      {spec.kind === "tracker_signal" ? (
        <label className="field">
          <span>signal path</span>
          <select
            value={str("signal_path")}
            onChange={(e) => set("signal_path", e.target.value)}
          >
            {!signals.find((s) => s.path === str("signal_path")) && str("signal_path") ? (
              <option value={str("signal_path")}>{str("signal_path")}</option>
            ) : null}
            {signals.map((s) => (
              <option key={s.path} value={s.path}>
                {s.path}
              </option>
            ))}
          </select>
        </label>
      ) : null}

      {spec.kind === "constant" ? (
        <label className="field">
          <span>value</span>
          <input
            type="number"
            step="0.01"
            value={num("value")}
            onChange={(e) => set("value", Number(e.target.value))}
          />
        </label>
      ) : null}

      {spec.kind === "map_range" ? (
        <>
          <div className="grid-2">
            <NumberField label="in min" value={num("in_min")} onChange={(v) => set("in_min", v)} />
            <NumberField label="in max" value={num("in_max")} onChange={(v) => set("in_max", v)} />
            <NumberField label="out min" value={num("out_min")} onChange={(v) => set("out_min", v)} />
            <NumberField label="out max" value={num("out_max")} onChange={(v) => set("out_max", v)} />
          </div>
          <label>
            <input
              type="checkbox"
              checked={bool("clamp")}
              onChange={(e) => set("clamp", e.target.checked)}
            />
            clamp
          </label>
          <label>
            <input
              type="checkbox"
              checked={bool("invert")}
              onChange={(e) => set("invert", e.target.checked)}
            />
            invert
          </label>
        </>
      ) : null}

      {spec.kind === "clamp" || spec.kind === "invert" ? (
        <div className="grid-2">
          <NumberField label="min" value={num("min")} onChange={(v) => set("min", v)} />
          <NumberField label="max" value={num("max")} onChange={(v) => set("max", v)} />
        </div>
      ) : null}

      {spec.kind === "smooth" ? (
        <NumberField
          label="alpha"
          value={num("alpha")}
          onChange={(v) => set("alpha", v)}
          step={0.01}
        />
      ) : null}

      {spec.kind === "deadzone" ? (
        <div className="grid-2">
          <NumberField label="center" value={num("center")} onChange={(v) => set("center", v)} step={0.01} />
          <NumberField label="radius" value={num("radius")} onChange={(v) => set("radius", v)} step={0.01} />
        </div>
      ) : null}

      {spec.kind === "threshold" ? (
        <NumberField
          label="threshold"
          value={num("threshold")}
          onChange={(v) => set("threshold", v)}
          step={0.01}
        />
      ) : null}

      {spec.kind === "osc_out" ? (
        <>
          <div className="grid-2">
            <label className="field">
              <span>host</span>
              <input
                value={str("host", "127.0.0.1")}
                onChange={(e) => set("host", e.target.value)}
              />
            </label>
            <label className="field">
              <span>port</span>
              <input
                type="number"
                value={num("port") || 9000}
                onChange={(e) => set("port", Number(e.target.value) || 0)}
              />
            </label>
          </div>
          <label className="field">
            <span>address</span>
            <input
              value={str("address")}
              onChange={(e) => set("address", e.target.value)}
            />
          </label>
          <label className="field">
            <span>payload type</span>
            <select
              value={str("payload_type", "float")}
              onChange={(e) => set("payload_type", e.target.value)}
            >
              <option value="float">float</option>
              <option value="int">int</option>
              <option value="bool">bool</option>
            </select>
          </label>
          <label>
            <input
              type="checkbox"
              checked={bool("enabled")}
              onChange={(e) => set("enabled", e.target.checked)}
            />
            enabled
          </label>
        </>
      ) : null}
    </div>
  );
}

function NumberField({
  label,
  value,
  onChange,
  step = 0.01,
}: {
  label: string;
  value: number;
  onChange: (v: number) => void;
  step?: number;
}) {
  return (
    <label className="field">
      <span>{label}</span>
      <input
        type="number"
        step={step}
        value={value}
        onChange={(e) => onChange(Number(e.target.value))}
      />
    </label>
  );
}
