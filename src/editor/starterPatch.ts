import type { GraphDocument } from "../lib/types";

export function createStarterPatch(): GraphDocument {
  const now = new Date().toISOString();
  return {
    schema_version: 1,
    id: "starter",
    name: "starter",
    description: "MediaPipe index_tip.x -> map_range -> smooth -> OSC /demo/hand/x",
    created_at: now,
    updated_at: now,
    nodes: [
      {
        id: "tracker_signal_1",
        kind: "tracker_signal",
        label: "hand.left.index_tip.x",
        config: { signal_path: "mp.hand.left.landmark.index_tip.x" },
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
        label: "/demo/hand/x",
        config: {
          host: "127.0.0.1",
          port: 9000,
          address: "/demo/hand/x",
          payload_type: "float",
          enabled: true,
        },
        position: [920, 160],
      },
    ],
    edges: [
      {
        id: "e-src-mr",
        source: "tracker_signal_1",
        source_port: "out",
        target: "map_range_1",
        target_port: "input",
      },
      {
        id: "e-mr-sm",
        source: "map_range_1",
        source_port: "out",
        target: "smooth_1",
        target_port: "input",
      },
      {
        id: "e-sm-osc",
        source: "smooth_1",
        source_port: "out",
        target: "osc_out_1",
        target_port: "input",
      },
    ],
  };
}
