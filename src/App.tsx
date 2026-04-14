import { useEffect, useState } from "react";
import { TrackingLab } from "./views/TrackingLab";
import { PatchEditor } from "./views/PatchEditor";
import { PacketMonitor } from "./views/PacketMonitor";
import { startPolling, useAppStore } from "./lib/store";
import { isTauri } from "./lib/tauri";
import { StatusPanel } from "./components/StatusPanel";

type Tab = "lab" | "editor" | "monitor";

export function App() {
  const tick = useAppStore((s) => s.tick);
  const [tab, setTab] = useState<Tab>("lab");

  useEffect(() => {
    if (!isTauri()) return;
    void tick();
    const stop = startPolling(100);
    return stop;
  }, [tick]);

  return (
    <div className="app-shell">
      <header className="app-header">
        <h1>SignalGraph</h1>
        <span className="header-sub">camera → MediaPipe → signal → graph → OSC</span>
        <nav className="tabs">
          <TabButton label="Lab" active={tab === "lab"} onClick={() => setTab("lab")} />
          <TabButton
            label="Editor"
            active={tab === "editor"}
            onClick={() => setTab("editor")}
          />
          <TabButton
            label="Monitor"
            active={tab === "monitor"}
            onClick={() => setTab("monitor")}
          />
        </nav>
        <div className="header-status">
          <StatusPanel compact />
        </div>
      </header>
      <main className="app-main">
        {tab === "lab" ? <TrackingLab /> : null}
        {tab === "editor" ? <PatchEditor /> : null}
        {tab === "monitor" ? <PacketMonitor /> : null}
      </main>
    </div>
  );
}

function TabButton({
  label,
  active,
  onClick,
}: {
  label: string;
  active: boolean;
  onClick: () => void;
}) {
  return (
    <button
      className={`tab-button ${active ? "tab-active" : ""}`}
      onClick={onClick}
    >
      {label}
    </button>
  );
}
