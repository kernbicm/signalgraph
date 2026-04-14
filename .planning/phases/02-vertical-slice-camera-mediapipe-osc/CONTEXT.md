# CONTEXT

## Goal
Prove the full product loop with the fewest moving parts:
camera preview, one real MediaPipe tracker profile, a compact signal adapter, and one live OSC mapping.

## Phase priority
This phase is not about elegance. It is about proving that the core loop works on real hardware.

## Tracker strategy
- Start with **one tracker profile** that gives strong creative leverage.
- Recommended default: hand tracking / gesture profile.
- Keep the worker contract generic enough that pose can fit later.

## Preview strategy
- Start with a frontend preview if it gets you to a stable demo fastest.
- If preview and worker camera ownership conflict on the target platform, stop and fix ownership before Phase 3 grows around a broken assumption.

## Required visible demo
A live tracker value updates on screen and drives an OSC packet in real time.
