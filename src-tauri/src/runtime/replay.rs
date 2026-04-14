//! Deterministic replay harness. Loads a newline-delimited fixture of
//! `TelemetryFrame` records and feeds them into a SignalBus + runtime,
//! returning captured OSC sink outputs so tests can assert the full
//! pipeline end-to-end.

use crate::contracts::{GraphDocument, TelemetryFrame};
use crate::runtime::{compile, nodes::SinkOutput, GraphRuntime};
use crate::signals::SignalBus;

#[derive(Debug, Clone)]
pub struct ReplayResult {
    pub ticks: usize,
    pub frame_outputs: Vec<Vec<SinkOutput>>,
    pub errors: Vec<String>,
}

/// Parse a JSONL fixture into a vector of frames.
pub fn parse_fixture(text: &str) -> Result<Vec<TelemetryFrame>, String> {
    let mut frames = Vec::new();
    for (i, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let frame: TelemetryFrame = serde_json::from_str(line)
            .map_err(|e| format!("line {}: {}", i + 1, e))?;
        frames.push(frame);
    }
    Ok(frames)
}

/// Run a compiled runtime over a sequence of telemetry frames.
pub fn run_doc(doc: &GraphDocument, frames: &[TelemetryFrame]) -> Result<ReplayResult, String> {
    let runtime = compile::compile(doc)?;
    Ok(run_compiled(runtime, frames))
}

pub fn run_compiled(mut runtime: GraphRuntime, frames: &[TelemetryFrame]) -> ReplayResult {
    let bus = SignalBus::new();
    let mut frame_outputs = Vec::with_capacity(frames.len());
    let mut errors = Vec::new();
    for frame in frames {
        bus.ingest(frame);
        let (snapshot, sinks) = runtime.tick(&bus, frame.monotonic_ms);
        if !snapshot.errors.is_empty() {
            errors.extend(snapshot.errors);
        }
        frame_outputs.push(sinks);
    }
    ReplayResult {
        ticks: frames.len(),
        frame_outputs,
        errors,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::OscPayload;
    use crate::runtime::starter::{multi_sink_patch, starter_patch};

    const HAND_SWEEP_FIXTURE: &str = include_str!("../../../fixtures/hand_left_sweep.jsonl");

    #[test]
    fn fixture_parses() {
        let frames = parse_fixture(HAND_SWEEP_FIXTURE).unwrap();
        assert_eq!(frames.len(), 11);
        assert_eq!(frames[0].signals.len(), 1);
    }

    #[test]
    fn starter_patch_runs_sweep_end_to_end() {
        let frames = parse_fixture(HAND_SWEEP_FIXTURE).unwrap();
        let result = run_doc(&starter_patch(), &frames).unwrap();
        assert_eq!(result.ticks, 11);
        assert!(result.errors.is_empty(), "errors: {:?}", result.errors);

        // Every tick must produce exactly one sink output (osc_out_1).
        for outputs in &result.frame_outputs {
            assert_eq!(outputs.len(), 1);
            assert_eq!(outputs[0].target.address, "/demo/hand/x");
        }

        // First frame feeds value 0.00 which clamps to out_min=0, then smoothed.
        let first = match result.frame_outputs[0][0].payload {
            OscPayload::Float(v) => v,
            _ => panic!("expected float"),
        };
        assert!((first - 0.0).abs() < 1e-4, "first = {first}");

        // By the final frame (value=1.00, above in_max=0.9), clamp=true caps
        // normalized input to 1.0; smoothed output converges toward 1.0.
        let last = match result.frame_outputs.last().unwrap()[0].payload {
            OscPayload::Float(v) => v,
            _ => panic!("expected float"),
        };
        assert!(last > 0.4, "smooth alpha=0.25 over 11 frames -> last={last}");
        assert!(last <= 1.0);
    }

    #[test]
    fn multi_sink_patch_drives_two_addresses() {
        let frames = parse_fixture(HAND_SWEEP_FIXTURE).unwrap();
        let result = run_doc(&multi_sink_patch(), &frames).unwrap();
        assert!(result.errors.is_empty(), "errors: {:?}", result.errors);
        for outputs in &result.frame_outputs {
            // One raw + one mirror sink per tick
            assert_eq!(outputs.len(), 2);
            let addresses: Vec<&str> =
                outputs.iter().map(|o| o.target.address.as_str()).collect();
            assert!(addresses.contains(&"/raw/x"));
            assert!(addresses.contains(&"/mirror/x"));
        }
        // At t=0 raw=0, mirror=invert(0)=1 (min=0,max=1 -> 1-0+0=1)
        let first_raw = result.frame_outputs[0]
            .iter()
            .find(|o| o.target.address == "/raw/x")
            .unwrap();
        let first_mirror = result.frame_outputs[0]
            .iter()
            .find(|o| o.target.address == "/mirror/x")
            .unwrap();
        assert!(matches!(first_raw.payload, OscPayload::Float(v) if v.abs() < 1e-4));
        assert!(matches!(first_mirror.payload, OscPayload::Float(v) if (v - 1.0).abs() < 1e-4));
    }

    #[test]
    fn missing_source_does_not_panic() {
        // Build a patch whose tracker signal path never appears in the fixture.
        let mut doc = starter_patch();
        doc.nodes[0].config =
            serde_json::json!({ "signal_path": "mp.never.going.to.exist" });
        let frames = parse_fixture(HAND_SWEEP_FIXTURE).unwrap();
        let result = run_doc(&doc, &frames).unwrap();
        assert_eq!(result.ticks, 11);
        // Downstream nodes will log errors about the None input, but the
        // runtime must survive. The sink either produces no output or
        // produces nothing for this frame - both are acceptable.
        assert!(!result.errors.is_empty());
    }
}
