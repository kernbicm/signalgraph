import type { GraphDocument } from "../lib/types";

function mkPatch(
  name: string,
  description: string,
  signalPath: string,
  address: string,
): GraphDocument {
  const now = new Date().toISOString();
  return {
    schema_version: 1,
    id: name,
    name,
    description,
    created_at: now,
    updated_at: now,
    nodes: [
      {
        id: "tracker_signal_1",
        kind: "tracker_signal",
        label: signalPath.split(".").slice(-3).join("."),
        config: { signal_path: signalPath },
        position: [80, 160],
      },
      {
        id: "map_range_1",
        kind: "map_range",
        label: "normalize",
        config: {
          in_min: 0.1,
          in_max: 0.9,
          out_min: 0,
          out_max: 1,
          clamp: true,
          invert: false,
        },
        position: [360, 160],
      },
      {
        id: "smooth_1",
        kind: "smooth",
        label: "smooth",
        config: { alpha: 0.25 },
        position: [640, 160],
      },
      {
        id: "osc_out_1",
        kind: "osc_out",
        label: address,
        config: {
          host: "127.0.0.1",
          port: 9000,
          address,
          payload_type: "float",
          enabled: true,
        },
        position: [920, 160],
      },
    ],
    edges: [
      {
        id: "e1",
        source: "tracker_signal_1",
        source_port: "out",
        target: "map_range_1",
        target_port: "input",
      },
      {
        id: "e2",
        source: "map_range_1",
        source_port: "out",
        target: "smooth_1",
        target_port: "input",
      },
      {
        id: "e3",
        source: "smooth_1",
        source_port: "out",
        target: "osc_out_1",
        target_port: "input",
      },
    ],
  };
}

export interface Template {
  id: string;
  label: string;
  description: string;
  build: () => GraphDocument;
}

export const TEMPLATES: Template[] = [
  {
    id: "hand_x",
    label: "Hand X → /demo/hand/x",
    description: "Index-tip x position, smoothed, to /demo/hand/x",
    build: () =>
      mkPatch(
        "hand_x",
        "Hand index tip X to /demo/hand/x",
        "mp.hand.left.landmark.index_tip.x",
        "/demo/hand/x",
      ),
  },
  {
    id: "wrist_y",
    label: "Wrist Y → /demo/wrist/y",
    description: "Wrist y position, smoothed, to /demo/wrist/y",
    build: () =>
      mkPatch(
        "wrist_y",
        "Hand wrist Y to /demo/wrist/y",
        "mp.hand.left.landmark.wrist.y",
        "/demo/wrist/y",
      ),
  },
  {
    id: "gesture_score",
    label: "Gesture score → /demo/gesture/score",
    description: "Open-hand gesture score to /demo/gesture/score",
    build: () =>
      mkPatch(
        "gesture_score",
        "Gesture score to /demo/gesture/score",
        "mp.hand.left.gesture.score",
        "/demo/gesture/score",
      ),
  },
];
