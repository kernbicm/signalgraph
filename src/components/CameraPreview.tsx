import { useCallback, useEffect, useRef, useState } from "react";

interface Props {
  onDevicesChanged?: (devices: MediaDeviceInfo[]) => void;
}

const LAST_DEVICE_KEY = "signalgraph.lastCameraDeviceId";

export function CameraPreview({ onDevicesChanged }: Props) {
  const videoRef = useRef<HTMLVideoElement | null>(null);
  const streamRef = useRef<MediaStream | null>(null);
  const [devices, setDevices] = useState<MediaDeviceInfo[]>([]);
  const [deviceId, setDeviceId] = useState<string>(
    () => localStorage.getItem(LAST_DEVICE_KEY) ?? "",
  );
  const [error, setError] = useState<string | null>(null);
  const [permissionDenied, setPermissionDenied] = useState(false);

  const stopStream = useCallback(() => {
    if (streamRef.current) {
      for (const track of streamRef.current.getTracks()) {
        track.stop();
      }
      streamRef.current = null;
    }
    if (videoRef.current) {
      videoRef.current.srcObject = null;
    }
  }, []);

  const startStream = useCallback(
    async (wantDeviceId?: string) => {
      setError(null);
      stopStream();
      try {
        const constraints: MediaStreamConstraints = {
          video: wantDeviceId
            ? { deviceId: { exact: wantDeviceId } }
            : { facingMode: "user" },
          audio: false,
        };
        const stream = await navigator.mediaDevices.getUserMedia(constraints);
        streamRef.current = stream;
        if (videoRef.current) {
          videoRef.current.srcObject = stream;
          try {
            await videoRef.current.play();
          } catch {
            // user-gesture-required autoplay; silent ignore
          }
        }
        // Refresh device list after permission grant so labels populate.
        const list = await navigator.mediaDevices.enumerateDevices();
        const videoDevices = list.filter((d) => d.kind === "videoinput");
        setDevices(videoDevices);
        onDevicesChanged?.(videoDevices);

        const actualId =
          stream.getVideoTracks()[0]?.getSettings().deviceId ?? wantDeviceId;
        if (actualId) {
          setDeviceId(actualId);
          localStorage.setItem(LAST_DEVICE_KEY, actualId);
        }
      } catch (e: unknown) {
        const err = e as { name?: string; message?: string };
        if (err?.name === "NotAllowedError" || err?.name === "SecurityError") {
          setPermissionDenied(true);
          setError("camera permission denied");
        } else {
          setError(err?.message ?? String(e));
        }
      }
    },
    [onDevicesChanged, stopStream],
  );

  useEffect(() => {
    void startStream(deviceId || undefined);
    const onChange = () => {
      void navigator.mediaDevices.enumerateDevices().then((list) => {
        const videoDevices = list.filter((d) => d.kind === "videoinput");
        setDevices(videoDevices);
        onDevicesChanged?.(videoDevices);
      });
    };
    navigator.mediaDevices.addEventListener?.("devicechange", onChange);
    return () => {
      navigator.mediaDevices.removeEventListener?.("devicechange", onChange);
      stopStream();
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <div className="camera-preview">
      <div className="preview-video-wrapper">
        <video
          ref={videoRef}
          playsInline
          muted
          autoPlay
          className="preview-video"
        />
        {permissionDenied ? (
          <div className="permission-overlay">
            <p>Camera permission denied.</p>
            <button onClick={() => void startStream(deviceId || undefined)}>
              retry
            </button>
          </div>
        ) : null}
      </div>
      <div className="preview-controls">
        <label>
          device
          <select
            value={deviceId}
            onChange={(e) => void startStream(e.target.value)}
          >
            {devices.length === 0 ? (
              <option value="">(no cameras)</option>
            ) : null}
            {devices.map((d, i) => (
              <option key={d.deviceId || i} value={d.deviceId}>
                {d.label || `camera ${i + 1}`}
              </option>
            ))}
          </select>
        </label>
        {error ? <span className="err-row">{error}</span> : null}
      </div>
    </div>
  );
}
