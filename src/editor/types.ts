import type { Node, Edge } from "@xyflow/react";

export type NodeKind =
  | "constant"
  | "tracker_signal"
  | "map_range"
  | "clamp"
  | "invert"
  | "add"
  | "multiply"
  | "smooth"
  | "deadzone"
  | "threshold"
  | "debug_meter"
  | "osc_out";

export type PortType = "number" | "bool" | "string" | "any";

export interface PortDef {
  id: string;
  label: string;
  type: PortType;
}

export interface NodeSpec {
  kind: NodeKind;
  label: string;
  category: "source" | "transform" | "output" | "debug";
  inputs: PortDef[];
  outputs: PortDef[];
  defaultConfig: Record<string, unknown>;
  description: string;
}

export interface PatchNodeData extends Record<string, unknown> {
  kind: NodeKind;
  label: string;
  config: Record<string, unknown>;
  liveValue?: unknown;
}

export type PatchNode = Node<PatchNodeData>;
export type PatchEdge = Edge;
