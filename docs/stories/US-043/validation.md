# Validation

## Proof Strategy

US-043 is docs-only, but the troubleshooting guide must preserve governance
semantics. The adoption verifier checks required failure surfaces and guardrail
phrases.

## Test Plan

| Layer | Cases |
| --- | --- |
| Unit | Adoption verifier checks troubleshooting sections and guardrails. |
| Integration | README and docs map link the guide. |
| E2E | Guide covers installer -> release -> providers -> gate -> dashboard failures. |
| Platform | Bash and Windows PowerShell examples are present. |
| Logs/Audit | Trace records scope, validation, and non-goals. |

## Fixtures

- `docs/troubleshooting.md`.

## Commands

```text
cargo fmt --check
cargo test --workspace
python scripts/verify-adoption-docs.py
harness-cli context --story US-043
harness-cli arch-check --story US-043
harness-cli story verify US-043
```

## Acceptance Criteria

- `docs/troubleshooting.md` exists.
- Installer failures are covered.
- Release verify failures are covered.
- CodeGraph unavailable is covered.
- NotebookLM auth/session failures are covered.
- NotebookLM uncited output failure is covered.
- Governance gate failures are covered.
- Governance report/dashboard failures are covered.
- The guide preserves `fail` and `inconclusive` semantics.
- README links to the guide.
- `docs/README.md` links to the guide.
- `python scripts/verify-adoption-docs.py` passes.
- Story governance gate passes.

## Acceptance Evidence

- GitHub issue created: `#21`.
- Intake recorded: `#32`.
- Story recorded: `US-043`, lane `normal`, verify command
  `python scripts/verify-adoption-docs.py`.
- Troubleshooting guide added: `docs/troubleshooting.md`.
- README links to `docs/troubleshooting.md`.
- `docs/README.md` links to `docs/troubleshooting.md`.
- Guide covers installer failures, merge/dry-run recovery, release pins, and
  checksum mismatch behavior.
- Guide covers `release verify --version 0.6.0`, release trust failures, and
  network/GitHub `inconclusive` states.
- Guide covers CodeGraph unavailable and malformed provider output.
- Guide covers NotebookLM missing executable, auth/session, notebook, and
  network unavailable states.
- Guide covers NotebookLM uncited output and provenance failure.
- Guide covers governance gate failures and missing evidence recovery.
- Guide covers governance report/dashboard failures.
- Guide preserves `fail` and `inconclusive` semantics and says not to weaken
  the story gate.
- Guide preserves credential/session guardrails.
- Adoption docs verifier extended for troubleshooting contract:
  `scripts/verify-adoption-docs.py`.
- `cargo fmt --check` passed.
- `cargo test --workspace` passed: 49 tests.
- `python scripts/verify-adoption-docs.py` passed.
- `git diff --check` passed.
- `harness-cli context --story US-043` passed.
- `harness-cli arch-check --story US-043` passed.
- Detailed trace recorded: `#42`, score `3/3`.
- `harness-cli story verify US-043` passed.
- Story governance gate passed.
- No provider live call, release, tag, installer pin, or CLI behavior change.
