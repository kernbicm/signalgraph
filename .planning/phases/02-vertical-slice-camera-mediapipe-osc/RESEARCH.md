# RESEARCH

## Phase focus
This phase de-risks four things:
1. camera permissions and device selection
2. live inference loop
3. telemetry adaptation into stable signals
4. live OSC output

## Signal adapter rule
The worker can emit rich data, but the app must promote only stable, named signals into the graph/runtime layer.

Examples:
- `mp.hand.left.landmark.index_tip.x`
- `mp.hand.left.landmark.index_tip.y`
- `mp.hand.left.gesture.category`
- `mp.hand.left.gesture.score`

## Common failure modes to design around
- device mismatch between preview and worker
- preview works but worker crashes silently
- tracker emits too much raw data with no stable naming
- OSC works locally but the UI gives no confirmation

## Phase guardrail
Do not wait for the React Flow editor before proving value mapping. Hard-code one mapping first.
