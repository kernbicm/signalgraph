import { useEffect } from "react";
import { StatusPanel } from "./components/StatusPanel";
import { SourceExplorer } from "./components/SourceExplorer";
import { OscLoopbackPanel } from "./components/OscLoopbackPanel";
import { startPolling, useAppStore } from "./lib/store";
import { isTauri } from "./lib/tauri";

export function App() {
  const tick = useAppStore((s) => s.tick);

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
      </header>
      <main className="app-main phase1-lab">
        <section className="panel">
          <h2>Runtime</h2>
          <StatusPanel />
        </section>
        <section className="panel">
          <h2>OSC loopback</h2>
          <OscLoopbackPanel />
        </section>
        <section className="panel source-panel">
          <h2>Source explorer</h2>
          <SourceExplorer />
        </section>
      </main>
    </div>
  );
}
