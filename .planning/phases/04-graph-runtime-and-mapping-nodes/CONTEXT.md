# CONTEXT

## Goal
Turn saved patch documents into live runtime behavior in Rust.

## Hard rules
- Rust evaluates graphs.
- Graphs are DAGs only.
- Evaluation order must be deterministic.
- Runtime errors must be surfaced clearly to the UI.

## Mandatory transform behavior
Numeric transforms must behave predictably and be tested heavily. The first release lives or dies on boring math being correct.

## Mandatory runtime capabilities
- graph validation
- topological sort
- typed port/value handling
- per-node evaluation
- debug snapshots
- graceful handling of missing source values
