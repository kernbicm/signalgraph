import { create } from "zustand";
import type {
  Connection,
  EdgeChange,
  NodeChange,
} from "@xyflow/react";
import {
  addEdge,
  applyEdgeChanges,
  applyNodeChanges,
} from "@xyflow/react";
import { NODE_SPECS } from "./nodeSpecs";
import type { NodeKind, PatchEdge, PatchNode } from "./types";
import type { GraphDocument, GraphNode, GraphEdge } from "../lib/types";

interface EditorState {
  nodes: PatchNode[];
  edges: PatchEdge[];
  selectedNodeId: string | null;
  dirty: boolean;
  patchName: string;
  onNodesChange: (changes: NodeChange<PatchNode>[]) => void;
  onEdgesChange: (changes: EdgeChange<PatchEdge>[]) => void;
  onConnect: (connection: Connection) => void;
  addNode: (kind: NodeKind, position: { x: number; y: number }) => void;
  deleteNode: (id: string) => void;
  selectNode: (id: string | null) => void;
  updateNodeConfig: (
    id: string,
    patch: Record<string, unknown>,
  ) => void;
  updateNodeLabel: (id: string, label: string) => void;
  loadFromDocument: (doc: GraphDocument) => void;
  toDocument: () => GraphDocument;
  reset: () => void;
  setPatchName: (name: string) => void;
  setLiveValues: (values: Record<string, unknown>) => void;
}

let nextNodeId = 1;
const genNodeId = (kind: NodeKind) => `${kind}_${nextNodeId++}`;

function toDoc(
  nodes: PatchNode[],
  edges: PatchEdge[],
  name: string,
): GraphDocument {
  const now = new Date().toISOString();
  const docNodes: GraphNode[] = nodes.map((n) => ({
    id: n.id,
    kind: n.data.kind,
    label: n.data.label,
    config: n.data.config,
    position: [n.position.x, n.position.y],
  }));
  const docEdges: GraphEdge[] = edges.map((e) => ({
    id: e.id,
    source: e.source,
    source_port: e.sourceHandle || "out",
    target: e.target,
    target_port: e.targetHandle || "input",
  }));
  return {
    schema_version: 1,
    id: name,
    name,
    description: null,
    created_at: now,
    updated_at: now,
    nodes: docNodes,
    edges: docEdges,
  };
}

function fromDoc(doc: GraphDocument): {
  nodes: PatchNode[];
  edges: PatchEdge[];
} {
  let maxIdNum = 0;
  const nodes: PatchNode[] = doc.nodes.map((n) => {
    const match = n.id.match(/_(\d+)$/);
    if (match) {
      maxIdNum = Math.max(maxIdNum, Number(match[1]));
    }
    return {
      id: n.id,
      type: n.kind,
      position: n.position
        ? { x: n.position[0], y: n.position[1] }
        : { x: 0, y: 0 },
      data: {
        kind: n.kind as NodeKind,
        label: n.label ?? NODE_SPECS[n.kind as NodeKind]?.label ?? n.kind,
        config: n.config as Record<string, unknown>,
      },
    };
  });
  nextNodeId = maxIdNum + 1;
  const edges: PatchEdge[] = doc.edges.map((e) => ({
    id: e.id,
    source: e.source,
    sourceHandle: e.source_port,
    target: e.target,
    targetHandle: e.target_port,
  }));
  return { nodes, edges };
}

