//! OSC transport layer. The only place production OSC packets are emitted.

use crate::contracts::{OscPayload, OscTarget, PacketLogEntry};
use parking_lot::Mutex;
use rosc::{encoder, OscMessage, OscPacket, OscType};
use std::collections::VecDeque;
use std::net::UdpSocket;
use std::sync::Arc;

const MAX_LOG_ENTRIES: usize = 256;

#[derive(Clone)]
pub struct OscSender {
    socket: Arc<UdpSocket>,
    log: Arc<Mutex<VecDeque<PacketLogEntry>>>,
}

impl OscSender {
    pub fn new() -> Result<Self, std::io::Error> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        Ok(Self {
            socket: Arc::new(socket),
            log: Arc::new(Mutex::new(VecDeque::with_capacity(MAX_LOG_ENTRIES))),
        })
    }

    pub fn send(
        &self,
        target: &OscTarget,
        payload: &OscPayload,
        source_label: Option<String>,
        monotonic_ms: u64,
    ) -> Result<(), String> {
        let msg = OscMessage {
            addr: target.address.clone(),
            args: vec![payload_to_osc_type(payload)],
        };
        let packet = OscPacket::Message(msg);
        let bytes = encoder::encode(&packet).map_err(|e| format!("encode error: {e:?}"))?;
        let addr = format!("{}:{}", target.host, target.port);
        let send_result = self.socket.send_to(&bytes, addr).map(|_| ());

        let (sent, error) = match &send_result {
            Ok(_) => (true, None),
            Err(e) => (false, Some(e.to_string())),
        };

        let entry = PacketLogEntry {
            monotonic_ms,
            host: target.host.clone(),
            port: target.port,
            address: target.address.clone(),
            payload_preview: payload_preview(payload),
            source_label,
            sent,
            error,
        };
        self.push_log(entry);
        send_result.map_err(|e| e.to_string())
    }

    pub fn log_snapshot(&self) -> Vec<PacketLogEntry> {
        self.log.lock().iter().cloned().collect()
    }

    fn push_log(&self, entry: PacketLogEntry) {
        let mut guard = self.log.lock();
        if guard.len() >= MAX_LOG_ENTRIES {
            guard.pop_front();
        }
        guard.push_back(entry);
    }
}

fn payload_to_osc_type(p: &OscPayload) -> OscType {
    match p {
        OscPayload::Float(v) => OscType::Float(*v),
        OscPayload::Int(v) => OscType::Int(*v),
        OscPayload::Bool(v) => OscType::Bool(*v),
        OscPayload::String(v) => OscType::String(v.clone()),
    }
}

fn payload_preview(p: &OscPayload) -> String {
    match p {
        OscPayload::Float(v) => format!("f32 {v:.4}"),
        OscPayload::Int(v) => format!("i32 {v}"),
        OscPayload::Bool(v) => format!("bool {v}"),
        OscPayload::String(v) => format!("\"{v}\""),
    }
}

/// Loopback test used for Phase 1 verification:
/// bind a receiver, send one packet to it, assert address + payload.
pub fn loopback_test() -> Result<String, String> {
    let receiver = UdpSocket::bind("127.0.0.1:0").map_err(|e| e.to_string())?;
    receiver
        .set_read_timeout(Some(std::time::Duration::from_millis(500)))
        .map_err(|e| e.to_string())?;
    let port = receiver.local_addr().map_err(|e| e.to_string())?.port();

    let sender = OscSender::new().map_err(|e| e.to_string())?;
    let target = OscTarget {
        host: "127.0.0.1".to_string(),
        port,
        address: "/loopback/test".to_string(),
    };
    sender
        .send(&target, &OscPayload::Float(0.5), Some("loopback".into()), 0)
        .map_err(|e| format!("send failed: {e}"))?;

    let mut buf = [0u8; 1024];
    let (size, _addr) = receiver.recv_from(&mut buf).map_err(|e| e.to_string())?;
    let packet = rosc::decoder::decode_udp(&buf[..size])
        .map_err(|e| format!("decode: {e:?}"))?
        .1;

    if let OscPacket::Message(msg) = packet {
        if msg.addr != "/loopback/test" {
            return Err(format!("wrong addr: {}", msg.addr));
        }
        if let Some(OscType::Float(v)) = msg.args.first() {
            if (*v - 0.5).abs() < 1e-4 {
                return Ok(format!("loopback ok on port {port}"));
            }
            return Err(format!("wrong float: {v}"));
        }
        Err("no float arg".to_string())
    } else {
        Err("bundle not expected".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loopback_round_trips() {
        let result = loopback_test().expect("loopback failed");
        assert!(result.contains("loopback ok"));
    }

    #[test]
    fn encode_various_payloads() {
        let sender = OscSender::new().unwrap();
        let target = OscTarget {
            host: "127.0.0.1".to_string(),
            port: 9, // discard port - typically no listener, sendto still succeeds
            address: "/x".to_string(),
        };
        let _ = sender.send(&target, &OscPayload::Int(7), None, 0);
        let _ = sender.send(&target, &OscPayload::Bool(true), None, 0);
        let _ = sender.send(&target, &OscPayload::String("hi".into()), None, 0);
        assert_eq!(sender.log_snapshot().len(), 3);
    }
}
