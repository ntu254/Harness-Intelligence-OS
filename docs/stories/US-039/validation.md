# Validation

## Proof Strategy

Prove the walkthrough with a temporary clean `HARNESS_DB`, then prevent docs
drift with an automated adoption-doc verifier.

US-039 is complete only when the walkthrough can explain:

- source clone CLI setup;
- local durable state initialization;
- local demo intake and story;
- context, architecture, trace, and story gate proof;
- governance report/dashboard export;
- trusted release verification;
- provider unavailability as inconclusive, not pass;
- ignored runtime artifacts.

## Test Plan

| Layer | Cases |
| --- | --- |
| Unit | Adoption verifier checks required sections and command phrases. |
| Integration | Temporary `HARNESS_DB` clean-flow smoke reaches story gate pass. |
| E2E | A reader can follow clone -> demo story -> dashboard from one walkthrough. |
| Platform | Walkthrough includes Bash and Windows PowerShell command forms. |
| Logs/Audit | Trace records adoption story work and validation evidence. |

## Fixtures

- Temporary `HARNESS_DB` path outside the repo.
- Local story id `US-DEMO`.
- Runtime outputs under ignored `.harness/`.

## Commands

```text
cargo fmt --check
cargo test --workspace
python scripts/verify-adoption-docs.py
harness-cli context --story US-039
harness-cli arch-check --story US-039
harness-cli story verify US-039
```

Clean-flow probe:

```text
HARNESS_DB=<temp-db> harness-cli init
HARNESS_DB=<temp-db> harness-cli import brownfield
HARNESS_DB=<temp-db> harness-cli intake --story US-DEMO ...
HARNESS_DB=<temp-db> harness-cli story add --id US-DEMO ...
HARNESS_DB=<temp-db> harness-cli context --story US-DEMO
HARNESS_DB=<temp-db> harness-cli arch-check --story US-DEMO
HARNESS_DB=<temp-db> harness-cli trace --story US-DEMO ...
HARNESS_DB=<temp-db> harness-cli story verify US-DEMO
HARNESS_DB=<temp-db> harness-cli governance report ...
HARNESS_DB=<temp-db> harness-cli governance dashboard ...
```

## Acceptance Criteria

- GitHub milestone `HI-OS v0.7.0: Adoption Ready` exists.
- GitHub issue for US-039 exists.
- Intake and story durable rows exist.
- Clean clone walkthrough exists.
- README links to the walkthrough.
- `docs/README.md` maps the adoption folder.
- `scripts/README.md` mentions the adoption docs verifier.
- Walkthrough explains clean clone local durable state.
- Walkthrough includes Bash and PowerShell commands.
- Walkthrough reaches story gate and dashboard evidence.
- Walkthrough explains provider-unavailable behavior.
- Walkthrough explains what not to commit.
- `python scripts/verify-adoption-docs.py` passes.
- Story governance gate passes.

## Acceptance Evidence

- GitHub milestone created: `HI-OS v0.7.0: Adoption Ready`.
- GitHub issue created: `#17`.
- Intake recorded: `#28`.
- Story recorded: `US-039`, lane `normal`, verify command
  `python scripts/verify-adoption-docs.py`.
- Clean clone walkthrough added:
  `docs/adoption/clean-clone-walkthrough.md`.
- README links to `docs/adoption/clean-clone-walkthrough.md`.
- `docs/README.md` maps the `adoption/` folder.
- `scripts/README.md` documents `python scripts/verify-adoption-docs.py`.
- Adoption docs verifier added: `scripts/verify-adoption-docs.py`.
- Temporary clean `HARNESS_DB` probe passed:
  - `init` applied schema;
  - `import brownfield` imported 12 decisions and 0 story rows, documenting
    that clean clone story history is local runtime state;
  - local `US-DEMO` intake and story were created;
  - `context --story US-DEMO` passed;
  - `arch-check --story US-DEMO` passed;
  - detailed local trace recorded;
  - `story verify US-DEMO` passed mechanical verification and governance gate;
  - governance report and dashboard exported with 1 story and 1 gate pass.
- `cargo fmt --check` passed.
- `cargo test --workspace` passed: 49 tests.
- `python scripts/verify-adoption-docs.py` passed.
- `harness-cli context --story US-039` passed.
- `harness-cli arch-check --story US-039` passed.
- Detailed trace recorded: `#38`, score `3/3`.
- `harness-cli story verify US-039` passed.
- Story governance gate passed.
- No release/tag/installer pin change.
- No CodeGraph or NotebookLM provider call.
- Runtime artifacts remain under ignored local paths.
