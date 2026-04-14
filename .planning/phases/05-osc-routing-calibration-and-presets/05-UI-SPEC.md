# 05-UI-SPEC

## Screen areas
- Patch editor remains central.
- Right inspector grows richer for output and calibration settings.
- Bottom panel becomes a packet monitor and runtime diagnostics surface.
- Add a patch browser for save/load/duplicate/import/export.

## Packet monitor
Show at least:
- time
- host:port
- OSC address
- payload preview
- source patch / node label
- send state

## Calibration UX
- For numeric source nodes or map nodes, allow:
  - start capture
  - stop capture
  - adopt captured min/max
  - reset
- Show live current value beside captured range so the user can tell whether they are calibrating nonsense.

## Error UX
- Invalid OSC settings should show inline validation.
- Disabled output nodes should look disabled on the canvas.
- Runtime send failures should appear in the diagnostics panel.