export const useEditorStore = create<EditorState>((set, get) => ({
  nodes: [],
  edges: [],
  selectedNodeId: null,
  dirty: false,
  patchName: "untitled",

  onNodesChange(changes) {
    set((s) => ({
      nodes: applyNodeChanges(changes, s.nodes),
      dirty: true,
    }));
  },
  onEdgesChange(changes) {
    set((s) => ({
      edges: applyEdgeChanges(changes, s.edges),
      dirty: true,
    }));
  },
  onConnect(connection) {
    if (!connection.source || !connection.target) return;
    if (connection.source === connection.target) return;
    // Guard against cycles by running a BFS reachability check.
    if (wouldCreateCycle(get().edges, connection.source, connection.target)) {
      console.warn("connection rejected: cycle");
      return;
    }
    // Enforce port type compatibility.
    const nodes = get().nodes;
    const srcNode = nodes.find((n) => n.id === connection.source);
    const dstNode = nodes.find((n) => n.id === connection.target);
    if (srcNode && dstNode) {
      const srcSpec = NODE_SPECS[srcNode.data.kind];
      const dstSpec = NODE_SPECS[dstNode.data.kind];
      const srcPort = srcSpec.outputs.find(
        (p) => p.id === (connection.sourceHandle || "out"),
      );
      const dstPort = dstSpec.inputs.find(
        (p) => p.id === (connection.targetHandle || "input"),
      );
      if (srcPort && dstPort && !typesCompatible(srcPort.type, dstPort.type)) {
        console.warn(
          `connection rejected: ${srcPort.type} -> ${dstPort.type}`,
        );
        return;
      }
    }
    // Replace any existing connection targeting the same input port
    // so single-input nodes don't accumulate dangling edges.
    const filtered = get().edges.filter(
      (e) =>
        !(
          e.target === connection.target &&
          (e.targetHandle || "input") === (connection.targetHandle || "input")
        ),
    );
    set({
      edges: addEdge(connection, filtered),
      dirty: true,
    });
  },
  addNode(kind, position) {
    const spec = NODE_SPECS[kind];
    const id = genNodeId(kind);
    const newNode: PatchNode = {
      id,
      type: kind,
      position,
      data: {
        kind,
        label: spec.label,
        config: { ...spec.defaultConfig },
      },
    };
    set((s) => ({
      nodes: [...s.nodes, newNode],
      selectedNodeId: id,
      dirty: true,
    }));
  },
  deleteNode(id) {
    set((s) => ({
      nodes: s.nodes.filter((n) => n.id !== id),
      edges: s.edges.filter((e) => e.source !== id && e.target !== id),
      selectedNodeId: s.selectedNodeId === id ? null : s.selectedNodeId,
      dirty: true,
    }));
  },
  selectNode(id) {
    set({ selectedNodeId: id });
  },
  updateNodeConfig(id, patch) {
    set((s) => ({
      nodes: s.nodes.map((n) =>
        n.id === id
          ? { ...n, data: { ...n.data, config: { ...n.data.config, ...patch } } }
          : n,
      ),
      dirty: true,
    }));
  },
  updateNodeLabel(id, label) {
    set((s) => ({
      nodes: s.nodes.map((n) =>
        n.id === id ? { ...n, data: { ...n.data, label } } : n,
      ),
      dirty: true,
    }));
  },
  loadFromDocument(doc) {
    const { nodes, edges } = fromDoc(doc);
    set({
      nodes,
      edges,
      dirty: false,
      patchName: doc.name,
      selectedNodeId: null,
    });
  },
  toDocument() {
    return toDoc(get().nodes, get().edges, get().patchName);
  },
  reset() {
    set({
      nodes: [],
      edges: [],
      selectedNodeId: null,
      dirty: false,
      patchName: "untitled",
    });
  },
  setPatchName(name) {
    set({ patchName: name, dirty: true });
  },
  setLiveValues(values) {
    set((s) => ({
      nodes: s.nodes.map((n) => {
        if (!(n.id in values)) return n;
        return { ...n, data: { ...n.data, liveValue: values[n.id] } };
      }),
    }));
  },
}));

function wouldCreateCycle(
  edges: PatchEdge[],
  newSource: string,
  newTarget: string,
): boolean {
  // Walk forward from `newTarget`; if we reach `newSource`, adding
  // newSource->newTarget closes a loop.
  const adj = new Map<string, string[]>();
  for (const e of edges) {
    adj.set(e.source, [...(adj.get(e.source) || []), e.target]);
  }
  const stack = [newTarget];
  const seen = new Set<string>();
  while (stack.length) {
    const cur = stack.pop()!;
    if (cur === newSource) return true;
    if (seen.has(cur)) continue;
    seen.add(cur);
    for (const next of adj.get(cur) || []) {
      stack.push(next);
    }
  }
  return false;
}

function typesCompatible(src: string, dst: string): boolean {
  if (src === "any" || dst === "any") return true;
  if (src === dst) return true;
  // Allow numeric to be read as bool (coerce) but not the other way.
  if (src === "number" && dst === "bool") return true;
  if (src === "bool" && dst === "number") return true;
  return false;
}
