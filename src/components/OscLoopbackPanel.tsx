import { useState } from "react";
import { api } from "../lib/tauri";

export function OscLoopbackPanel() {
  const [result, setResult] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  return (
    <div className="loopback-panel">
      <button
        onClick={async () => {
          setError(null);
          setResult(null);
          try {
            const msg = await api.oscLoopbackTest();
            setResult(msg);
          } catch (e) {
            setError(String(e));
          }
        }}
      >
        run loopback test
      </button>
      {result ? <div className="ok-row">✓ {result}</div> : null}
      {error ? <div className="err-row">✗ {error}</div> : null}
      <p className="muted small">
        Sends one /loopback/test float to a throwaway UDP receiver and verifies
        it came back intact. Runs entirely on localhost.
      </p>
    </div>
  );
}
