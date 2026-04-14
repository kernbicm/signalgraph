//! Node evaluators. Phase 1 only needs the scaffolding for a handful of
//! kinds; Phase 4 will expand them with full test coverage.

use crate::contracts::{OscPayload, OscTarget};
use crate::signals::SignalBus;
use serde_json::json;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub enum Value {
    Float(f64),
    Int(i64),
    Bool(bool),
    Category(String),
    None,
}

impl Value {
    pub fn as_float(&self) -> Option<f64> {
        match self {
            Value::Float(v) => Some(*v),
            Value::Int(v) => Some(*v as f64),
            Value::Bool(b) => Some(if *b { 1.0 } else { 0.0 }),
            _ => None,
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        match self {
            Value::Float(v) => json!(v),
            Value::Int(v) => json!(v),
            Value::Bool(v) => json!(v),
            Value::Category(v) => json!(v),
            Value::None => serde_json::Value::Null,
        }
    }
}

#[derive(Debug)]
pub enum CompiledNode {
    Constant(Constant),
    TrackerSignal(TrackerSignal),
    MapRange(MapRange),
    Clamp(Clamp),
    Invert(Invert),
    Add(BinaryOp),
    Multiply(BinaryOp),
    Smooth(Smooth),
    Deadzone(Deadzone),
    Threshold(Threshold),
    DebugMeter(DebugMeter),
    OscOut(OscOutSink),
}

impl CompiledNode {
    pub fn evaluate(
        &mut self,
        bus: &SignalBus,
        outputs: &BTreeMap<String, Value>,
        _frame_monotonic_ms: u64,
    ) -> Result<Value, String> {
        match self {
            CompiledNode::Constant(n) => Ok(Value::Float(n.value)),
            CompiledNode::TrackerSignal(n) => Ok(bus
                .read_float(&n.signal_path)
                .map(Value::Float)
                .unwrap_or(Value::None)),
            CompiledNode::MapRange(n) => {
                let input = resolve_float(outputs, &n.input)?;
                let out = map_range(input, n.in_min, n.in_max, n.out_min, n.out_max, n.clamp, n.invert);
                Ok(Value::Float(out))
            }
            CompiledNode::Clamp(n) => {
                let input = resolve_float(outputs, &n.input)?;
                Ok(Value::Float(input.clamp(n.min, n.max)))
            }
            CompiledNode::Invert(n) => {
                let input = resolve_float(outputs, &n.input)?;
                Ok(Value::Float(n.max - input + n.min))
            }
            CompiledNode::Add(n) => {
                let a = resolve_float(outputs, &n.a)?;
                let b = resolve_float(outputs, &n.b)?;
                Ok(Value::Float(a + b))
            }
            CompiledNode::Multiply(n) => {
                let a = resolve_float(outputs, &n.a)?;
                let b = resolve_float(outputs, &n.b)?;
                Ok(Value::Float(a * b))
            }
            CompiledNode::Smooth(n) => {
                let input = resolve_float(outputs, &n.input)?;
                let prev = n.state.unwrap_or(input);
                let next = prev + (input - prev) * n.alpha;
                n.state = Some(next);
                Ok(Value::Float(next))
            }
            CompiledNode::Deadzone(n) => {
                let input = resolve_float(outputs, &n.input)?;
                let out = if (input - n.center).abs() <= n.radius {
                    n.center
                } else {
                    input
                };
                Ok(Value::Float(out))
            }
            CompiledNode::Threshold(n) => {
                let input = resolve_float(outputs, &n.input)?;
                Ok(Value::Bool(input >= n.threshold))
            }
            CompiledNode::DebugMeter(n) => {
                let input = resolve_float(outputs, &n.input)?;
                n.last = Some(input);
                Ok(Value::Float(input))
            }
            CompiledNode::OscOut(n) => {
                // The sink stores its most recent computed value for later materialization.
                let v = outputs
                    .get(&n.input)
                    .cloned()
                    .unwrap_or(Value::None);
                n.last_value = Some(v.clone());
                Ok(v)
            }
        }
    }
}

fn resolve_float(outputs: &BTreeMap<String, Value>, node_id: &str) -> Result<f64, String> {
    let v = outputs.get(node_id).ok_or_else(|| format!("missing input {node_id}"))?;
    v.as_float().ok_or_else(|| format!("input {node_id} is not numeric"))
}

pub fn map_range(
    v: f64,
    in_min: f64,
    in_max: f64,
    out_min: f64,
    out_max: f64,
    clamp: bool,
    invert: bool,
) -> f64 {
    let span = in_max - in_min;
    if span == 0.0 {
        return out_min;
    }
    let t = (v - in_min) / span;
    let t = if invert { 1.0 - t } else { t };
    let t = if clamp { t.clamp(0.0, 1.0) } else { t };
    out_min + t * (out_max - out_min)
}

// ---------- Node structs ----------

#[derive(Debug, Clone)]
pub struct Constant {
    pub value: f64,
}

#[derive(Debug, Clone)]
pub struct TrackerSignal {
    pub signal_path: String,
}

#[derive(Debug, Clone)]
pub struct MapRange {
    pub input: String,
    pub in_min: f64,
    pub in_max: f64,
    pub out_min: f64,
    pub out_max: f64,
    pub clamp: bool,
    pub invert: bool,
}

#[derive(Debug, Clone)]
pub struct Clamp {
    pub input: String,
    pub min: f64,
    pub max: f64,
}

#[derive(Debug, Clone)]
pub struct Invert {
    pub input: String,
    pub min: f64,
    pub max: f64,
}

#[derive(Debug, Clone)]
pub struct BinaryOp {
    pub a: String,
    pub b: String,
}

#[derive(Debug)]
pub struct Smooth {
    pub input: String,
    pub alpha: f64,
    pub state: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct Deadzone {
    pub input: String,
    pub center: f64,
    pub radius: f64,
}

#[derive(Debug, Clone)]
pub struct Threshold {
    pub input: String,
    pub threshold: f64,
}

#[derive(Debug)]
pub struct DebugMeter {
    pub input: String,
    pub last: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct OscOutSink {
    pub input: String,
    pub label: String,
    pub target: OscTarget,
    pub payload_type: String, // float | int | bool
    pub enabled: bool,
    pub last_value: Option<Value>,
}

#[derive(Debug, Clone)]
pub struct SinkOutput {
    pub target: OscTarget,
    pub payload: OscPayload,
    pub label: String,
}

impl OscOutSink {
    pub fn materialize(&self, outputs: &BTreeMap<String, Value>) -> Option<SinkOutput> {
        let v = outputs.get(&self.input)?;
        let payload = match self.payload_type.as_str() {
            "int" => OscPayload::Int(v.as_float()? as i32),
            "bool" => OscPayload::Bool(v.as_float()? > 0.5),
            _ => OscPayload::Float(v.as_float()? as f32),
        };
        Some(SinkOutput {
            target: self.target.clone(),
            payload,
            label: self.label.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signals::SignalBus;

    fn outputs(pairs: &[(&str, Value)]) -> BTreeMap<String, Value> {
        pairs.iter().map(|(k, v)| (k.to_string(), v.clone())).collect()
    }

    #[test]
    fn map_range_basic() {
        assert_eq!(map_range(0.5, 0.0, 1.0, 0.0, 100.0, false, false), 50.0);
        assert_eq!(map_range(0.25, 0.0, 1.0, 0.0, 100.0, false, true), 75.0);
        assert_eq!(map_range(-1.0, 0.0, 1.0, 0.0, 100.0, true, false), 0.0);
        assert_eq!(map_range(2.0, 0.0, 1.0, 0.0, 100.0, true, false), 100.0);
    }

    #[test]
    fn map_range_zero_span_returns_out_min() {
        assert_eq!(map_range(0.5, 1.0, 1.0, 3.0, 7.0, false, false), 3.0);
    }

    #[test]
    fn constant_node_outputs_its_value() {
        let mut n = CompiledNode::Constant(Constant { value: 42.5 });
        let bus = SignalBus::new();
        let v = n.evaluate(&bus, &outputs(&[]), 0).unwrap();
        assert_eq!(v.as_float(), Some(42.5));
    }

    #[test]
    fn clamp_node_clamps() {
        let mut n = CompiledNode::Clamp(Clamp {
            input: "src".into(),
            min: 0.0,
            max: 1.0,
        });
        let bus = SignalBus::new();
        let out_low = n
            .evaluate(&bus, &outputs(&[("src", Value::Float(-5.0))]), 0)
            .unwrap();
        assert_eq!(out_low.as_float(), Some(0.0));
        let out_high = n
            .evaluate(&bus, &outputs(&[("src", Value::Float(5.0))]), 0)
            .unwrap();
        assert_eq!(out_high.as_float(), Some(1.0));
        let out_mid = n
            .evaluate(&bus, &outputs(&[("src", Value::Float(0.5))]), 0)
            .unwrap();
        assert_eq!(out_mid.as_float(), Some(0.5));
    }

    #[test]
    fn invert_node_mirrors() {
        let mut n = CompiledNode::Invert(Invert {
            input: "src".into(),
            min: 0.0,
            max: 1.0,
        });
        let bus = SignalBus::new();
        let v = n
            .evaluate(&bus, &outputs(&[("src", Value::Float(0.2))]), 0)
            .unwrap();
        assert_eq!(v.as_float(), Some(0.8));
    }

    #[test]
    fn binary_ops_work() {
        let mut add = CompiledNode::Add(BinaryOp {
            a: "a".into(),
            b: "b".into(),
        });
        let mut mul = CompiledNode::Multiply(BinaryOp {
            a: "a".into(),
            b: "b".into(),
        });
        let bus = SignalBus::new();
        let ctx = outputs(&[("a", Value::Float(3.0)), ("b", Value::Float(4.0))]);
        assert_eq!(add.evaluate(&bus, &ctx, 0).unwrap().as_float(), Some(7.0));
        assert_eq!(mul.evaluate(&bus, &ctx, 0).unwrap().as_float(), Some(12.0));
    }

    #[test]
    fn smooth_node_low_pass() {
        let mut n = CompiledNode::Smooth(Smooth {
            input: "src".into(),
            alpha: 0.5,
            state: None,
        });
        let bus = SignalBus::new();
        // First sample initializes state to input
        let v1 = n
            .evaluate(&bus, &outputs(&[("src", Value::Float(1.0))]), 0)
            .unwrap();
        assert_eq!(v1.as_float(), Some(1.0));
        // Sudden drop to 0.0 with alpha=0.5 -> halfway
        let v2 = n
            .evaluate(&bus, &outputs(&[("src", Value::Float(0.0))]), 0)
            .unwrap();
        assert_eq!(v2.as_float(), Some(0.5));
        // Stay at 0.0 -> 0.25
        let v3 = n
            .evaluate(&bus, &outputs(&[("src", Value::Float(0.0))]), 0)
            .unwrap();
        assert_eq!(v3.as_float(), Some(0.25));
    }

    #[test]
    fn deadzone_snaps_near_center() {
        let mut n = CompiledNode::Deadzone(Deadzone {
            input: "src".into(),
            center: 0.5,
            radius: 0.1,
        });
        let bus = SignalBus::new();
        let inside = n
            .evaluate(&bus, &outputs(&[("src", Value::Float(0.55))]), 0)
            .unwrap();
        assert_eq!(inside.as_float(), Some(0.5));
        let outside = n
            .evaluate(&bus, &outputs(&[("src", Value::Float(0.8))]), 0)
            .unwrap();
        assert_eq!(outside.as_float(), Some(0.8));
    }

    #[test]
    fn threshold_node_emits_bool() {
        let mut n = CompiledNode::Threshold(Threshold {
            input: "src".into(),
            threshold: 0.5,
        });
        let bus = SignalBus::new();
        let low = n
            .evaluate(&bus, &outputs(&[("src", Value::Float(0.4))]), 0)
            .unwrap();
        assert!(matches!(low, Value::Bool(false)));
        let high = n
            .evaluate(&bus, &outputs(&[("src", Value::Float(0.7))]), 0)
            .unwrap();
        assert!(matches!(high, Value::Bool(true)));
    }

    #[test]
    fn osc_out_sink_payload_types() {
        use crate::contracts::OscTarget;
        let sink = OscOutSink {
            input: "src".into(),
            label: "l".into(),
            target: OscTarget {
                host: "127.0.0.1".into(),
                port: 9000,
                address: "/t".into(),
            },
            payload_type: "int".into(),
            enabled: true,
            last_value: None,
        };
        let out = sink
            .materialize(&outputs(&[("src", Value::Float(3.7))]))
            .unwrap();
        match out.payload {
            crate::contracts::OscPayload::Int(v) => assert_eq!(v, 3),
            _ => panic!(),
        }

        let bool_sink = OscOutSink {
            input: "src".into(),
            label: "l".into(),
            target: OscTarget {
                host: "127.0.0.1".into(),
                port: 9000,
                address: "/t".into(),
            },
            payload_type: "bool".into(),
            enabled: true,
            last_value: None,
        };
        let out = bool_sink
            .materialize(&outputs(&[("src", Value::Float(0.6))]))
            .unwrap();
        match out.payload {
            crate::contracts::OscPayload::Bool(v) => assert!(v),
            _ => panic!(),
        }
    }
}
