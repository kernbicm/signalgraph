# CONTEXT

## Goal
Add a visual patch editor so the user can author mappings without hand-editing config.

## Hard rule
React Flow is the authoring surface. It is **not** the runtime evaluator.

## Graph policy for v1
- DAG only
- typed ports
- no executable code nodes
- no cycles
- no graph-level scripting
- one patch loaded at a time

## Mandatory starter node set
- `tracker_signal`
- `constant`
- `map_range`
- `clamp`
- `smooth`
- `debug_meter`
- `osc_out`

## UX outcome
A user can build this graph visually:
`tracker_signal -> map_range -> smooth -> osc_out`
