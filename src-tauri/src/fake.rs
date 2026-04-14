//! Deterministic fake telemetry producer. Runs when source_mode = Fake.
//!
//! Generates a small set of hand-ish signals moving through smooth sine curves
//! so Phases 3-6 can progress with no camera attached.

use crate::contracts::{SignalValue, TelemetryFrame};
use crate::signals::SignalBus;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

const FAKE_PATHS: &[&str] = &[
    "mp.hand.left.landmark.index_tip.x",
    "mp.hand.left.landmark.index_tip.y",
    "mp.hand.left.landmark.wrist.x",
    "mp.hand.left.landmark.wrist.y",
    "mp.hand.left.gesture.score",
];

pub struct FakeSource {
    stop: Arc<AtomicBool>,
}

impl FakeSource {
    pub fn start(bus: SignalBus) -> Self {
        let stop = Arc::new(AtomicBool::new(false));
        let stop_flag = stop.clone();
        std::thread::spawn(move || {
            let start = Instant::now();
            let mut frame_count: u64 = 0;
            while !stop_flag.load(Ordering::Relaxed) {
                let t = start.elapsed().as_secs_f64();
                let monotonic_ms = (t * 1000.0) as u64;
                let mut frame = TelemetryFrame::new("fake", monotonic_ms);

                // Soft phase-shifted signals in [0,1]
                let ix = 0.5 + 0.5 * (t * 1.2).sin();
                let iy = 0.5 + 0.5 * (t * 0.7 + 1.1).sin();
                let wx = 0.5 + 0.5 * (t * 0.9 + 0.3).sin();
                let wy = 0.5 + 0.5 * (t * 0.5 + 2.2).sin();
                let gs = 0.5 + 0.5 * (t * 0.4).cos();

                frame.signals.insert(FAKE_PATHS[0].into(), SignalValue::Float(ix));
                frame.signals.insert(FAKE_PATHS[1].into(), SignalValue::Float(iy));
                frame.signals.insert(FAKE_PATHS[2].into(), SignalValue::Float(wx));
                frame.signals.insert(FAKE_PATHS[3].into(), SignalValue::Float(wy));
                frame.signals.insert(FAKE_PATHS[4].into(), SignalValue::Float(gs));

                bus.ingest(&frame);
                frame_count += 1;
                if frame_count % 300 == 0 {
                    tracing::debug!(frame_count, "fake source producing");
                }
                std::thread::sleep(Duration::from_millis(33));
            }
            tracing::info!("fake source stopped");
        });
        Self { stop }
    }

    pub fn stop(&self) {
        self.stop.store(true, Ordering::Relaxed);
    }
}

impl Drop for FakeSource {
    fn drop(&mut self) {
        self.stop();
    }
}
