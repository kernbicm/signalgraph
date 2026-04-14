import { CameraPreview } from "../components/CameraPreview";
import { HardcodedMappingPanel } from "../components/HardcodedMappingPanel";
import { OscLoopbackPanel } from "../components/OscLoopbackPanel";
import { SourceExplorer } from "../components/SourceExplorer";
import { TrackerControls } from "../components/TrackerControls";

export function TrackingLab() {
  return (
    <div className="tracking-lab">
      <section className="panel panel-preview">
        <h2>Camera preview</h2>
        <CameraPreview />
      </section>
      <section className="panel panel-tracker">
        <h2>Tracker controls</h2>
        <TrackerControls />
      </section>
      <section className="panel panel-sources">
        <h2>Source explorer</h2>
        <SourceExplorer />
      </section>
      <section className="panel panel-mapping">
        <h2>Hardcoded mapping (regression path)</h2>
        <HardcodedMappingPanel />
      </section>
      <section className="panel panel-loopback">
        <h2>OSC loopback</h2>
        <OscLoopbackPanel />
      </section>
    </div>
  );
}
