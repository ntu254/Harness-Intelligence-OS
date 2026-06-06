# Validation

## Proof Strategy

Prove the adapter as a grounded-context producer and the existing US-024 command
as the trust boundary. Tests must distinguish provider/session unavailability,
uncited summaries, malformed artifacts, source-provenance failures, and valid
grounded evidence.

Implementation must exercise at least one real NotebookLM-compatible provider
or export path before claiming completion. Planning alone must not pass the
story gate.

## Test Plan

| Layer | Cases |
| --- | --- |
| Unit | Provider response mapping, source hashes, citations, claims, unavailable and error status mapping |
| Integration | Generated artifact validates against US-023 and passes through US-024 ingest without direct SQLite access |
| E2E | Adapter produces artifact, ingest records pass, context pack renders grounded context, story gate accepts required NotebookLM proof |
| Platform | Path, process/session, exit-code, and JSON behavior on supported adapter platforms |
| Failure | Missing provider/session/source is inconclusive; uncited claims, malformed output, missing provenance, and hash mismatch are fail |
| Logs/Audit | No credentials in output; raw response reference, artifact, SHA256, ingest report, SQLite summary, and Detailed trace are inspectable |

## Fixtures

- Passing grounded brief with at least one source and cited claim.
- Passing brief with constraints, open questions, and affected docs.
- Provider unavailable or session expired response.
- Source unavailable response.
- Summary with no citations.
- Claim citation pointing to an unknown source.
- Source hash mismatch.
- Malformed or incomplete provider response.

## Commands

Adapter command:

```text
harness-cli notebooklm brief \
  --story US-026 \
  --notebook <provider-notebook-id-or-alias> \
  --query "Find grounded product rules and prior decisions relevant to US-026." \
  --timeout 120 \
  --output .harness/context/US-026-notebooklm-brief.json \
  --raw-output .harness/context/US-026-notebooklm-provider-response.json
```

Baseline validation:

```text
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
python scripts/verify-mcp-artifact-contracts.py
harness-cli notebooklm brief --story US-026 --notebook <provider-notebook-id-or-alias> --query <grounded-question>
harness-cli context ingest --story US-026 --source notebooklm --file <generated-artifact>
harness-cli context --story US-026
harness-cli arch-check --story US-026
harness-cli story verify US-026
```

## Acceptance Criteria

- NotebookLM-compatible provider contract is documented and accepted.
- Authentication/session behavior is documented without storing secrets.
- Raw response is captured or referenced safely.
- The adapter writes a schema-valid `notebooklm-brief` artifact.
- Passing artifacts contain required source provenance and citations.
- Uncited summaries or claims fail validation and cannot satisfy governance.
- Provider unavailable, permission denied, timeout, source unavailable, or
  insufficient evidence is inconclusive and cannot satisfy governance.
- The adapter never writes directly to Harness SQLite.
- The generated artifact passes through US-024 context ingest.
- Passing evidence updates grounded context and appears in the context pack.
- An explicitly configured NotebookLM requirement passes the story governance
  gate only after successful ingest.
- CodeGraph, auto-intake defaults, release/tag, and installer pin remain
  unchanged.

## Acceptance Evidence

Design-review evidence:

- Default provider contract accepted for implementation planning:
  `notebooklm-mcp-cli`.
- Default invocation boundary: local CLI executable `nlm`.
- Optional MCP boundary: `notebooklm-mcp`.
- Authentication/session behavior: interactive Google session managed by the
  provider and external to Harness.
- Harness storage rule: no Google credentials, cookies, browser profiles,
  tokens, or provider session files may be stored by Harness.
- Raw provider response must be captured or referenced safely before
  normalization.
- Normalized artifacts must preserve the US-023 schema shape using
  `provenance.sources[]` and `brief.claims[].citations[]`.
- NotebookLM summaries without claim-level citations are not grounded evidence.

Implementation evidence is added after provider invocation, adapter generation,
US-024 ingest, context pack rendering, and story governance verification.

Implementation evidence recorded so far:

