//! Application-wide shared state.

use crate::contracts::{
    HardcodedMapping, RuntimeSnapshot, RuntimeStatus, SourceMode, WorkerState,
};
use crate::fake::FakeSource;
use crate::osc::OscSender;
use crate::runtime::patches::PatchStore;
use crate::runtime::{compile, GraphRuntime};
use crate::signals::SignalBus;
use crate::worker::Worker;
use parking_lot::Mutex;
use std::sync::Arc;

pub struct AppState {
    pub bus: SignalBus,
    pub worker: Worker,
    pub osc: OscSender,
    pub fake: Mutex<Option<FakeSource>>,
    pub source_mode: Mutex<SourceMode>,
    pub hardcoded: Mutex<HardcodedMapping>,
    pub runtime: Arc<Mutex<GraphRuntime>>,
    pub loaded_patch: Mutex<Option<String>>,
    pub patches: PatchStore,
    pub runtime_snapshot: Mutex<RuntimeSnapshot>,
    pub calibration: Mutex<Option<CalibrationSession>>,
}

pub struct CalibrationSession {
    pub signal_path: String,
    pub min: f64,
    pub max: f64,
    pub samples: usize,
    pub started_monotonic_ms: u64,
}

impl AppState {
    pub fn new() -> Self {
        let bus = SignalBus::new();
        let osc = OscSender::new().expect("osc bind");
        let worker = Worker::new();

        // Start fake source by default so a fresh app can see signals immediately.
        let fake = FakeSource::start(bus.clone());

        Self {
            bus,
            worker,
            osc,
            fake: Mutex::new(Some(fake)),
            source_mode: Mutex::new(SourceMode::Fake),
            hardcoded: Mutex::new(HardcodedMapping::default()),
            runtime: Arc::new(Mutex::new(GraphRuntime::empty())),
            loaded_patch: Mutex::new(None),
            patches: PatchStore::new_default(),
            runtime_snapshot: Mutex::new(RuntimeSnapshot::default()),
            calibration: Mutex::new(None),
        }
    }

    pub fn status(&self) -> RuntimeStatus {
        let worker_state = self.worker.state();
        let source_mode = *self.source_mode.lock();
        let last_frame_monotonic_ms = self.bus.latest_frame_monotonic_ms();
        let last_error = match worker_state {
            WorkerState::Error => self.worker.last_error(),
            _ => None,
        };
        RuntimeStatus {
            worker_state,
            source_mode,
            last_frame_monotonic_ms,
            last_error,
            osc_last_send_monotonic_ms: self
                .osc
                .log_snapshot()
                .last()
                .map(|e| e.monotonic_ms),
            signals_count: self.bus.count(),
            loaded_patch: self.loaded_patch.lock().clone(),
        }
    }

    pub fn set_source_mode_fake(&self) {
        let mut guard = self.fake.lock();
        if guard.is_none() {
            *guard = Some(FakeSource::start(self.bus.clone()));
        }
        *self.source_mode.lock() = SourceMode::Fake;
    }

    pub fn set_source_mode_worker(&self) {
        if let Some(f) = self.fake.lock().take() {
            f.stop();
        }
        *self.source_mode.lock() = SourceMode::Worker;
    }

    pub fn reload_runtime_from_loaded_patch(&self) -> Result<(), String> {
        let name = match self.loaded_patch.lock().clone() {
            Some(n) => n,
            None => {
                *self.runtime.lock() = GraphRuntime::empty();
                return Ok(());
            }
        };
        let doc = self
            .patches
            .load(&name)
            .map_err(|e| format!("load patch: {e}"))?;
        let compiled = compile::compile(&doc)?;
        *self.runtime.lock() = compiled;
        Ok(())
    }
}
