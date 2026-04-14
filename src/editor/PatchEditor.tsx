import { useCallback, useEffect, useRef, useState } from "react";
import {
  Background,
  Controls,
  MiniMap,
  ReactFlow,
  ReactFlowProvider,
  useReactFlow,
  type ReactFlowInstance,
} from "@xyflow/react";
import "@xyflow/react/dist/style.css";
import { NodePalette } from "./NodePalette";
import { NodeInspector } from "./NodeInspector";
import { CustomNode } from "./CustomNode";
import { NODE_ORDER } from "./nodeSpecs";
import { useEditorStore } from "./editorStore";
import { createStarterPatch } from "./starterPatch";
import { api, isTauri } from "../lib/tauri";
import { useAppStore } from "../lib/store";
import type { NodeKind } from "./types";
import type { PatchNode, PatchEdge } from "./types";

const nodeTypes: Record<string, typeof CustomNode> = Object.fromEntries(
  NODE_ORDER.map((k) => [k, CustomNode]),
);

export function PatchEditor() {
  return (
    <ReactFlowProvider>
      <PatchEditorInner />
    </ReactFlowProvider>
  );
}

function PatchEditorInner() {
  const nodes = useEditorStore((s) => s.nodes);
  const edges = useEditorStore((s) => s.edges);
  const onNodesChange = useEditorStore((s) => s.onNodesChange);
  const onEdgesChange = useEditorStore((s) => s.onEdgesChange);
  const onConnect = useEditorStore((s) => s.onConnect);
  const selectNode = useEditorStore((s) => s.selectNode);
  const patchName = useEditorStore((s) => s.patchName);
  const setPatchName = useEditorStore((s) => s.setPatchName);
  const loadFromDocument = useEditorStore((s) => s.loadFromDocument);
  const toDocument = useEditorStore((s) => s.toDocument);
  const reset = useEditorStore((s) => s.reset);
  const addNode = useEditorStore((s) => s.addNode);
  const setLiveValues = useEditorStore((s) => s.setLiveValues);
  const snapshot = useAppStore((s) => s.snapshot);

  const patchList = useAppStore((s) => s.patchList);
  const refreshPatchList = useAppStore((s) => s.refreshPatchList);

  const [saveStatus, setSaveStatus] = useState<string | null>(null);
  const [loadError, setLoadError] = useState<string | null>(null);

  const wrapperRef = useRef<HTMLDivElement | null>(null);
  const rfInstanceRef = useRef<ReactFlowInstance<PatchNode, PatchEdge> | null>(null);

  useEffect(() => {
    void refreshPatchList();
  }, [refreshPatchList]);

  useEffect(() => {
    if (snapshot?.node_outputs) {
      setLiveValues(snapshot.node_outputs);
    }
  }, [snapshot, setLiveValues]);

  const handleSave = async () => {
    setSaveStatus(null);
    try {
      const doc = toDocument();
      if (!isTauri()) {
        setSaveStatus("(dev) patch serialized");
        console.log(doc);
        return;
      }
      await api.savePatch(doc);
      setSaveStatus(`saved '${doc.name}'`);
      await refreshPatchList();
    } catch (e) {
      setSaveStatus(`save failed: ${e}`);
    }
  };

  const handleLoad = async (name: string) => {
    setLoadError(null);
    try {
      if (!isTauri()) {
        setLoadError("(dev) load unavailable outside Tauri");
        return;
      }
      const doc = await api.loadPatch(name);
      loadFromDocument(doc);
    } catch (e) {
      setLoadError(String(e));
    }
  };

  const handleNewStarter = () => {
    loadFromDocument(createStarterPatch());
  };

  const onDragOver = useCallback((event: React.DragEvent) => {
    event.preventDefault();
    event.dataTransfer.dropEffect = "move";
  }, []);

  const onDrop = useCallback(
    (event: React.DragEvent) => {
      event.preventDefault();
      const kind = event.dataTransfer.getData(
        "application/signalgraph.node",
      ) as NodeKind;
      if (!kind) return;
      const bounds = wrapperRef.current?.getBoundingClientRect();
      if (!bounds || !rfInstanceRef.current) return;
      const position = rfInstanceRef.current.screenToFlowPosition({
        x: event.clientX,
        y: event.clientY,
      });
      addNode(kind, position);
    },
    [addNode],
  );

  return (
    <div className="patch-editor">
      <aside className="palette-sidebar">
        <NodePalette />
      </aside>
      <div className="editor-center" ref={wrapperRef}>
        <div className="editor-toolbar">
          <input
            className="patch-name"
            value={patchName}
            onChange={(e) => setPatchName(e.target.value)}
            placeholder="patch name"
          />
          <button onClick={handleNewStarter}>starter patch</button>
          <button onClick={() => reset()}>new empty</button>
          <button className="primary" onClick={handleSave}>
            save
          </button>
          <select
            value=""
            onChange={(e) => {
              if (e.target.value) void handleLoad(e.target.value);
              e.currentTarget.value = "";
            }}
          >
            <option value="">load patch…</option>
            {patchList.map((n) => (
              <option key={n} value={n}>
                {n}
              </option>
            ))}
          </select>
          {saveStatus ? <span className="muted small">{saveStatus}</span> : null}
          {loadError ? <span className="err-row small">{loadError}</span> : null}
        </div>
        <div
          className="reactflow-wrapper"
          onDragOver={onDragOver}
          onDrop={onDrop}
        >
          <ReactFlow
            nodes={nodes}
            edges={edges}
            onNodesChange={onNodesChange}
            onEdgesChange={onEdgesChange}
            onConnect={onConnect}
            onInit={(inst) => {
              rfInstanceRef.current = inst;
            }}
            onNodeClick={(_e, node) => selectNode(node.id)}
            onPaneClick={() => selectNode(null)}
            nodeTypes={nodeTypes}
            fitView
            proOptions={{ hideAttribution: true }}
            defaultEdgeOptions={{ animated: false }}
          >
            <Background color="#303542" gap={18} />
            <MiniMap pannable zoomable />
            <Controls />
          </ReactFlow>
        </div>
        <RuntimeErrorDock />
      </div>
      <aside className="inspector-sidebar">
        <NodeInspector />
      </aside>
    </div>
  );
}

function RuntimeErrorDock() {
  const snapshot = useAppStore((s) => s.snapshot);
  const errors = snapshot?.errors ?? [];
  if (errors.length === 0) {
    return (
      <div className="runtime-dock ok-row small">runtime: no errors</div>
    );
  }
  return (
    <div className="runtime-dock err-row small">
      {errors.slice(0, 5).map((e, i) => (
        <div key={i}>{e}</div>
      ))}
    </div>
  );
}
