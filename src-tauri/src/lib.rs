pub mod contracts;
pub mod signals;
pub mod fake;
pub mod osc;
pub mod worker;
pub mod runtime;
pub mod app_state;
pub mod commands;

use tracing_subscriber::{fmt, EnvFilter};

pub fn run() {
    init_tracing();
    tracing::info!("signalgraph starting");

    let state = app_state::AppState::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            commands::get_runtime_status,
            commands::start_worker,
            commands::stop_worker,
            commands::set_source_mode,
            commands::list_signals,
            commands::read_signal,
            commands::osc_loopback_test,
            commands::send_hardcoded_mapping,
            commands::set_hardcoded_mapping,
            commands::get_hardcoded_mapping,
            commands::load_patch,
            commands::save_patch,
            commands::list_patches,
            commands::delete_patch,
            commands::validate_patch,
            commands::runtime_snapshot,
            commands::packet_log,
            commands::calibrate_start,
            commands::calibrate_stop,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,signalgraph_lib=debug"));
    let _ = fmt()
        .with_env_filter(filter)
        .with_target(true)
        .try_init();
}
