# Validation

## Proof Strategy

README is documentation, but it still needs executable guardrails. US-040 uses
the adoption docs verifier to prevent the README from losing its quickstart,
workflow, trust, provider, and walkthrough commitments.

## Test Plan

| Layer | Cases |
| --- | --- |
| Unit | Adoption verifier checks required README and walkthrough phrases. |
| Integration | Story context and architecture checks pass after README rewrite. |
| E2E | README describes install -> intake -> context -> verify -> trace -> dashboard. |
| Platform | README includes Bash and Windows PowerShell command shapes. |
| Logs/Audit | Trace records issue, intake, story, validation, and non-goals. |

## Fixtures

- `README.md`
- `docs/adoption/clean-clone-walkthrough.md`
- `scripts/verify-adoption-docs.py`

## Commands

```text
cargo fmt --check
cargo test --workspace
python scripts/verify-adoption-docs.py
harness-cli context --story US-040
harness-cli arch-check --story US-040
harness-cli story verify US-040
```

## Acceptance Criteria

- README uses HI-OS as the primary identity.
- README includes a five-minute quickstart.
- README shows the core workflow: intake, context, verify, trace, dashboard.
- README links to `docs/adoption/clean-clone-walkthrough.md`.
- README explains `release verify`.
- README explains the governance dashboard.
- README explains CodeGraph and NotebookLM at overview level.
- README states provider unavailable is `inconclusive`, not `pass`.
- README states Harness does not store Google credentials or provider sessions.
- `python scripts/verify-adoption-docs.py` passes.
- Story governance gate passes.

## Acceptance Evidence

- GitHub issue created: `#18`.
- Intake recorded: `#29`.
- Story recorded: `US-040`, lane `normal`, verify command
  `python scripts/verify-adoption-docs.py`.
- README rewritten with HI-OS as the primary identity.
- README includes `5-Minute Quickstart`.
- README documents the core workflow:
  `intake -> context -> validation proof -> trace -> governance dashboard`.
- README links to `docs/adoption/clean-clone-walkthrough.md`.
- README explains `release verify --version 0.6.0` and the default public
  origin `ntu254/Harness-Intelligence-OS`.
- README explains governance report/dashboard output.
- README explains CodeGraph and NotebookLM as file-based provider boundaries.
- README states provider unavailable is `inconclusive`, not `pass`.
- README states Harness does not store Google credentials, cookies, tokens,
  browser profiles, or provider session files.
- Adoption docs verifier extended for README contract:
  `scripts/verify-adoption-docs.py`.
- `cargo fmt --check` passed.
- `cargo test --workspace` passed: 49 tests.
- `python scripts/verify-adoption-docs.py` passed.
- `git diff --check` passed.
- `harness-cli context --story US-040` passed.
- `harness-cli arch-check --story US-040` passed.
- Detailed trace recorded: `#39`, score `3/3`.
- `harness-cli story verify US-040` passed.
- Story governance gate passed.
- No release/tag/installer pin change.
- No CodeGraph or NotebookLM provider call.
- Sovereign identity work remains out of scope for US-046.
