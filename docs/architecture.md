# XENØr Core Architecture

XENØr Core models token incentives as a deterministic graph propagation system.

Participants and protocol components are represented as nodes in a directed graph.

Incentive relationships are represented as edges.

Reward propagation follows deterministic routing rules.

---

## Conceptual model

```text
        Protocol
           │
        Node A
        /    \
    Node B  Node C
```

Propagation is deterministic.

Given the same graph state and input events, the incentive distribution will always produce the same result.

---

## Key design goals

- deterministic propagation
- auditable incentive flows
- simulation-first design
- protocol-native reward routing

---

## System layers

```text
xenor-core
│
├─ graph model
├─ propagation engine
├─ incentive routing logic
└─ deterministic execution layer
```
