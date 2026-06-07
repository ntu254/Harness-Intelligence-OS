# Exec Plan

## Goal

Rewrite README so a new reader understands HI-OS and can start the core
workflow in about five minutes.

## Scope

In scope:

- Rewrite `README.md` around HI-OS identity and adoption.
- Include a short quickstart.
- Include the core workflow: intake, context, verify, trace, dashboard.
- Link to `docs/adoption/clean-clone-walkthrough.md`.
- Explain release verify, governance dashboard, CodeGraph, and NotebookLM.
- Extend `scripts/verify-adoption-docs.py`.
- Record story validation evidence and run the story governance gate.

Out of scope:

- Full agent workflow example.
- Agent instruction packs.
- Troubleshooting guide.
- Command cookbook.
- Sovereign identity CLI/config work.
- v0.7 release/tag/installer pin changes.

## Risk Classification

Risk flags:

- Existing behavior: changes the public README.
- Weak proof: README adoption promises can drift from actual commands.

Hard gates:

- None.

Lane: normal.

## Work Phases

1. Record GitHub issue, intake, and story row.
2. Rewrite README.
3. Add story packet.
4. Extend adoption verifier for README contract.
5. Run validation.
6. Record trace.
7. Run story governance gate.
8. Commit, push, and close issue.

## Stop Conditions

Pause for human confirmation if:

- README would need to change release origin or installer pin.
- README would claim provider evidence passes without a real provider.
- Sovereign identity changes become necessary.
- Validation requirements need to be weakened.

