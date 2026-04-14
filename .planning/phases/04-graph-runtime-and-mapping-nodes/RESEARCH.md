# RESEARCH

## Why this phase matters
A nice graph editor without a trustworthy runtime is just a colorful lie.

## Runtime design recommendation
- compile the patch document into an executable runtime graph
- validate once on patch load / patch change
- execute on each telemetry frame or tick
- produce:
  - node output cache
  - sink outputs
  - error list
  - timing metrics

## Starter value types
- float
- int
- bool
- string / category
- vec2 / vec3 only if immediately useful

## Out of scope for v1
- cyclic graphs
- user-defined code nodes
- asynchronous per-node execution
- distributed graph execution
