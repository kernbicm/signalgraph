# Project: SignalGraph

## Working title
SignalGraph is a desktop creative-coding tool built with Tauri, Rust, a Python MediaPipe worker, and a React Flow editor. It turns live human-tracking data into named signals, lets the user visually transform those signals, and sends the result to OSC targets.

## Assumptions
- "Rect Flow" is treated as **React Flow**.
- Desktop is the only target for v1.
- The first release optimizes for a single local performer on one machine.
- v1 must prove the full loop before it expands the tracker matrix:
  `camera -> MediaPipe -> normalized signal -> graph transform -> OSC packet`.

## Core product promise
A user can pick a live MediaPipe value such as `hand.left.index_tip.x`, visually map and smooth it, and send it to a chosen OSC address such as `/fx/filter/cutoff` without writing code.

## Architecture principles
1. **React Flow is the editor, not the runtime.**
   - The graph canvas is a visual authoring surface.
   - The authoritative graph evaluator lives in Rust.
   - The frontend edits graph JSON and observes runtime state.

2. **Rust owns orchestration and transport.**
   - Rust manages app state, sidecar lifecycle, graph validation, graph evaluation, presets, and OSC output.
   - Rust is the only layer allowed to emit production OSC packets.

3. **Python owns MediaPipe for v1.**
   - The worker handles camera capture, inference, and compact telemetry output.
   - Raw MediaPipe structures are adapted into a stable internal schema before the UI or graph sees them.

4. **Start as a DAG.**
   - v1 supports only acyclic graphs.
   - No feedback loops, user scripting, or custom code nodes in v1.
   - Statefulness is limited to explicit runtime nodes such as `smooth`, `hold`, or `rate_limit`.

5. **Telemetry stays compact.**
   - Do not shuttle raw video frames through the normal app data path unless a phase explicitly introduces a preview transport.
   - UI subscriptions should consume typed signal snapshots, not giant payload blobs.

6. **Every phase must preserve the vertical slice.**
   - At any point after Phase 2, the project should still be able to send at least one live OSC value from a MediaPipe source.

## User stories
- As a creative coder, I want to route hand and pose values to visual software over OSC.
- As a performer, I want confidence that the graph I build sends stable, predictable values.
- As a tinkerer, I want a node-based patching surface instead of editing JSON by hand.
- As a builder, I want reusable presets for common mappings and target apps.

## Success criteria for v1
- The app shows a working camera preview.
- The app can read at least one live MediaPipe tracker profile.
- The app exposes tracker outputs as named source nodes.
- The user can visually build a patch:
  `Source -> Transform -> OSC Out`.
- The app can save and reload patches.
- The app can emit correct OSC packets to a user-chosen host and port.
- The app remains responsive while tracking and routing live data.

## Explicit non-goals for v1
- Cross-machine collaboration
- Arbitrary code execution inside nodes
- Cyclic graphs / feedback loops
- Plugin SDK for third-party nodes
- Network discovery of OSC targets
- GPU optimization work beyond obvious low-risk wins

## Canonical internal signal examples
- `mp.hand.left.landmark.index_tip.x`
- `mp.hand.left.landmark.index_tip.y`
- `mp.hand.left.gesture.category`
- `mp.hand.left.gesture.score`
- `mp.pose.landmark.left_wrist.x`
- `mp.pose.landmark.left_wrist.visibility`

## Example patch
`mp.hand.left.landmark.index_tip.x`
-> `map_range(0.10..0.90 => 0.00..1.00)`
-> `smooth(alpha=0.2)`
-> `osc_out(address="/fx/filter/cutoff", type=float)`

## Delivery strategy
- De-risk the stack in this order:
  1. bootstrap and contracts
  2. end-to-end vertical slice with hard-coded mapping
  3. visual graph editor
  4. runtime graph evaluation
  5. richer OSC features and presets
  6. hardening and packaging
