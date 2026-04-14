# STATE

## Current status
- Project artifacts drafted
- No implementation started
- All phases marked `not_started`

## Key decisions already made
1. Assume **React Flow** is the intended visual graph library.
2. Use a **Python MediaPipe worker** for v1.
3. Use **Rust** as the authoritative orchestration, graph-runtime, and OSC layer.
4. Start with a **DAG-only** graph model in v1.
5. Land a **hard-coded vertical slice** before introducing the graph editor.

## Risks to watch early
- Browser camera preview device IDs may not line up cleanly with the worker's camera indexing.
- Running preview and inference from separate camera owners may fail or behave inconsistently on some systems.
- MediaPipe output schemas are rich; the app needs a stable internal signal contract before graphing begins.
- It is easy to overbuild the graph system. v1 only needs enough nodes to make real mappings useful.

## Decision triggers
- If the preview and worker cannot reliably share the same camera, promote worker-owned preview before Phase 3 exits.
- If tracker latency or CPU usage is too high, reduce preview fidelity before cutting mapping features.
- If graph complexity starts bloating, freeze the node set and move anything nonessential to backlog.

## Backlog seeds
- Face blendshape nodes
- Script/custom-code node
- Network target discovery
- Plugin SDK
- Feedback loops / time-based graph nodes

## Suggested first acceptance demo
Open the app, pick a camera, raise a hand, observe one live source value, map it to 0..1, and send it to `/demo/hand/x` over OSC.
