# Validation

## Proof Strategy

Validate US-041 by checking the example contract with
`scripts/verify-adoption-docs.py`, then run the normal Harness story evidence
chain.

## Test Plan

| Layer | Cases |
| --- | --- |
| Unit | Adoption verifier checks required example commands and troubleshooting sections. |
| Integration | README and docs map link the example. |
| E2E | Example covers intake through governance dashboard. |
| Platform | Example includes Windows PowerShell where command syntax differs. |
| Logs/Audit | Trace records scope, validation, and non-goals. |

## Fixtures

- `docs/examples/full-agent-workflow.md`.
- Fictional local story id `US-EXAMPLE`.
- Provider-unavailable examples for CodeGraph and NotebookLM.

## Commands

```text
cargo fmt --check
cargo test --workspace
python scripts/verify-adoption-docs.py
harness-cli context --story US-041
harness-cli arch-check --story US-041
harness-cli story verify US-041
```

## Acceptance Criteria

- `docs/examples/full-agent-workflow.md` exists.
- Example demonstrates one story from intake to dashboard.
- Example has command samples.
- Example has short expected output snippets.
- Example includes troubleshooting for missing CodeGraph provider.
- Example includes troubleshooting for missing NotebookLM auth/session.
- Example states provider `inconclusive` is not `pass`.
- README links to the example.
- `docs/README.md` maps `docs/examples/`.
- `python scripts/verify-adoption-docs.py` passes.
- Story governance gate passes.

## Acceptance Evidence

- GitHub issue created: `#19`.
- Intake recorded: `#30`.
- Story recorded: `US-041`, lane `normal`, verify command
  `python scripts/verify-adoption-docs.py`.
- Full workflow example added:
  `docs/examples/full-agent-workflow.md`.
- README links to `docs/examples/full-agent-workflow.md`.
- `docs/README.md` maps `docs/examples/`.
- Example demonstrates `US-EXAMPLE` from context reading through intake,
  story creation, optional provider context, context pack, validation, proof
  flags, trace, story verify, governance report, and governance dashboard.
- Example includes Bash and Windows PowerShell command samples.
- Example includes expected output snippets for intake, story creation,
  context generation, validation, trace, story verify, report, and dashboard.
- Example includes CodeGraph unavailable troubleshooting.
- Example includes NotebookLM auth/session troubleshooting.
- Example states provider `inconclusive` is not `pass`.
- Example states Harness must not store Google credentials, cookies, tokens,
  browser profiles, or provider session files.
- Adoption docs verifier extended for the example contract:
  `scripts/verify-adoption-docs.py`.
- `cargo fmt --check` passed.
- `cargo test --workspace` passed: 49 tests.
- `python scripts/verify-adoption-docs.py` passed.
- `git diff --check` passed.
- `harness-cli context --story US-041` passed.
- `harness-cli arch-check --story US-041` passed.
- Detailed trace recorded: `#40`, score `3/3`.
- `harness-cli story verify US-041` passed.
- Story governance gate passed.
- No release/tag/installer pin change.
- No CodeGraph or NotebookLM provider live call.
- No CLI feature change.
