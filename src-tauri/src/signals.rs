//! SignalBus - the single source of truth for the most recent signal snapshot.
//!
//! Telemetry frames (fake or worker) are adapted into stable named signals here.
//! The runtime and UI read from this bus. Raw worker payloads never reach the UI.

use crate::contracts::{SignalPath, SignalValue, TelemetryFrame};
use parking_lot::RwLock;
use std::collections::BTreeMap;
use std::sync::Arc;

#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct SignalDescriptor {
    pub path: SignalPath,
    pub kind: String, // "float" | "int" | "bool" | "category" | "vec2" | "vec3" | "missing"
    pub last_value: Option<serde_json::Value>,
    pub last_update_monotonic_ms: u64,
    pub source: String,
}

#[derive(Default)]
struct Inner {
    signals: BTreeMap<SignalPath, SignalDescriptor>,
    latest_frame_monotonic_ms: Option<u64>,
    latest_frame_source: Option<String>,
}

#[derive(Clone)]
pub struct SignalBus {
    inner: Arc<RwLock<Inner>>,
}

impl SignalBus {
    pub fn new() -> Self {
        Self { inner: Arc::new(RwLock::new(Inner::default())) }
    }

    pub fn ingest(&self, frame: &TelemetryFrame) {
        let mut guard = self.inner.write();
        guard.latest_frame_monotonic_ms = Some(frame.monotonic_ms);
        guard.latest_frame_source = Some(frame.source.clone());
        for (path, value) in &frame.signals {
            let (kind, json_value) = describe(value);
            guard
                .signals
                .entry(path.clone())
                .and_modify(|desc| {
                    desc.kind = kind.to_string();
                    desc.last_value = Some(json_value.clone());
                    desc.last_update_monotonic_ms = frame.monotonic_ms;
                    desc.source = frame.source.clone();
                })
                .or_insert(SignalDescriptor {
                    path: path.clone(),
                    kind: kind.to_string(),
                    last_value: Some(json_value),
                    last_update_monotonic_ms: frame.monotonic_ms,
                    source: frame.source.clone(),
                });
        }
    }

    pub fn list_descriptors(&self) -> Vec<SignalDescriptor> {
        self.inner.read().signals.values().cloned().collect()
    }

    pub fn count(&self) -> usize {
        self.inner.read().signals.len()
    }

    pub fn latest_frame_monotonic_ms(&self) -> Option<u64> {
        self.inner.read().latest_frame_monotonic_ms
    }

    pub fn read(&self, path: &str) -> Option<SignalValue> {
        let guard = self.inner.read();
        let desc = guard.signals.get(path)?;
        let value = desc.last_value.as_ref()?;
        json_to_signal(value, &desc.kind)
    }

    pub fn read_float(&self, path: &str) -> Option<f64> {
        self.read(path).and_then(|v| v.as_float())
    }

    pub fn clear(&self) {
        let mut guard = self.inner.write();
        guard.signals.clear();
        guard.latest_frame_monotonic_ms = None;
        guard.latest_frame_source = None;
    }
}

fn describe(value: &SignalValue) -> (&'static str, serde_json::Value) {
    match value {
        SignalValue::Float(v) => ("float", serde_json::json!(v)),
        SignalValue::Int(v) => ("int", serde_json::json!(v)),
        SignalValue::Bool(v) => ("bool", serde_json::json!(v)),
        SignalValue::Category(v) => ("category", serde_json::json!(v)),
        SignalValue::Vec2(v) => ("vec2", serde_json::json!(v)),
        SignalValue::Vec3(v) => ("vec3", serde_json::json!(v)),
        SignalValue::Missing => ("missing", serde_json::Value::Null),
    }
}

fn json_to_signal(value: &serde_json::Value, kind: &str) -> Option<SignalValue> {
    match kind {
        "float" => value.as_f64().map(SignalValue::Float),
        "int" => value.as_i64().map(SignalValue::Int),
        "bool" => value.as_bool().map(SignalValue::Bool),
        "category" => value.as_str().map(|s| SignalValue::Category(s.to_string())),
        "vec2" => {
            let arr = value.as_array()?;
            if arr.len() == 2 {
                Some(SignalValue::Vec2([arr[0].as_f64()?, arr[1].as_f64()?]))
            } else {
                None
            }
        }
        "vec3" => {
            let arr = value.as_array()?;
            if arr.len() == 3 {
                Some(SignalValue::Vec3([
                    arr[0].as_f64()?,
                    arr[1].as_f64()?,
                    arr[2].as_f64()?,
                ]))
            } else {
                None
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ingests_and_reads_float() {
        let bus = SignalBus::new();
        let mut frame = TelemetryFrame::new("fake", 1000);
        frame
            .signals
            .insert("mp.hand.left.landmark.index_tip.x".into(), SignalValue::Float(0.42));
        bus.ingest(&frame);

        assert_eq!(bus.count(), 1);
        assert_eq!(
            bus.read_float("mp.hand.left.landmark.index_tip.x"),
            Some(0.42)
        );
    }
}
