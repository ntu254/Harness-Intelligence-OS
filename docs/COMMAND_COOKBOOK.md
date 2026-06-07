# Command Cookbook

This cookbook groups common HI-OS commands by workflow area. Use it as a quick
reference after reading the README or the full workflow example.

Windows users can replace `scripts/bin/harness-cli` with
`.\scripts\bin\harness-cli.exe`.

## Setup And State

Initialize local durable state:

```bash
scripts/bin/harness-cli init
```

Show tracked HI-OS identity:

```bash
scripts/bin/harness-cli identity
```

Apply pending migrations:

```bash
scripts/bin/harness-cli migrate
```

Import markdown-backed brownfield state:

```bash
scripts/bin/harness-cli import brownfield
```

Inspect current proof matrix:

```bash
scripts/bin/harness-cli query matrix
scripts/bin/harness-cli query matrix --numeric
```

Inspect stats:

```bash
scripts/bin/harness-cli query stats
```

## Intake

Record a normal change request:

```bash
scripts/bin/harness-cli intake \
  --type "change request" \
  --summary "Add onboarding copy" \
  --lane normal \
  --story US-001 \
  --docs "README.md,docs/product/overview.md"
```

Record a harness improvement:

```bash
scripts/bin/harness-cli intake \
  --type "harness improvement" \
  --summary "Improve agent onboarding docs" \
  --lane normal \
  --story US-042 \
  --docs "AGENTS.md,docs/agents"
```

Use automated intake from previously ingested evidence:

```bash
scripts/bin/harness-cli intake \
  --summary "Implement selected story" \
  --story US-001 \
  --auto
```

## Stories

Add a story:

```bash
scripts/bin/harness-cli story add \
  --id US-001 \
  --title "Add onboarding copy" \
  --lane normal \
  --contract README.md \
  --verify "python scripts/verify-adoption-docs.py"
```

Update proof flags:

```bash
scripts/bin/harness-cli story update \
  --id US-001 \
  --status implemented \
  --unit 1 \
  --integration 1 \
  --e2e 0 \
  --platform 1 \
  --evidence "Validation passed with docs verifier and story gate."
```

Require release proof:

```bash
scripts/bin/harness-cli story update --id US-045 --release-proof 1
```

Require provider evidence:

```bash
scripts/bin/harness-cli story update --id US-025 --codegraph-ingest 1
scripts/bin/harness-cli story update --id US-026 --notebooklm-ingest 1
```

## Context

Generate a story context pack:

```bash
scripts/bin/harness-cli context --story US-001
```

Validate and ingest a CodeGraph artifact:

```bash
scripts/bin/harness-cli context ingest \
  --story US-001 \
  --source codegraph \
  --file .harness/context/US-001-codegraph-impact.json
```

Validate and ingest a NotebookLM artifact:

```bash
scripts/bin/harness-cli context ingest \
  --story US-001 \
  --source notebooklm \
  --file .harness/context/US-001-notebooklm-brief.json
```

## Verify

Run architecture check:

```bash
scripts/bin/harness-cli arch-check --story US-001
```

Run the story mechanical proof and governance gate:

```bash
scripts/bin/harness-cli story verify US-001
```

Validate adoption docs:

```bash
python scripts/verify-adoption-docs.py
```

Validate MCP artifact contracts:

```bash
python scripts/verify-mcp-artifact-contracts.py
```

Validate governance report schema:

```bash
python scripts/verify-governance-report-schema.py .harness/reports/governance-report.json
```

Run Rust validation:

```bash
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

## Trace

Record a trace:

```bash
scripts/bin/harness-cli trace \
  --summary "Completed US-001 onboarding copy" \
  --intake 1 \
  --story US-001 \
  --agent Codex \
  --outcome completed \
  --duration 20 \
  --tokens 0 \
  --friction "none" \
  --actions "read context; updated docs; ran validation" \
  --read "README.md; .harness/context/US-001-context.md" \
  --changed "README.md" \
  --decisions "kept change docs-only" \
  --errors "none" \
  --notes "Example trace."
```

Score the latest trace:

```bash
scripts/bin/harness-cli score-trace
```

Score a specific trace:

```bash
scripts/bin/harness-cli score-trace --id 42
```

Query traces and friction:

```bash
scripts/bin/harness-cli query traces
scripts/bin/harness-cli query friction
```

## Release

Build and verify the production-clean distribution payload:

```bash
bash scripts/build-production-payload.sh --version 0.7.0
python scripts/verify-production-payload.py --version 0.7.0 --source-check
```

On Windows PowerShell:

```powershell
.\scripts\build-production-payload.ps1 -Version 0.7.0
python scripts/verify-production-payload.py --version 0.7.0 --source-check
```

The output is `dist/hios-production-v0.7.0.zip` plus its `.sha256` asset.
Publishing those files remains part of release hardening.

Verify a public CLI release:

```bash
scripts/bin/harness-cli release verify --version 0.7.0
```

Verify release proof for a story:

```bash
scripts/bin/harness-cli release verify --version 0.7.0 --story US-045
```

Override origin only when intentionally verifying a fork:

```bash
scripts/bin/harness-cli release verify --version 0.7.0 --origin ntu254/Harness-Intelligence-OS
```

Network or GitHub availability failures are `inconclusive`, not `pass`.
Checksum, asset, version, or smoke mismatches are `fail`.

## Dashboard

Generate governance report:

```bash
scripts/bin/harness-cli governance report \
  --output .harness/reports/governance-report.json
```

Verify report schema:

```bash
python scripts/verify-governance-report-schema.py .harness/reports/governance-report.json
```

Export static dashboard:

```bash
scripts/bin/harness-cli governance dashboard \
  --report .harness/reports/governance-report.json \
  --output .harness/dashboard/index.html
```

Open `.harness/dashboard/index.html` locally.

## MCP / Provider Evidence

Generate CodeGraph impact from changed files:

```bash
mkdir -p .harness/context
git diff --name-only > .harness/context/US-001-changed-files.txt
scripts/bin/harness-cli codegraph impact \
  --story US-001 \
  --mode changed-files \
  --changed-files .harness/context/US-001-changed-files.txt
```

Generate CodeGraph impact for a symbol:

```bash
scripts/bin/harness-cli codegraph impact \
  --story US-001 \
  --mode symbol \
  --symbol validate_context_artifact \
  --depth 2
```

Generate a NotebookLM grounded brief:

```bash
scripts/bin/harness-cli notebooklm brief \
  --story US-001 \
  --notebook <notebook-id-or-alias> \
  --query "Find citation-backed context relevant to US-001."
```

Provider rules:

- Missing provider executable, network, auth, notebook, or source:
  `inconclusive`.
- Malformed output, missing provenance, or uncited claims: `fail`.
- Only passing ingested artifacts satisfy explicit provider proof.
- Harness must not store Google credentials, cookies, tokens, browser profiles,
  or provider session files.

## Friction And Learning Loop

Capture a structured friction event:

```bash
scripts/bin/harness-cli friction add \
  --story US-001 \
  --type unclear_validation \
  --summary "Validation command was unclear" \
  --evidence "The story packet did not name the proof command."
```

Review backlog suggestions:

```bash
scripts/bin/harness-cli backlog suggest
```

Review rule suggestions:

```bash
scripts/bin/harness-cli rules suggest
```

Create a backlog item explicitly:

```bash
scripts/bin/harness-cli backlog add \
  --title "Clarify validation command" \
  --pain "Agents could not identify proof command" \
  --risk normal
```
