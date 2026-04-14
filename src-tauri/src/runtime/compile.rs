//! Compile a `GraphDocument` (authored in React Flow) into a `GraphRuntime`.

use crate::contracts::{GraphDocument, OscTarget};
use crate::runtime::nodes::*;
use crate::runtime::validate;
use crate::runtime::GraphRuntime;
use std::collections::{BTreeMap, HashMap, VecDeque};

pub fn compile(doc: &GraphDocument) -> Result<GraphRuntime, String> {
    validate::validate(doc)?;

    let order = topological_order(doc)?;
    let input_map = build_input_map(doc);
    let mut compiled: BTreeMap<String, CompiledNode> = BTreeMap::new();
    let mut sinks: Vec<String> = Vec::new();

    for node in &doc.nodes {
        let cfg = &node.config;
        let get_f = |k: &str, default: f64| cfg.get(k).and_then(|v| v.as_f64()).unwrap_or(default);
        let get_b = |k: &str, default: bool| cfg.get(k).and_then(|v| v.as_bool()).unwrap_or(default);
        let get_s = |k: &str, default: &str| {
            cfg.get(k)
                .and_then(|v| v.as_str())
                .unwrap_or(default)
                .to_string()
        };

        let compiled_node = match node.kind.as_str() {
            "constant" => CompiledNode::Constant(Constant {
                value: get_f("value", 0.0),
            }),
            "tracker_signal" => CompiledNode::TrackerSignal(TrackerSignal {
                signal_path: get_s("signal_path", ""),
            }),
            "map_range" => {
                let input = require_input(&input_map, &node.id, "input")?;
                CompiledNode::MapRange(MapRange {
                    input,
                    in_min: get_f("in_min", 0.0),
                    in_max: get_f("in_max", 1.0),
                    out_min: get_f("out_min", 0.0),
                    out_max: get_f("out_max", 1.0),
                    clamp: get_b("clamp", true),
                    invert: get_b("invert", false),
                })
            }
            "clamp" => CompiledNode::Clamp(Clamp {
                input: require_input(&input_map, &node.id, "input")?,
                min: get_f("min", 0.0),
                max: get_f("max", 1.0),
            }),
            "invert" => CompiledNode::Invert(Invert {
                input: require_input(&input_map, &node.id, "input")?,
                min: get_f("min", 0.0),
                max: get_f("max", 1.0),
            }),
            "add" => CompiledNode::Add(BinaryOp {
                a: require_input(&input_map, &node.id, "a")?,
                b: require_input(&input_map, &node.id, "b")?,
            }),
            "multiply" => CompiledNode::Multiply(BinaryOp {
                a: require_input(&input_map, &node.id, "a")?,
                b: require_input(&input_map, &node.id, "b")?,
            }),
            "smooth" => CompiledNode::Smooth(Smooth {
                input: require_input(&input_map, &node.id, "input")?,
                alpha: get_f("alpha", 0.2),
                state: None,
            }),
            "deadzone" => CompiledNode::Deadzone(Deadzone {
                input: require_input(&input_map, &node.id, "input")?,
                center: get_f("center", 0.5),
                radius: get_f("radius", 0.05),
            }),
            "threshold" => CompiledNode::Threshold(Threshold {
                input: require_input(&input_map, &node.id, "input")?,
                threshold: get_f("threshold", 0.5),
            }),
            "debug_meter" => CompiledNode::DebugMeter(DebugMeter {
                input: require_input(&input_map, &node.id, "input")?,
                last: None,
            }),
            "osc_out" => {
                let host = get_s("host", "127.0.0.1");
                let port = cfg.get("port").and_then(|v| v.as_u64()).unwrap_or(9000) as u16;
                let address = get_s("address", "/signalgraph/out");
                let label = node.label.clone().unwrap_or_else(|| address.clone());
                let payload_type = get_s("payload_type", "float");
                let enabled = get_b("enabled", true);
                let input = require_input(&input_map, &node.id, "input")?;
                sinks.push(node.id.clone());
                CompiledNode::OscOut(OscOutSink {
                    input,
                    label,
                    target: OscTarget { host, port, address },
                    payload_type,
                    enabled,
                    last_value: None,
                })
            }
            other => return Err(format!("unknown node kind '{other}'")),
        };
        compiled.insert(node.id.clone(), compiled_node);
    }

    Ok(GraphRuntime {
        doc: Some(doc.clone()),
        order,
        nodes: compiled,
        sinks,
        errors: Vec::new(),
    })
}

