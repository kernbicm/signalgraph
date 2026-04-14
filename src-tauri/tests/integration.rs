//! End-to-end integration tests that wire the whole pipeline up:
//! SignalBus + compiled runtime + real UDP OSC sender + a loopback receiver.

use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

use signalgraph_lib::contracts::{
    GraphDocument, GraphEdge, GraphNode, SignalValue, TelemetryFrame, SCHEMA_VERSION,
};
use signalgraph_lib::osc::OscSender;
use signalgraph_lib::runtime::compile::compile;
use signalgraph_lib::signals::SignalBus;

fn patch_with_sink(host: &str, port: u16, address: &str) -> GraphDocument {
    GraphDocument {
        schema_version: SCHEMA_VERSION,
        id: "p".into(),
        name: "p".into(),
        description: None,
        created_at: "".into(),
        updated_at: "".into(),
        nodes: vec![
            GraphNode {
                id: "src".into(),
                kind: "tracker_signal".into(),
                label: None,
                config: serde_json::json!({
                    "signal_path": "mp.hand.left.landmark.index_tip.x"
                }),
                position: None,
            },
            GraphNode {
                id: "mr".into(),
                kind: "map_range".into(),
                label: None,
                config: serde_json::json!({
                    "in_min": 0.0, "in_max": 1.0,
                    "out_min": 0.0, "out_max": 100.0,
                    "clamp": true, "invert": false,
                }),
                position: None,
            },
            GraphNode {
                id: "sink".into(),
                kind: "osc_out".into(),
                label: Some("sink".into()),
                config: serde_json::json!({
                    "host": host,
                    "port": port,
                    "address": address,
                    "payload_type": "float",
                    "enabled": true,
                }),
                position: None,
            },
        ],
        edges: vec![
            GraphEdge {
                id: "e1".into(),
                source: "src".into(),
                source_port: "out".into(),
                target: "mr".into(),
                target_port: "input".into(),
            },
            GraphEdge {
                id: "e2".into(),
                source: "mr".into(),
                source_port: "out".into(),
                target: "sink".into(),
                target_port: "input".into(),
            },
        ],
    }
}

#[test]
fn runtime_drives_real_udp_receiver() {
    // Bind a receiver on an ephemeral port.
    let receiver =
        UdpSocket::bind("127.0.0.1:0").expect("bind receiver");
    receiver
        .set_read_timeout(Some(Duration::from_millis(500)))
        .unwrap();
    let port = receiver.local_addr().unwrap().port();

    // Compile a patch whose sink targets that receiver.
    let doc = patch_with_sink("127.0.0.1", port, "/integration/x");
    let mut runtime = compile(&doc).expect("compile");

    let bus = SignalBus::new();
    let sender = OscSender::new().expect("osc sender");

    // Ingest a frame, tick, and dispatch.
    let mut frame = TelemetryFrame::new("test", 100);
    frame.signals.insert(
        "mp.hand.left.landmark.index_tip.x".into(),
        SignalValue::Float(0.42),
    );
    bus.ingest(&frame);
    let (_snapshot, sinks) = runtime.tick(&bus, frame.monotonic_ms);
    assert_eq!(sinks.len(), 1);
    for sink in &sinks {
        sender
            .send(&sink.target, &sink.payload, Some(sink.label.clone()), 100)
            .expect("send");
    }

    // Reader thread: receive one UDP packet and decode it.
    let handle = thread::spawn(move || {
        let mut buf = [0u8; 1024];
        let (n, _) = receiver.recv_from(&mut buf).expect("recv");
        rosc::decoder::decode_udp(&buf[..n]).expect("decode").1
    });

    let packet = handle.join().expect("thread");
    match packet {
        rosc::OscPacket::Message(msg) => {
            assert_eq!(msg.addr, "/integration/x");
            match msg.args.as_slice() {
                [rosc::OscType::Float(v)] => {
                    // 0.42 -> map_range(0..1 -> 0..100, clamp) -> 42.0
                    assert!((*v - 42.0).abs() < 1e-3, "got {v}");
                }
                other => panic!("unexpected args: {other:?}"),
            }
        }
        other => panic!("unexpected packet: {other:?}"),
    }

    // Verify the sender's log recorded the packet as sent.
    let log = sender.log_snapshot();
    assert_eq!(log.len(), 1);
    assert!(log[0].sent);
    assert_eq!(log[0].address, "/integration/x");
}
