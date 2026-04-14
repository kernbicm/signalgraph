# Start here

This pack is structured to fit the **GSD** `.planning/` layout and assumes your visual editor library is **React Flow**.

## What is included
- `.planning/PROJECT.md`
- `.planning/REQUIREMENTS.md`
- `.planning/ROADMAP.md`
- `.planning/STATE.md`
- one phase directory per phase under `.planning/phases/`
- `CONTEXT.md`, `RESEARCH.md`, and at least one `XX-YY-PLAN.md` per phase
- `UI-SPEC` files for UI-heavy phases

## Suggested Claude Code / GSD run order

### Phase 1
- `/gsd-discuss-phase 1`
- `/gsd-plan-phase 1`
- `/gsd-execute-phase 1`
- `/gsd-verify-work 1`

### Phase 2
- `/gsd-discuss-phase 2`
- `/gsd-ui-phase 2`
- `/gsd-plan-phase 2`
- `/gsd-execute-phase 2`
- `/gsd-verify-work 2`

### Phase 3
- `/gsd-discuss-phase 3`
- `/gsd-ui-phase 3`
- `/gsd-plan-phase 3`
- `/gsd-execute-phase 3`
- `/gsd-verify-work 3`

### Phase 4
- `/gsd-discuss-phase 4`
- `/gsd-plan-phase 4`
- `/gsd-execute-phase 4`
- `/gsd-verify-work 4`

### Phase 5
- `/gsd-discuss-phase 5`
- `/gsd-ui-phase 5`
- `/gsd-plan-phase 5`
- `/gsd-execute-phase 5`
- `/gsd-verify-work 5`

### Phase 6
- `/gsd-discuss-phase 6`
- `/gsd-plan-phase 6`
- `/gsd-execute-phase 6`
- `/gsd-verify-work 6`

## Recommended ground rules for Claude Code
- Do not move graph evaluation into the React frontend.
- Preserve the hard-coded vertical slice until Phase 4 is stable.
- Do not add cyclic graphs in v1.
- Prefer one tracker profile that really works over many broken ones.
- Treat calibration and packet monitoring as product features, not polish.

## Fast sanity demo after Phase 2
- open the app
- choose a camera
- start tracking
- select one live signal
- map it to 0..1
- send it to `/demo/hand/x`
- verify receipt in a local OSC monitor

## If Phase 2 blows up
Insert a subphase before Phase 3 and fix camera ownership or preview transport before the visual editor grows around a broken runtime assumption.
