// Mirrors src-tauri/src/contracts.rs. Keep in sync.

export const SCHEMA_VERSION = 1;

export type SignalKind =
  | "float"
  | "int"
  | "bool"
  | "category"
  | "vec2"
  | "vec3"
  | "missing";

export interface SignalDescriptor {
  path: string;
  kind: SignalKind;
  last_value: unknown;
  last_update_monotonic_ms: number;
  source: string;
}

export type WorkerState = "stopped" | "starting" | "running" | "error";
export type SourceMode = "fake" | "worker";

export interface RuntimeStatus {
  worker_state: WorkerState;
  source_mode: SourceMode;
  last_frame_monotonic_ms: number | null;
  last_error: string | null;
  osc_last_send_monotonic_ms: number | null;
  signals_count: number;
  loaded_patch: string | null;
}

export interface OscTarget {
  host: string;
  port: number;
  address: string;
}

export type OscPayloadType = "float" | "int" | "bool";

export interface HardcodedMapping {
  enabled: boolean;
  source: string;
  in_min: number;
  in_max: number;
  out_min: number;
  out_max: number;
  invert: boolean;
  target: OscTarget;
  payload_type: OscPayloadType;
}

export interface PacketLogEntry {
  monotonic_ms: number;
  host: string;
  port: number;
  address: string;
  payload_preview: string;
  source_label: string | null;
  sent: boolean;
  error: string | null;
}

export interface GraphNode {
  id: string;
  kind: string;
  label?: string | null;
  config: Record<string, unknown>;
  position?: [number, number] | null;
}

export interface GraphEdge {
  id: string;
  source: string;
  source_port: string;
  target: string;
  target_port: string;
}

export interface GraphDocument {
  schema_version: number;
  id: string;
  name: string;
  description?: string | null;
  created_at: string;
  updated_at: string;
  nodes: GraphNode[];
  edges: GraphEdge[];
}

export interface SinkSnapshot {
  node_id: string;
  label: string;
  address: string;
  host: string;
  port: number;
  last_value: unknown;
  enabled: boolean;
}

export interface RuntimeSnapshot {
  frame_monotonic_ms: number | null;
  node_outputs: Record<string, unknown>;
  errors: string[];
  sinks: SinkSnapshot[];
}

export interface CalibrationResult {
  signal_path: string;
  min: number;
  max: number;
  samples: number;
}
