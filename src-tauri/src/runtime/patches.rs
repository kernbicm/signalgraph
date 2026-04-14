//! Patch persistence on disk. Each patch is a JSON file inside a user data dir.
//!
//! Phase 1 uses a local `./patches/` directory so dev mode has a known
//! location. Phase 6 will switch to the Tauri appDataDir.

use crate::contracts::GraphDocument;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct PatchStore {
    root: PathBuf,
}

impl PatchStore {
    pub fn new_default() -> Self {
        let root = PathBuf::from("patches");
        if !root.exists() {
            let _ = fs::create_dir_all(&root);
        }
        Self { root }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn list(&self) -> Vec<String> {
        let mut names = Vec::new();
        if let Ok(entries) = fs::read_dir(&self.root) {
            for e in entries.flatten() {
                if let Some(ext) = e.path().extension() {
                    if ext == "json" {
                        if let Some(stem) = e.path().file_stem().and_then(|s| s.to_str()) {
                            names.push(stem.to_string());
                        }
                    }
                }
            }
        }
        names.sort();
        names
    }

    pub fn load(&self, name: &str) -> Result<GraphDocument, String> {
        let path = self.root.join(format!("{name}.json"));
        let bytes = fs::read(&path).map_err(|e| format!("read {path:?}: {e}"))?;
        serde_json::from_slice(&bytes).map_err(|e| format!("parse {path:?}: {e}"))
    }

    pub fn save(&self, doc: &GraphDocument) -> Result<(), String> {
        let path = self.root.join(format!("{}.json", doc.name));
        let bytes = serde_json::to_vec_pretty(doc).map_err(|e| e.to_string())?;
        let mut file = fs::File::create(&path).map_err(|e| format!("create {path:?}: {e}"))?;
        file.write_all(&bytes).map_err(|e| format!("write {path:?}: {e}"))?;
        Ok(())
    }

    pub fn delete(&self, name: &str) -> Result<(), String> {
        let path = self.root.join(format!("{name}.json"));
        if path.exists() {
            fs::remove_file(&path).map_err(|e| format!("delete {path:?}: {e}"))?;
        }
        Ok(())
    }
}