fn build_input_map(doc: &GraphDocument) -> HashMap<(String, String), String> {
    let mut map = HashMap::new();
    for edge in &doc.edges {
        map.insert((edge.target.clone(), edge.target_port.clone()), edge.source.clone());
    }
    map
}

fn require_input(
    map: &HashMap<(String, String), String>,
    node_id: &str,
    port: &str,
) -> Result<String, String> {
    map.get(&(node_id.to_string(), port.to_string()))
        .cloned()
        .ok_or_else(|| format!("node {node_id} missing input on port '{port}'"))
}

pub fn topological_order(doc: &GraphDocument) -> Result<Vec<String>, String> {
    let mut indeg: HashMap<String, usize> = doc.nodes.iter().map(|n| (n.id.clone(), 0)).collect();
    let mut adj: HashMap<String, Vec<String>> = HashMap::new();
    for edge in &doc.edges {
        adj.entry(edge.source.clone()).or_default().push(edge.target.clone());
        *indeg.entry(edge.target.clone()).or_insert(0) += 1;
    }
    let mut queue: VecDeque<String> = indeg
        .iter()
        .filter(|(_, &d)| d == 0)
        .map(|(k, _)| k.clone())
        .collect();
    // Keep deterministic order regardless of HashMap iteration:
    let mut initial: Vec<String> = queue.into_iter().collect();
    initial.sort();
    queue = initial.into();

    let mut order = Vec::new();
    while let Some(id) = queue.pop_front() {
        order.push(id.clone());
        if let Some(targets) = adj.get(&id) {
            let mut targets_sorted = targets.clone();
            targets_sorted.sort();
            for t in targets_sorted {
                let e = indeg.get_mut(&t).unwrap();
                *e -= 1;
                if *e == 0 {
                    queue.push_back(t);
                }
            }
        }
    }
    if order.len() != doc.nodes.len() {
        return Err("graph contains a cycle".into());
    }
    Ok(order)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::{GraphEdge, GraphNode, SCHEMA_VERSION};
    use serde_json::json;

    fn node(id: &str, kind: &str, config: serde_json::Value) -> GraphNode {
        GraphNode {
            id: id.to_string(),
            kind: kind.to_string(),
            label: None,
            config,
            position: None,
        }
    }

    fn edge(id: &str, source: &str, target: &str, port: &str) -> GraphEdge {
        GraphEdge {
            id: id.to_string(),
            source: source.to_string(),
            source_port: "out".to_string(),
            target: target.to_string(),
            target_port: port.to_string(),
        }
    }

    #[test]
    fn topological_order_is_deterministic() {
        let doc = GraphDocument {
            schema_version: SCHEMA_VERSION,
            id: "p".into(),
            name: "p".into(),
            description: None,
            created_at: "".into(),
            updated_at: "".into(),
            nodes: vec![
                node("src", "tracker_signal", json!({"signal_path": "mp.hand.left.landmark.index_tip.x"})),
                node(
                    "mr",
                    "map_range",
                    json!({"in_min":0.0,"in_max":1.0,"out_min":0.0,"out_max":100.0}),
                ),
                node("sink", "osc_out", json!({"address": "/x"})),
            ],
            edges: vec![
                edge("e1", "src", "mr", "input"),
                edge("e2", "mr", "sink", "input"),
            ],
        };
        let order = topological_order(&doc).unwrap();
        assert_eq!(order, vec!["src", "mr", "sink"]);
    }

    #[test]
    fn cycles_rejected() {
        let doc = GraphDocument {
            schema_version: SCHEMA_VERSION,
            id: "p".into(),
            name: "p".into(),
            description: None,
            created_at: "".into(),
            updated_at: "".into(),
            nodes: vec![
                node("a", "add", json!({})),
                node("b", "add", json!({})),
            ],
            edges: vec![edge("e1", "a", "b", "a"), edge("e2", "b", "a", "a")],
        };
        assert!(topological_order(&doc).is_err());
    }
}
