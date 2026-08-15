# axiom-node

Process-level orchestrator. `axiom-node` does not link the other repositories as libraries; it invokes their stable
CLIs, keeping each component independently replaceable.

> **Maturity:** research prototype v0.1. The default verifier proves properties by exhaustive evaluation over an
> explicitly finite input domain. A VALID receipt is therefore a theorem about that bounded model, not a claim of
> unbounded program correctness.

```bash
cargo run -- demo --bin-dir ../bin --work-dir ./work
```

Expected executable names: `axiom-spec`, `axiom-synth`, `axiom-verifier`, `axiom-runtime`, `axiom-proof`.
