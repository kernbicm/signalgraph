//! Python worker lifecycle. The worker is a sidecar process that emits
//! newline-delimited JSON TelemetryFrame records on stdout.
//!
//! Phase 1 ships the lifecycle plumbing and structured log capture.
//! Phase 2 plugs the real MediaPipe script underneath this same interface.

use crate::contracts::{TelemetryFrame, WorkerState};
use crate::signals::SignalBus;
use parking_lot::Mutex;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::JoinHandle;

pub struct Worker {
    inner: Arc<WorkerInner>,
}

struct WorkerInner {
    state: Mutex<WorkerState>,
    last_error: Mutex<Option<String>>,
    process: Mutex<Option<Child>>,
    stop_flag: AtomicBool,
    reader_handle: Mutex<Option<JoinHandle<()>>>,
}

impl Worker {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(WorkerInner {
                state: Mutex::new(WorkerState::Stopped),
                last_error: Mutex::new(None),
                process: Mutex::new(None),
                stop_flag: AtomicBool::new(false),
                reader_handle: Mutex::new(None),
            }),
        }
    }

    pub fn state(&self) -> WorkerState {
        *self.inner.state.lock()
    }

    pub fn last_error(&self) -> Option<String> {
        self.inner.last_error.lock().clone()
    }

    /// Start the worker process. If we can't resolve a runnable worker binary
    /// we transition to `Error` and store the reason. Sidecar packaging lands
    /// in Phase 6; dev invokes Python directly via `python worker/main.py`.
    pub fn start(&self, bus: SignalBus, selected_camera: Option<String>) -> Result<(), String> {
        if matches!(self.state(), WorkerState::Running | WorkerState::Starting) {
            return Err("worker already running".into());
        }
        *self.inner.state.lock() = WorkerState::Starting;
        *self.inner.last_error.lock() = None;
        self.inner.stop_flag.store(false, Ordering::Relaxed);

        let (program, mut args) = resolve_worker_invocation()?;
        if let Some(cam) = selected_camera {
            args.push("--camera".to_string());
            args.push(cam);
        }

        let mut cmd = Command::new(&program);
        cmd.args(&args);
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let mut child = match cmd.spawn() {
            Ok(c) => c,
            Err(e) => {
                let msg = format!("spawn failed ({}): {}", program.display(), e);
                *self.inner.state.lock() = WorkerState::Error;
                *self.inner.last_error.lock() = Some(msg.clone());
                return Err(msg);
            }
        };

        let stdout = child.stdout.take().ok_or_else(|| "no stdout".to_string())?;
        let stderr = child.stderr.take();
        *self.inner.process.lock() = Some(child);

        let inner = self.inner.clone();
        let handle = std::thread::spawn(move || {
            let reader = BufReader::new(stdout);
            *inner.state.lock() = WorkerState::Running;
            for line in reader.lines() {
                if inner.stop_flag.load(Ordering::Relaxed) {
                    break;
                }
                match line {
                    Ok(text) => {
                        if text.is_empty() {
                            continue;
                        }
                        match serde_json::from_str::<TelemetryFrame>(&text) {
                            Ok(frame) => bus.ingest(&frame),
                            Err(e) => {
                                tracing::debug!(error = ?e, line = %text, "non-frame line from worker");
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!(error = %e, "worker stdout read error");
                        break;
                    }
                }
            }
            if !inner.stop_flag.load(Ordering::Relaxed) {
                *inner.state.lock() = WorkerState::Error;
                *inner.last_error.lock() = Some("worker exited unexpectedly".into());
            } else {
                *inner.state.lock() = WorkerState::Stopped;
            }
        });
        *self.inner.reader_handle.lock() = Some(handle);

        // Drain stderr into tracing logs in the background.
        if let Some(err) = stderr {
            std::thread::spawn(move || {
                let reader = BufReader::new(err);
                for line in reader.lines().flatten() {
                    tracing::warn!(target = "worker_stderr", "{}", line);
                }
            });
        }
        Ok(())
    }

    pub fn stop(&self) {
        self.inner.stop_flag.store(true, Ordering::Relaxed);
        if let Some(mut child) = self.inner.process.lock().take() {
            let _ = child.kill();
            let _ = child.wait();
        }
        *self.inner.state.lock() = WorkerState::Stopped;
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Resolve either a bundled sidecar binary or a dev invocation of `python worker/main.py`.
fn resolve_worker_invocation() -> Result<(PathBuf, Vec<String>), String> {
    // Prefer a sidecar binary placed under src-tauri/binaries/signalgraph-worker(.exe)
    let exe_name = if cfg!(windows) {
        "signalgraph-worker.exe"
    } else {
        "signalgraph-worker"
    };
    let sidecar = Path::new("binaries").join(exe_name);
    if sidecar.exists() {
        return Ok((sidecar, vec![]));
    }
    // Fall back to `python worker/main.py` (dev mode)
    let worker_main = Path::new("../worker/main.py");
    if worker_main.exists() {
        let py = if cfg!(windows) { "python" } else { "python3" };
        return Ok((
            PathBuf::from(py),
            vec![worker_main.to_string_lossy().to_string()],
        ));
    }
    Err("worker binary or worker/main.py not found".into())
}
