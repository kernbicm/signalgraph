//! Tauri IPC commands. Keep this layer thin: validate args, delegate to state.

use crate::app_state::{AppState, CalibrationSession};
use crate::contracts::{
    GraphDocument, HardcodedMapping, OscPayload, PacketLogEntry, RuntimeSnapshot,
    RuntimeStatus, SourceMode,
};
use crate::osc::loopback_test;
use crate::runtime::{compile, validate};
use crate::signals::SignalDescriptor;
use tauri::State;

#[tauri::command]
pub fn get_runtime_status(state: State<AppState>) -> RuntimeStatus {
    state.status()
}

#[tauri::command]
pub fn list_signals(state: State<AppState>) -> Vec<SignalDescriptor> {
    state.bus.list_descriptors()
}

#[tauri::command]
pub fn read_signal(state: State<AppState>, path: String) -> Option<f64> {
    state.bus.read_float(&path)
}

#[tauri::command]
pub fn set_source_mode(state: State<AppState>, mode: SourceMode) -> Result<(), String> {
    match mode {
        SourceMode::Fake => state.set_source_mode_fake(),
        SourceMode::Worker => state.set_source_mode_worker(),
    }
    Ok(())
}

#[tauri::command]
pub fn start_worker(state: State<AppState>, camera: Option<String>) -> Result<(), String> {
    state.set_source_mode_worker();
    state.worker.start(state.bus.clone(), camera)
}

#[tauri::command]
pub fn stop_worker(state: State<AppState>) -> Result<(), String> {
    state.worker.stop();
    Ok(())
}

#[tauri::command]
pub fn osc_loopback_test() -> Result<String, String> {
    loopback_test()
}

#[tauri::command]
pub fn send_hardcoded_mapping(state: State<AppState>) -> Result<String, String> {
    let mapping = state.hardcoded.lock().clone();
    if !mapping.enabled {
        return Err("hardcoded mapping disabled".into());
    }
    let value = state
        .bus
        .read_float(&mapping.source)
        .ok_or_else(|| format!("signal {} not available", mapping.source))?;
    let t = crate::runtime::nodes::map_range(
        value,
        mapping.in_min,
        mapping.in_max,
        mapping.out_min,
        mapping.out_max,
        true,
        mapping.invert,
    );
    let payload = match mapping.payload_type.as_str() {
        "int" => OscPayload::Int(t as i32),
        "bool" => OscPayload::Bool(t > 0.5),
        _ => OscPayload::Float(t as f32),
    };
    let monotonic_ms = state.bus.latest_frame_monotonic_ms().unwrap_or(0);
    state
        .osc
        .send(&mapping.target, &payload, Some("hardcoded".into()), monotonic_ms)
        .map(|_| format!("{} -> {:?}", mapping.target.address, payload))
}

#[tauri::command]
pub fn set_hardcoded_mapping(
    state: State<AppState>,
    mapping: HardcodedMapping,
) -> Result<(), String> {
    *state.hardcoded.lock() = mapping;
    Ok(())
}

#[tauri::command]
pub fn get_hardcoded_mapping(state: State<AppState>) -> HardcodedMapping {
    state.hardcoded.lock().clone()
}

#[tauri::command]
pub fn list_patches(state: State<AppState>) -> Vec<String> {
    state.patches.list()
}

#[tauri::command]
pub fn load_patch(state: State<AppState>, name: String) -> Result<GraphDocument, String> {
    let doc = state.patches.load(&name)?;
    *state.loaded_patch.lock() = Some(name);
    let runtime = compile::compile(&doc)?;
    *state.runtime.lock() = runtime;
    Ok(doc)
}

#[tauri::command]
pub fn save_patch(state: State<AppState>, doc: GraphDocument) -> Result<(), String> {
    validate::validate(&doc)?;
    state.patches.save(&doc)?;
    *state.loaded_patch.lock() = Some(doc.name.clone());
    let runtime = compile::compile(&doc)?;
    *state.runtime.lock() = runtime;
    Ok(())
}

#[tauri::command]
pub fn delete_patch(state: State<AppState>, name: String) -> Result<(), String> {
    state.patches.delete(&name)
}

#[tauri::command]
pub fn validate_patch(doc: GraphDocument) -> Result<(), String> {
    validate::validate(&doc)?;
    compile::compile(&doc).map(|_| ())
}

/// Run one runtime tick and dispatch any emitted OSC messages.
#[tauri::command]
pub fn runtime_snapshot(state: State<AppState>) -> RuntimeSnapshot {
    let frame_monotonic = state.bus.latest_frame_monotonic_ms().unwrap_or(0);
    let (snapshot, sinks) = {
        let mut runtime = state.runtime.lock();
        runtime.tick(&state.bus, frame_monotonic)
    };
    for sink in sinks {
        let _ = state.osc.send(
            &sink.target,
            &sink.payload,
            Some(sink.label.clone()),
            frame_monotonic,
        );
    }
    update_calibration(&state, frame_monotonic);
    *state.runtime_snapshot.lock() = snapshot.clone();
    snapshot
}

#[tauri::command]
pub fn packet_log(state: State<AppState>) -> Vec<PacketLogEntry> {
    state.osc.log_snapshot()
}

#[tauri::command]
pub fn calibrate_start(state: State<AppState>, signal_path: String) -> Result<(), String> {
    let frame_monotonic = state.bus.latest_frame_monotonic_ms().unwrap_or(0);
    *state.calibration.lock() = Some(CalibrationSession {
        signal_path,
        min: f64::INFINITY,
        max: f64::NEG_INFINITY,
        samples: 0,
        started_monotonic_ms: frame_monotonic,
    });
    Ok(())
}

#[tauri::command]
pub fn calibrate_stop(
    state: State<AppState>,
) -> Option<crate::contracts::CalibrationResult> {
    let session = state.calibration.lock().take();
    session.map(|s| crate::contracts::CalibrationResult {
        signal_path: s.signal_path,
        min: if s.min.is_finite() { s.min } else { 0.0 },
        max: if s.max.is_finite() { s.max } else { 1.0 },
        samples: s.samples,
    })
}

fn update_calibration(state: &State<AppState>, _frame_monotonic: u64) {
    let mut guard = state.calibration.lock();
    if let Some(session) = guard.as_mut() {
        if let Some(v) = state.bus.read_float(&session.signal_path) {
            if v < session.min {
                session.min = v;
            }
            if v > session.max {
                session.max = v;
            }
            session.samples += 1;
        }
    }
}
