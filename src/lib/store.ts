import { create } from "zustand";
import type {
  GraphDocument,
  PacketLogEntry,
  RuntimeSnapshot,
  RuntimeStatus,
  SignalDescriptor,
} from "./types";
import { api, isTauri } from "./tauri";

interface AppStateShape {
  status: RuntimeStatus | null;
  signals: SignalDescriptor[];
  snapshot: RuntimeSnapshot | null;
  packetLog: PacketLogEntry[];
  currentPatch: GraphDocument | null;
  patchList: string[];
  tick: () => Promise<void>;
  refreshPatchList: () => Promise<void>;
  setCurrentPatch: (doc: GraphDocument | null) => void;
}

export const useAppStore = create<AppStateShape>((set, get) => ({
  status: null,
  signals: [],
  snapshot: null,
  packetLog: [],
  currentPatch: null,
  patchList: [],

  async tick() {
    if (!isTauri()) {
      return;
    }
    try {
      const [status, signals, snapshot, packetLog] = await Promise.all([
        api.getRuntimeStatus(),
        api.listSignals(),
        api.runtimeSnapshot(),
        api.packetLog(),
      ]);
      set({ status, signals, snapshot, packetLog });
    } catch (e) {
      console.error("tick failed", e);
    }
  },

  async refreshPatchList() {
    if (!isTauri()) return;
    try {
      const patchList = await api.listPatches();
      set({ patchList });
    } catch (e) {
      console.error("refreshPatchList failed", e);
    }
  },

  setCurrentPatch(doc) {
    set({ currentPatch: doc });
  },
}));

export function startPolling(intervalMs = 100): () => void {
  const handle = setInterval(() => {
    void useAppStore.getState().tick();
  }, intervalMs);
  return () => clearInterval(handle);
}