- Story moved to In Progress after provider contract acceptance.
- CLI command added: `harness-cli notebooklm brief`.
- Adapter invokes `notebooklm-mcp-cli` through local executable `nlm`.
- Provider discovery corrected the invocation from the planned `nlm ask` shape
  to the actual `nlm query notebook --json <notebook> <question>` shape.
- Adapter writes `.harness/context/<story>-notebooklm-provider-response.json`
  when raw provider output is available.
- Adapter writes `.harness/context/<story>-notebooklm-brief.json`.
- Adapter composes generated artifacts with US-024 `context ingest --source
  notebooklm`.
- Missing executable/session/provider path records an inconclusive artifact and
  cannot satisfy governance.
- Provider stdout that cannot parse as grounded JSON records `fail`.
- Provider summaries or claims without citations record `fail`.
- Valid cited raw provider output normalizes into a schema-valid passing
  `notebooklm-brief` artifact in tests.
- Real `nlm query notebook --json` output shape (`answer`, `sources_used`,
  `citations`, `references`) normalizes into a schema-valid passing
  `notebooklm-brief` artifact in tests.
- Validation ladder passes locally: `cargo fmt --check`, `cargo test
  --workspace` with 42 tests, `cargo clippy --workspace --all-targets -- -D
  warnings`, `python scripts/verify-mcp-artifact-contracts.py`, and release
  build.
- Local smoke with missing executable produced inconclusive evidence at
  `.harness/context/US-026-notebooklm-brief.json` and
  `.harness/context/US-026-notebooklm-ingest-result.json`.
- `notebooklm-mcp-cli` `0.7.1` was installed locally and provides `nlm` plus
  `notebooklm-mcp`.
- `nlm login --check` reports that the default profile is missing, so live
  NotebookLM proof and final story gate remain intentionally unclaimed until a
  provider profile and notebook are available.
- Real provider smoke writes schema-valid inconclusive evidence because
  `nlm query notebook --json --timeout 120 hios-provider-proof <question>`
  returns `Error: Profile 'default' not found. Run 'nlm login' first.`.
- `harness-cli story verify US-026` now records mechanical verification pass
  and governance gate fail with only `NotebookLM context ingest proof` missing.
- No Google credentials, cookies, tokens, browser profiles, session files, MCP
  server direct writes, release/tag, installer pin, or CodeGraph changes.

## Failure Semantics

| Condition | Required result |
| --- | --- |
| Provider executable missing | `inconclusive` |
| Provider unauthenticated or expired session | `inconclusive` |
| Notebook/source not found | `inconclusive` |
| Network unavailable | `inconclusive` |
| Provider timeout | `inconclusive` |
| Permission denied | `inconclusive` |
| Provider returns insufficient evidence | `inconclusive` |
| Raw response cannot be parsed | `fail` |
| Artifact is schema invalid | `fail` |
| Missing required provenance | `fail` |
| Missing source hash or hash mismatch | `fail` |
| Summary or claim lacks citations | `fail` |
| Citation references unknown source | `fail` |
| Valid cited artifact ingests through US-024 | `pass` |

## Planning Evidence

- Intake: `15`.
- Story: `US-026`, High-Risk, NotebookLM ingest proof required.
- GitHub issue: `#4`.
- Milestone: `HI-OS v0.4.0: Real MCP Adapter Layer`.
- Context pack: `.harness/context/US-026-context.md`.
- Architecture check: passed during story creation.
- Planning trace: `#19`, Detailed `3/3`.
- Provider contract design-review trace: `#20`, Detailed `3/3`.
- Implementation trace: `#21`, Detailed `3/3`, outcome `partial` because live
  `nlm` provider proof was unavailable locally before provider installation.
- Provider-contract correction trace: `#24`, Detailed `3/3`, outcome
  `partial` because authenticated NotebookLM provider proof is still missing.
- Decision 0010 remains the governing file-based boundary.
- Provider contract: accepted for implementation planning.
- Default provider: `notebooklm-mcp-cli`.
- Alternate provider candidate: `PleasePrompto/notebooklm-mcp`.
- Story status: In Progress.
