use crate::contracts::GraphDocument;
use std::collections::HashSet;

const KNOWN_KINDS: &[&str] = &[
    "constant",
    "tracker_signal",
    "map_range",
    "clamp",
    "invert",
    "add",
    "multiply",
    "smooth",
    "deadzone",
    "threshold",
    "debug_meter",
    "osc_out",
];

pub fn validate(doc: &GraphDocument) -> Result<(), String> {
    let mut ids: HashSet<&str> = HashSet::new();
    for node in &doc.nodes {
        if !ids.insert(node.id.as_str()) {
            return Err(format!("duplicate node id '{}'", node.id));
        }
        if !KNOWN_KINDS.contains(&node.kind.as_str()) {
            return Err(format!(
                "unknown node kind '{}' on node '{}'",
                node.kind, node.id
            ));
        }
        if node.kind == "tracker_signal" {
            let ok = node
                .config
                .get("signal_path")
                .and_then(|v| v.as_str())
                .map(|s| !s.is_empty())
                .unwrap_or(false);
            if !ok {
                return Err(format!(
                    "tracker_signal '{}' missing signal_path",
                    node.id
                ));
            }
        }
        if node.kind == "osc_out" {
            let addr = node
                .config
                .get("address")
                .and_then(|v| v.as_str())
                .map(|s| !s.is_empty())
                .unwrap_or(false);
            if !addr {
                return Err(format!("osc_out '{}' missing address", node.id));
            }
        }
    }
    for edge in &doc.edges {
        if !ids.contains(edge.source.as_str()) {
            return Err(format!("edge {} references unknown source {}", edge.id, edge.source));
        }
        if !ids.contains(edge.target.as_str()) {
            return Err(format!("edge {} references unknown target {}", edge.id, edge.target));
        }
    }
    Ok(())
}
