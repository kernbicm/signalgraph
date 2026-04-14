# RESEARCH

## Why this phase is separate
The visual graph will create complexity fast:
- custom node rendering
- port typing
- inspector forms
- graph persistence
- invalid edge handling

Keeping it separate from runtime execution avoids hiding bugs under pretty UI.

## Editor responsibilities
- create and delete nodes
- connect and disconnect edges
- edit node configs
- validate obvious wiring mistakes
- save/load patch JSON
- display live node values when available

## Runtime responsibilities that stay out of the editor
- final graph validation
- topological ordering
- node evaluation
- OSC sending
