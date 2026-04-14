import { NODE_ORDER, NODE_SPECS } from "./nodeSpecs";
import type { NodeKind } from "./types";
import { useEditorStore } from "./editorStore";

const CATEGORIES: {
  id: "source" | "transform" | "output" | "debug";
  label: string;
}[] = [
  { id: "source", label: "Source" },
  { id: "transform", label: "Transform" },
  { id: "output", label: "Output" },
  { id: "debug", label: "Debug" },
];

export function NodePalette() {
  const addNode = useEditorStore((s) => s.addNode);

  return (
    <div className="node-palette">
      {CATEGORIES.map((cat) => {
        const kinds = NODE_ORDER.filter((k) => NODE_SPECS[k].category === cat.id);
        if (kinds.length === 0) return null;
        return (
          <div key={cat.id} className="palette-category">
            <h3>{cat.label}</h3>
            <div className="palette-items">
              {kinds.map((kind) => (
                <PaletteItem
                  key={kind}
                  kind={kind}
                  onAdd={() =>
                    addNode(kind, {
                      x: 80 + Math.random() * 80,
                      y: 80 + Math.random() * 80,
                    })
                  }
                />
              ))}
            </div>
          </div>
        );
      })}
    </div>
  );
}

function PaletteItem({ kind, onAdd }: { kind: NodeKind; onAdd: () => void }) {
  const spec = NODE_SPECS[kind];
  const handleDragStart = (e: React.DragEvent) => {
    e.dataTransfer.setData("application/signalgraph.node", kind);
    e.dataTransfer.effectAllowed = "move";
  };
  return (
    <button
      className="palette-item"
      draggable
      onDragStart={handleDragStart}
      onClick={onAdd}
      title={spec.description}
    >
      <span className="palette-kind">{spec.kind}</span>
      <span className="palette-label">{spec.label}</span>
    </button>
  );
}
