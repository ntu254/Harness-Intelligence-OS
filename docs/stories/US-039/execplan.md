# Exec Plan

## Goal

Make a first-time clean clone path explicit enough that a new user can reach a
passing local governance loop without prior Harness context.

## Scope

In scope:

- Create `docs/adoption/clean-clone-walkthrough.md`.
- Link it from `README.md` and `docs/README.md`.
- Add a lightweight adoption docs verifier.
- Record US-039 intake, story, validation evidence, and trace.
- Keep runtime evidence ignored and out of commit.

Out of scope:

- Full README quickstart rewrite.
- Command cookbook.
- Troubleshooting guide.
- Agent instruction packs.
- Release/tag/installer pin changes.

## Risk Classification

Risk flags:

- Existing behavior: changes user-facing onboarding docs.
- Weak proof: docs can drift from CLI behavior without a verifier.

Hard gates:

- None.

Lane: normal.

## Work Phases

1. Create v0.7 milestone and US-039 issue.
2. Record intake and story durable rows.
3. Probe a clean local runtime database with `HARNESS_DB`.
4. Write the walkthrough from the proven command path.
5. Add verifier coverage for required adoption docs.
6. Run validation and story gate.
7. Commit and push source changes.

## Stop Conditions

Pause for human confirmation if:

- The walkthrough would need new installer behavior.
- The story requires release/tag changes.
- Governance requirements need to be weakened to pass.
- Provider credentials or session files would need to be stored by Harness.

