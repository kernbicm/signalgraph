# 03-UI-SPEC

## Screen
**Patch Editor**

## Layout
- Left sidebar: node palette grouped by Source, Transform, Output, Debug.
- Center: React Flow canvas with minimap and controls.
- Right sidebar: inspector for selected node or edge.
- Bottom dock: live runtime status and graph errors.

## Node design rules
- Inputs on the left, outputs on the right.
- Node header shows node type and short label.
- Numeric live values should be visible without opening the inspector where practical.
- Invalid or unbound nodes must look obviously unfinished.

## Interaction rules
- Drag from palette to canvas to create a node.
- Clicking a node opens inspector fields in the right panel.
- Invalid connections should be blocked, not merely warned about later.
- Saving should be automatic or one-click obvious.
- Include a starter patch button that creates a minimal working graph.

## Required inspector fields
- Source node:
  - signal path
  - label
- MapRange:
  - input min/max
  - output min/max
  - invert toggle
  - clamp toggle
- Smooth:
  - alpha or smoothing factor
- OSC Out:
  - label
  - host
  - port
  - address
  - payload type
