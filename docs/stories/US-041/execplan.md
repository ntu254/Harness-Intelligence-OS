# Exec Plan

## Goal

Add a full agent workflow example that a new user can read after the README and
clean clone walkthrough.

## Scope

In scope:

- Create `docs/examples/full-agent-workflow.md`.
- Link the example from README and `docs/README.md`.
- Include command samples and expected output snippets.
- Include provider troubleshooting for CodeGraph and NotebookLM.
- Extend the adoption docs verifier.
- Record validation evidence and pass story governance.

Out of scope:

- Agent instruction packs.
- Troubleshooting guide.
- Command cookbook.
- Release/tag/installer pin changes.
- Provider live calls.
- CLI feature changes.

## Risk Classification

Risk flags:

- Existing behavior: changes public adoption docs.
- Weak proof: example docs can drift from command names.

Hard gates:

- None.

Lane: normal.

## Work Phases

1. Open GitHub issue, intake, and story row.
2. Draft the full workflow example.
3. Link it from README and docs map.
4. Extend adoption docs verifier.
5. Run validation.
6. Record trace.
7. Run story governance gate.
8. Commit, push, and close issue.

## Stop Conditions

Pause for human confirmation if:

- The example would require a new CLI command.
- The example would require a live external provider.
- The example would claim provider `inconclusive` as successful proof.
- Release or installer behavior must change.

