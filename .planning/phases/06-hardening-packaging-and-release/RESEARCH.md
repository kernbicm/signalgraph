# RESEARCH

## What usually breaks late
- sidecar permissions and paths
- worker startup assumptions
- camera permissions in packaged builds
- stale logs and vague error messages
- graph migration bugs from saved patches

## Hardening priorities
- deterministic replay tests
- integration tests around OSC output
- smoke tests for patch load and runtime startup
- crash-safe worker restarts
- first-run troubleshooting docs

## Release rule
Do not add major new nodes or tracker modes here. This phase is about making the existing loop trustworthy.
