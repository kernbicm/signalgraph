//! Rust factory for the default starter patch. Mirrors the TS factory in
//! `src/editor/starterPatch.ts` so the replay tests can round-trip the same
//! shape the editor saves.

use crate::contracts::{GraphDocument, GraphEdge, GraphNode, SCHEMA_VERSION};
use serde_json::json;

pub fn starter_patch() -> GraphDocument {
    GraphDocument {
        schema_version: SCHEMA_VERSION,
        id: "starter".to_string(),
        name: "starter".to_string(),
        description: Some(
            "MediaPipe index_tip.x -> map_range -> smooth -> OSC /demo/hand/x".to_string(),
        ),
        created_at: "1970-01-01T00:00:00Z".to_string(),
        updated_at: "1970-01-01T00:00:00Z".to_string(),
        nodes: vec![
            GraphNode {
                id: "tracker_signal_1".into(),
                kind: "tracker_signal".into(),
                label: Some("hand.left.index_tip.x".into()),
                config: json!({
                    "signal_path": "mp.hand.left.landmark.index_tip.x"
                }),
                position: Some([80.0, 160.0]),
            },
            GraphNode {
                id: "map_range_1".into(),
                kind: "map_range".into(),
                label: Some("normalize".into()),
                config: json!({
                    "in_min": 0.1,
                    "in_max": 0.9,
                    "out_min": 0.0,
                    "out_max": 1.0,
                    "clamp": true,
                    "invert": false
                }),
                position: Some([360.0, 160.0]),
            },
            GraphNode {
                id: "smooth_1".into(),
                kind: "smooth".into(),
                label: Some("smooth".into()),
                config: json!({ "alpha": 0.25 }),
                position: Some([640.0, 160.0]),
            },
            GraphNode {
                id: "osc_out_1".into(),
                kind: "osc_out".into(),
                label: Some("/demo/hand/x".into()),
                config: json!({
                    "host": "127.0.0.1",
                    "port": 9000,
                    "address": "/demo/hand/x",
                    "payload_type": "float",
                    "enabled": true
                }),
                position: Some([920.0, 160.0]),
            },
        ],
        edges: vec![
            GraphEdge {
                id: "e-src-mr".into(),
                source: "tracker_signal_1".into(),
                source_port: "out".into(),
                target: "map_range_1".into(),
                target_port: "input".into(),
            },
            GraphEdge {
                id: "e-mr-sm".into(),
                source: "map_range_1".into(),
                source_port: "out".into(),
                target: "smooth_1".into(),
                target_port: "input".into(),
            },
            GraphEdge {
                id: "e-sm-osc".into(),
                source: "smooth_1".into(),
                source_port: "out".into(),
                target: "osc_out_1".into(),
                target_port: "input".into(),
            },
        ],
    }
}

/// Starter with two sinks - one raw, one inverted - used by multi-sink tests.
pub fn multi_sink_patch() -> GraphDocument {
    GraphDocument {
        schema_version: SCHEMA_VERSION,
        id: "multi_sink".into(),
        name: "multi_sink".into(),
        description: Some("two OSC sinks driven by one source".into()),
        created_at: "1970-01-01T00:00:00Z".into(),
        updated_at: "1970-01-01T00:00:00Z".into(),
        nodes: vec![
            GraphNode {
                id: "src".into(),
                kind: "tracker_signal".into(),
                label: None,
                config: json!({
                    "signal_path": "mp.hand.left.landmark.index_tip.x"
                }),
                position: None,
            },
            GraphNode {
                id: "inv".into(),
                kind: "invert".into(),
                label: None,
                config: json!({ "min": 0.0, "max": 1.0 }),
                position: None,
            },
            GraphNode {
                id: "raw".into(),
                kind: "osc_out".into(),
                label: Some("raw".into()),
                config: json!({
                    "host": "127.0.0.1",
                    "port": 9000,
                    "address": "/raw/x",
                    "payload_type": "float",
                    "enabled": true
                }),
                position: None,
            },
            GraphNode {
                id: "mirror".into(),
                kind: "osc_out".into(),
                label: Some("mirror".into()),
                config: json!({
                    "host": "127.0.0.1",
                    "port": 9000,
                    "address": "/mirror/x",
                    "payload_type": "float",
                    "enabled": true
                }),
                position: None,
            },
        ],
        edges: vec![
            GraphEdge {
                id: "e1".into(),
                source: "src".into(),
                source_port: "out".into(),
                target: "raw".into(),
                target_port: "input".into(),
            },
            GraphEdge {
                id: "e2".into(),
                source: "src".into(),
                source_port: "out".into(),
                target: "inv".into(),
                target_port: "input".into(),
            },
            GraphEdge {
                id: "e3".into(),
                source: "inv".into(),
                source_port: "out".into(),
                target: "mirror".into(),
                target_port: "input".into(),
            },
        ],
    }
}
