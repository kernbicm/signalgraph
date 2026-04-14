//! Internal data contracts shared across frontend, core, and worker.
//!
//! Everything the UI or the graph sees must be shaped like these types.
//! The Python worker emits telemetry frames that map to `TelemetryFrame`.
//! TypeScript mirrors in `src/lib/types.ts`.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const SCHEMA_VERSION: u32 = 1;

// ---------- Telemetry ----------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum SignalValue {
    Float(f64),
    Int(i64),
    Bool(bool),
    Category(String),
    Vec2([f64; 2]),
    Vec3([f64; 3]),
    Missing,
}

impl SignalValue {
    pub fn as_float(&self) -> Option<f64> {
        match self {
            SignalValue::Float(v) => Some(*v),
            SignalValue::Int(v) => Some(*v as f64),
            SignalValue::Bool(b) => Some(if *b { 1.0 } else { 0.0 }),
            _ => None,
        }
    }
}

/// Stable, dot-separated signal path, e.g. `mp.hand.left.landmark.index_tip.x`.
pub type SignalPath = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryFrame {
    pub schema_version: u32,
    pub source: String,
    pub monotonic_ms: u64,
    pub signals: BTreeMap<SignalPath, SignalValue>,
}

impl TelemetryFrame {
    pub fn new(source: impl Into<String>, monotonic_ms: u64) -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            source: source.into(),
            monotonic_ms,
            signals: BTreeMap::new(),
        }
    }
}

// ---------- Runtime status ----------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkerState {
    Stopped,
    Starting,
    Running,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceMode {
    Fake,
    Worker,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeStatus {
    pub worker_state: WorkerState,
    pub source_mode: SourceMode,
    pub last_frame_monotonic_ms: Option<u64>,
    pub last_error: Option<String>,
    pub osc_last_send_monotonic_ms: Option<u64>,
    pub signals_count: usize,
    pub loaded_patch: Option<String>,
}

impl Default for RuntimeStatus {
    fn default() -> Self {
        Self {
            worker_state: WorkerState::Stopped,
            source_mode: SourceMode::Fake,
            last_frame_monotonic_ms: None,
            last_error: None,
            osc_last_send_monotonic_ms: None,
            signals_count: 0,
            loaded_patch: None,
        }
    }
}

// ---------- OSC ----------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OscPayload {
    Float(f32),
    Int(i32),
    Bool(bool),
    String(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OscTarget {
    pub host: String,
    pub port: u16,
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OscMessageDraft {
    pub target: OscTarget,
    pub payload: OscPayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacketLogEntry {
    pub monotonic_ms: u64,
    pub host: String,
    pub port: u16,
    pub address: String,
    pub payload_preview: String,
    pub source_label: Option<String>,
    pub sent: bool,
    pub error: Option<String>,
}

// ---------- Graph document ----------

pub type NodeId = String;
pub type PortName = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: NodeId,
    pub kind: String,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub config: serde_json::Value,
    #[serde(default)]
    pub position: Option<[f64; 2]>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub id: String,
    pub source: NodeId,
    pub source_port: PortName,
    pub target: NodeId,
    pub target_port: PortName,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphDocument {
    pub schema_version: u32,
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

impl GraphDocument {
    pub fn empty(id: &str, name: &str) -> Self {
        let now = chrono_now_iso8601();
        Self {
            schema_version: SCHEMA_VERSION,
            id: id.to_string(),
            name: name.to_string(),
            description: None,
            created_at: now.clone(),
            updated_at: now,
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
}

/// Monotonic-ish pseudo-timestamp without pulling chrono as a dep.
fn chrono_now_iso8601() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("1970-01-01T00:00:00Z+{}", secs)
}

// ---------- Hardcoded mapping (phase 2 regression path) ----------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardcodedMapping {
    pub enabled: bool,
    pub source: SignalPath,
    pub in_min: f64,
    pub in_max: f64,
    pub out_min: f64,
    pub out_max: f64,
    pub invert: bool,
    pub target: OscTarget,
    pub payload_type: String, // "float" | "int" | "bool"
}

impl Default for HardcodedMapping {
    fn default() -> Self {
        Self {
            enabled: false,
            source: "mp.hand.left.landmark.index_tip.x".to_string(),
            in_min: 0.0,
            in_max: 1.0,
            out_min: 0.0,
            out_max: 1.0,
            invert: false,
            target: OscTarget {
                host: "127.0.0.1".to_string(),
                port: 9000,
                address: "/demo/hand/x".to_string(),
            },
            payload_type: "float".to_string(),
        }
    }
}

// ---------- Runtime snapshot ----------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RuntimeSnapshot {
    pub frame_monotonic_ms: Option<u64>,
    pub node_outputs: BTreeMap<String, serde_json::Value>,
    pub errors: Vec<String>,
    pub sinks: Vec<SinkSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationResult {
    pub signal_path: String,
    pub min: f64,
    pub max: f64,
    pub samples: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SinkSnapshot {
    pub node_id: String,
    pub label: String,
    pub address: String,
    pub host: String,
    pub port: u16,
    pub last_value: Option<serde_json::Value>,
    pub enabled: bool,
}
