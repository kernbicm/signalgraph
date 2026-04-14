import { invoke } from "@tauri-apps/api/core";
import type {
  CalibrationResult,
  GraphDocument,
  HardcodedMapping,
  PacketLogEntry,
  RuntimeSnapshot,
  RuntimeStatus,
  SignalDescriptor,
  SourceMode,
} from "./types";

export const api = {
  getRuntimeStatus: () => invoke<RuntimeStatus>("get_runtime_status"),
  listSignals: () => invoke<SignalDescriptor[]>("list_signals"),
  readSignal: (path: string) =>
    invoke<number | null>("read_signal", { path }),
  setSourceMode: (mode: SourceMode) =>
    invoke<void>("set_source_mode", { mode }),
  startWorker: (camera: string | null) =>
    invoke<void>("start_worker", { camera }),
  stopWorker: () => invoke<void>("stop_worker"),
  oscLoopbackTest: () => invoke<string>("osc_loopback_test"),

  getHardcodedMapping: () =>
    invoke<HardcodedMapping>("get_hardcoded_mapping"),
  setHardcodedMapping: (mapping: HardcodedMapping) =>
    invoke<void>("set_hardcoded_mapping", { mapping }),
  sendHardcodedMapping: () =>
    invoke<string>("send_hardcoded_mapping"),

  listPatches: () => invoke<string[]>("list_patches"),
  loadPatch: (name: string) => invoke<GraphDocument>("load_patch", { name }),
  savePatch: (doc: GraphDocument) => invoke<void>("save_patch", { doc }),
  deletePatch: (name: string) => invoke<void>("delete_patch", { name }),
  validatePatch: (doc: GraphDocument) =>
    invoke<void>("validate_patch", { doc }),

  runtimeSnapshot: () => invoke<RuntimeSnapshot>("runtime_snapshot"),
  packetLog: () => invoke<PacketLogEntry[]>("packet_log"),

  calibrateStart: (signalPath: string) =>
    invoke<void>("calibrate_start", { signalPath }),
  calibrateStop: () =>
    invoke<CalibrationResult | null>("calibrate_stop"),
};

export function isTauri(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}
