//! Rust-owned graph runtime. Compiles patches into an executable form,
//! validates, and evaluates on each telemetry tick.
//!
//! Phase 1 lands the module skeleton and an always-empty runtime so the
//! UI and commands can wire through. Phase 4 fills out the real evaluators.

pub mod compile;
pub mod nodes;
pub mod patches;
pub mod replay;
pub mod starter;
pub mod validate;

use crate::contracts::{GraphDocument, RuntimeSnapshot, SinkSnapshot};
use crate::signals::SignalBus;
use std::collections::BTreeMap;

/// Compiled, ready-to-execute runtime graph.
pub struct GraphRuntime {
    pub doc: Option<GraphDocument>,
    pub order: Vec<String>,
    pub nodes: BTreeMap<String, nodes::CompiledNode>,
    pub sinks: Vec<String>,
    pub errors: Vec<String>,
}

impl GraphRuntime {
    pub fn empty() -> Self {
        Self {
            doc: None,
            order: Vec::new(),
            nodes: BTreeMap::new(),
            sinks: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.order.is_empty()
    }

    /// Evaluate the graph once against the current SignalBus snapshot and
    /// produce a `RuntimeSnapshot` plus a list of OSC messages to send.
    pub fn tick(
        &mut self,
        bus: &SignalBus,
        frame_monotonic_ms: u64,
    ) -> (RuntimeSnapshot, Vec<nodes::SinkOutput>) {
        let mut snapshot = RuntimeSnapshot {
            frame_monotonic_ms: Some(frame_monotonic_ms),
            node_outputs: BTreeMap::new(),
            errors: Vec::new(),
            sinks: Vec::new(),
        };
        let mut sink_outputs: Vec<nodes::SinkOutput> = Vec::new();
        let mut outputs: BTreeMap<String, nodes::Value> = BTreeMap::new();

        for node_id in self.order.clone() {
            let Some(node) = self.nodes.get_mut(&node_id) else {
                snapshot.errors.push(format!("missing node {node_id}"));
                continue;
            };
            let result = node.evaluate(bus, &outputs, frame_monotonic_ms);
            match result {
                Ok(value) => {
                    outputs.insert(node_id.clone(), value.clone());
                    let as_json = value.to_json();
                    snapshot
                        .node_outputs
                        .insert(node_id.clone(), as_json.clone());
                    if let nodes::CompiledNode::OscOut(sink) = node {
                        let enabled = sink.enabled;
                        snapshot.sinks.push(SinkSnapshot {
                            node_id: node_id.clone(),
                            label: sink.label.clone(),
                            address: sink.target.address.clone(),
                            host: sink.target.host.clone(),
                            port: sink.target.port,
                            last_value: Some(as_json.clone()),
                            enabled,
                        });
                        if enabled {
                            if let Some(output) = sink.materialize(&outputs) {
                                sink_outputs.push(output);
                            }
                        }
                    }
                }
                Err(e) => {
                    snapshot.errors.push(format!("{node_id}: {e}"));
                }
            }
        }
        self.errors = snapshot.errors.clone();
        (snapshot, sink_outputs)
    }
}
