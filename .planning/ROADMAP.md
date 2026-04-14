# ROADMAP

## Milestone 1
**Goal:** ship a v1 build that can convert at least one live MediaPipe value into a user-defined OSC output through a visual graph.

## Recommended branch strategy
Use **phase-based branching** for Phases 2 through 6. Phase 1 can stay on the default branch if it is only scaffolding.

## Phase overview

| Phase | Title | Depends on | Why it exists | Key exit gate |
|---|---|---:|---|---|
| 1 | Bootstrap and runtime contracts | - | Sets repo structure, sidecar wiring, schemas, logging, and test harnesses | App boots, fake worker stream works, OSC loopback smoke test passes |
| 2 | Vertical slice: camera, MediaPipe, OSC | 1 | Proves the full stack with a hard-coded mapping before visual graphing | Live tracker value reaches OSC output |
| 3 | React Flow editor shell | 2 | Introduces visual patch authoring and patch persistence | User can draw and save a valid source->transform->output graph |
| 4 | Graph runtime and mapping nodes | 3 | Moves mapping logic into a proper Rust graph evaluator | Runtime executes saved graphs deterministically |
| 5 | OSC routing, calibration, presets, monitoring | 4 | Makes the tool practical for performers and creative coders | Multiple sink nodes, calibration, and packet monitor work |
| 6 | Hardening, packaging, release docs | 5 | Turns the prototype into a shippable desktop build | Packaged app passes end-to-end acceptance test |

## Phase notes

### Phase 1
- Keep it brutally simple.
- Build the contracts before you build the UI.
- Create at least one fake telemetry producer so later phases can progress without a camera.

### Phase 2
- Treat this as the make-or-break technical phase.
- Prefer one tracker profile done well over three half-working ones.
- Preserve the hard-coded OSC mapping even after the graph editor arrives.

### Phase 3
- React Flow is an editor shell only.
- Do not evaluate graphs in React state.
- Persist graph JSON early and often.

### Phase 4
- Enforce DAG validation before runtime execution.
- Separate graph validation from graph evaluation.
- Add replay-based tests before adding fancy transform nodes.

### Phase 5
- This is where the tool becomes useful instead of merely impressive.
- Spend time on calibration, monitoring, and clear error surfaces.
- Build starter patches for common "map value to OSC address" workflows.

### Phase 6
- Package the worker cleanly.
- Verify startup, shutdown, logging, and failure modes.
- Ship docs that help a user fix the three dumbest problems quickly: camera permissions, wrong device selection, wrong OSC host/port.

## GSD execution cadence
- For UI-heavy phases, run:
  - `/gsd-discuss-phase N`
  - `/gsd-ui-phase N`
  - `/gsd-plan-phase N`
  - `/gsd-execute-phase N`
  - `/gsd-verify-work N`
- For backend-heavy phases, skip `/gsd-ui-phase N`.
- Do not re-run full execute on a completed phase for tiny fixes; use the quick path.

## Out-of-sequence rule
If Phase 2 reveals camera ownership problems between preview and worker inference, insert a narrow subphase before Phase 3 to unify preview transport. Do not paper over it and hope the packaging gods feel merciful.
