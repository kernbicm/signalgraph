"""SignalGraph tracking worker.

Emits newline-delimited JSON `TelemetryFrame` records on stdout.

Phase 1: prints deterministic fake frames so the Rust reader path can be
smoke-tested without OpenCV or MediaPipe installed.

Phase 2: when --live is passed, opens the selected camera and runs
MediaPipe Hand Landmarker, emitting adapted signals under the
`mp.hand.*` namespace.

Protocol:
    { "schema_version": 1,
      "source": "worker.hand",
      "monotonic_ms": 12345,
      "signals": { "<path>": {"kind": "float", "value": 0.42}, ... } }

stdout is reserved for frames. stderr is for human-readable logs.
"""
from __future__ import annotations

import argparse
import json
import math
import sys
import time
from typing import Any, Dict


SCHEMA_VERSION = 1


def log(msg: str) -> None:
    print(msg, file=sys.stderr, flush=True)


def emit(frame: Dict[str, Any]) -> None:
    sys.stdout.write(json.dumps(frame, separators=(",", ":")))
    sys.stdout.write("\n")
    sys.stdout.flush()


def make_frame(monotonic_ms: int, source: str) -> Dict[str, Any]:
    return {
        "schema_version": SCHEMA_VERSION,
        "source": source,
        "monotonic_ms": monotonic_ms,
        "signals": {},
    }


def set_signal(frame: Dict[str, Any], path: str, kind: str, value: Any) -> None:
    frame["signals"][path] = {"kind": kind, "value": value}


# ---------- fake mode ----------

def run_fake(fps: float) -> None:
    """Deterministic signal pattern - mirrors what the Rust fake source produces."""
    start = time.monotonic()
    while True:
        t = time.monotonic() - start
        monotonic_ms = int(t * 1000)
        frame = make_frame(monotonic_ms, "worker.fake")
        ix = 0.5 + 0.5 * math.sin(t * 1.2)
        iy = 0.5 + 0.5 * math.sin(t * 0.7 + 1.1)
        wx = 0.5 + 0.5 * math.sin(t * 0.9 + 0.3)
        wy = 0.5 + 0.5 * math.sin(t * 0.5 + 2.2)
        gs = 0.5 + 0.5 * math.cos(t * 0.4)
        set_signal(frame, "mp.hand.left.landmark.index_tip.x", "float", ix)
        set_signal(frame, "mp.hand.left.landmark.index_tip.y", "float", iy)
        set_signal(frame, "mp.hand.left.landmark.wrist.x", "float", wx)
        set_signal(frame, "mp.hand.left.landmark.wrist.y", "float", wy)
        set_signal(frame, "mp.hand.left.gesture.score", "float", gs)
        set_signal(frame, "mp.hand.left.gesture.category", "category", "none")
        emit(frame)
        time.sleep(max(0.0, 1.0 / fps))


# ---------- live MediaPipe mode ----------

def run_live(camera_index: int, fps_cap: float) -> None:
    try:
        import cv2  # type: ignore
    except Exception as exc:
        log(f"opencv import failed: {exc}. Falling back to fake mode.")
        run_fake(fps_cap)
        return
    try:
        import mediapipe as mp  # type: ignore
    except Exception as exc:
        log(f"mediapipe import failed: {exc}. Falling back to fake mode.")
        run_fake(fps_cap)
        return

    cap = cv2.VideoCapture(camera_index)
    if not cap.isOpened():
        log(f"cannot open camera {camera_index}, falling back to fake mode")
        run_fake(fps_cap)
        return

    mp_hands = mp.solutions.hands
    hands = mp_hands.Hands(
        max_num_hands=2,
        model_complexity=0,
        min_detection_confidence=0.5,
        min_tracking_confidence=0.5,
    )
    log(f"worker live mode on camera {camera_index}")

    start = time.monotonic()
    min_frame_time = 1.0 / max(fps_cap, 1.0)
    while True:
        t0 = time.monotonic()
        ok, frame_bgr = cap.read()
        if not ok:
            log("camera read failed")
            time.sleep(0.1)
            continue
        frame_rgb = cv2.cvtColor(frame_bgr, cv2.COLOR_BGR2RGB)
        results = hands.process(frame_rgb)

        monotonic_ms = int((time.monotonic() - start) * 1000)
        frame = make_frame(monotonic_ms, "worker.hand")

        if results.multi_hand_landmarks:
            handedness_list = results.multi_handedness or []
            for idx, landmarks in enumerate(results.multi_hand_landmarks):
                hand_label = "left"
                if idx < len(handedness_list):
                    hand_label = handedness_list[idx].classification[0].label.lower()
                prefix = f"mp.hand.{hand_label}.landmark"
                for name, landmark_idx in LANDMARK_MAP.items():
                    lm = landmarks.landmark[landmark_idx]
                    set_signal(frame, f"{prefix}.{name}.x", "float", float(lm.x))
                    set_signal(frame, f"{prefix}.{name}.y", "float", float(lm.y))
                    set_signal(frame, f"{prefix}.{name}.z", "float", float(lm.z))
                # Gesture score placeholder (MediaPipe Gesture Recognizer is a separate
                # task; for the vertical slice we fake a score from index_tip proximity).
                index_tip = landmarks.landmark[8]
                wrist = landmarks.landmark[0]
                dist = math.sqrt(
                    (index_tip.x - wrist.x) ** 2
                    + (index_tip.y - wrist.y) ** 2
                    + (index_tip.z - wrist.z) ** 2
                )
                gesture_score = max(0.0, min(1.0, 1.0 - dist * 2.0))
                set_signal(frame, f"mp.hand.{hand_label}.gesture.score", "float", gesture_score)
                set_signal(
                    frame,
                    f"mp.hand.{hand_label}.gesture.category",
                    "category",
                    "open" if gesture_score > 0.5 else "closed",
                )

        emit(frame)

        elapsed = time.monotonic() - t0
        if elapsed < min_frame_time:
            time.sleep(min_frame_time - elapsed)


LANDMARK_MAP = {
    "wrist": 0,
    "thumb_tip": 4,
    "index_tip": 8,
    "middle_tip": 12,
    "ring_tip": 16,
    "pinky_tip": 20,
}


def main() -> int:
    parser = argparse.ArgumentParser(prog="signalgraph-worker")
    parser.add_argument("--live", action="store_true", help="use MediaPipe + camera")
    parser.add_argument("--camera", type=int, default=0, help="camera device index")
    parser.add_argument("--fps", type=float, default=30.0, help="target frame rate")
    args = parser.parse_args()

    try:
        if args.live:
            run_live(args.camera, args.fps)
        else:
            run_fake(args.fps)
    except KeyboardInterrupt:
        log("worker interrupted")
    return 0


if __name__ == "__main__":
    sys.exit(main())
