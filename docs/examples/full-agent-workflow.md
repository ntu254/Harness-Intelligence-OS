# Full Agent Workflow Example

This example shows one complete HI-OS loop from a human request to a governance
dashboard. It is written as a reusable pattern for agents and maintainers.

The example story is intentionally docs-only so it can be understood without a
chosen app stack:

```text
Human request:
  "Add a short onboarding note that tells new contributors where to start."

Story:
  US-EXAMPLE: Add contributor onboarding note
```

In a real application, replace the docs-only verification command with the
project's actual test command, such as `npm test`, `pytest`, `cargo test`, or
an E2E suite.

## 1. Agent Reads The Operating Context

Before changing files, the agent reads the stable Harness entrypoints:

```text
AGENTS.md
README.md
docs/HARNESS.md
docs/FEATURE_INTAKE.md
docs/ARCHITECTURE.md
docs/CONTEXT_RULES.md
scripts/bin/harness-cli query matrix
```

Expected conclusion:

```text
Input type: harness improvement
Lane: tiny or normal
Reason: docs-only onboarding change, no auth/data/provider/release behavior
```

Use `normal` when you want a visible story packet and governance proof, as this
example does.

## 2. Record Intake

```bash
scripts/bin/harness-cli intake \
  --type "harness improvement" \
  --summary "Add contributor onboarding note" \
  --lane normal \
  --story US-EXAMPLE \
  --docs "README.md,docs/examples/full-agent-workflow.md" \
  --notes "Example workflow story; replace verification with project tests in real apps."
```

Windows PowerShell:

```powershell
.\scripts\bin\harness-cli.exe intake `
  --type "harness improvement" `
  --summary "Add contributor onboarding note" `
  --lane normal `
  --story US-EXAMPLE `
  --docs "README.md,docs/examples/full-agent-workflow.md" `
  --notes "Example workflow story; replace verification with project tests in real apps."
```

Expected output:

```text
Intake #<id> recorded.
```

## 3. Add A Story Row

For a docs-only example in this repo:

```bash
scripts/bin/harness-cli story add \
  --id US-EXAMPLE \
  --title "Add contributor onboarding note" \
  --lane normal \
  --contract README.md \
  --verify "python scripts/verify-adoption-docs.py" \
  --notes "Example full agent workflow story."
```

Windows PowerShell:

```powershell
.\scripts\bin\harness-cli.exe story add `
  --id US-EXAMPLE `
  --title "Add contributor onboarding note" `
  --lane normal `
  --contract README.md `
  --verify "python scripts/verify-adoption-docs.py" `
  --notes "Example full agent workflow story."
```

Expected output:

```text
Story US-EXAMPLE added.
```

In an application repo, use the real proof command:

```text
--verify "npm test"
--verify "pytest"
--verify "cargo test --workspace"
```

## 4. Optional Provider Context

Provider evidence is useful, but it is not allowed to write straight into
Harness SQLite. Providers produce file artifacts; Harness validates and ingests
them.

CodeGraph changed-files example:

```bash
mkdir -p .harness/context
git diff --name-only > .harness/context/US-EXAMPLE-changed-files.txt
scripts/bin/harness-cli codegraph impact \
  --story US-EXAMPLE \
  --mode changed-files \
  --changed-files .harness/context/US-EXAMPLE-changed-files.txt
```

NotebookLM grounded brief example:

```bash
scripts/bin/harness-cli notebooklm brief \
  --story US-EXAMPLE \
  --notebook <notebook-id-or-alias> \
  --query "Find citation-backed onboarding and governance rules relevant to US-EXAMPLE."
```

Expected passing provider result:

```text
Context ingest: pass
```

If CodeGraph or NotebookLM is missing, unauthenticated, offline, or has no
usable notebook/source set, the correct result is:

```text
Context ingest: inconclusive
```

Do not convert `inconclusive` provider evidence into `pass`. Continue with
manual context, or install/configure the provider and rerun the adapter.

## 5. Generate The Context Pack

```bash
scripts/bin/harness-cli context --story US-EXAMPLE
```

Windows PowerShell:

```powershell
.\scripts\bin\harness-cli.exe context --story US-EXAMPLE
```

Expected output:

```text
Context pack generated successfully at <repo>/.harness/context/US-EXAMPLE-context.md
```

The agent reads the context pack before editing. If provider evidence was
inconclusive, the context pack should make that visible rather than pretending
the provider passed.

## 6. Implement The Smallest Change

For this example, the product delta might be a short README or docs note.

Example implementation rule:

```text
Change only the onboarding text needed for US-EXAMPLE.
Do not refactor unrelated docs.
Do not change release tags, installer pins, schemas, or provider contracts.
```

For app code, the agent should make the smallest vertical slice that satisfies
the story and its validation proof.

## 7. Run Validation

For this docs-only example:

```bash
cargo fmt --check
cargo test --workspace
python scripts/verify-adoption-docs.py
scripts/bin/harness-cli arch-check --story US-EXAMPLE
```

Windows PowerShell:

```powershell
cargo fmt --check
cargo test --workspace
python scripts\verify-adoption-docs.py
.\scripts\bin\harness-cli.exe arch-check --story US-EXAMPLE
```

Expected output snippets:

```text
49 passed
Adoption docs verification passed.
Architecture check passed.
```

For an app story, add the app's unit, integration, E2E, or platform commands.
Do not mark a proof column as passing until the matching command has run.

## 8. Record Proof Flags

```bash
scripts/bin/harness-cli story update \
  --id US-EXAMPLE \
  --status implemented \
  --unit 1 \
  --integration 1 \
  --e2e 0 \
  --platform 1 \
  --evidence "Docs update verified with adoption verifier, tests, context, and architecture check."
```

Windows PowerShell:

```powershell
.\scripts\bin\harness-cli.exe story update `
  --id US-EXAMPLE `
  --status implemented `
  --unit 1 `
  --integration 1 `
  --e2e 0 `
  --platform 1 `
  --evidence "Docs update verified with adoption verifier, tests, context, and architecture check."
```

Expected output:

```text
Story US-EXAMPLE updated.
```

Use `1` and `0`, not `yes` and `no`.

## 9. Record The Trace

```bash
scripts/bin/harness-cli trace \
  --summary "Completed US-EXAMPLE contributor onboarding note" \
  --story US-EXAMPLE \
  --agent Codex \
  --outcome completed \
  --duration 20 \
  --tokens 0 \
  --friction "none" \
  --actions "read Harness docs; generated context; updated onboarding note; ran validation" \
  --read "AGENTS.md; README.md; docs/HARNESS.md; docs/FEATURE_INTAKE.md; .harness/context/US-EXAMPLE-context.md" \
  --changed "README.md" \
  --decisions "kept the change docs-only; did not change release or provider behavior" \
  --errors "none" \
  --notes "Example trace shape; adapt duration and validation details in real work."
```

Expected output:

```text
Trace #<id> recorded.
Tier achieved: detailed (3/3)
MEETS REQUIREMENT
```

## 10. Run The Story Gate

```bash
scripts/bin/harness-cli story verify US-EXAMPLE
```

Windows PowerShell:

```powershell
.\scripts\bin\harness-cli.exe story verify US-EXAMPLE
```

Expected output:

```text
Story US-EXAMPLE verification: pass
Story US-EXAMPLE governance gate: pass
```

If the gate fails, read the missing evidence line instead of forcing the story
to done. Common missing evidence:

- context pack was not generated;
- architecture check was not run;
- proof flags were not recorded;
- trace was not linked;
- provider evidence was required but only inconclusive;
- mechanical verify command failed.

## 11. Export The Governance Dashboard

```bash
scripts/bin/harness-cli governance report \
  --output .harness/reports/US-EXAMPLE-governance-report.json

scripts/bin/harness-cli governance dashboard \
  --report .harness/reports/US-EXAMPLE-governance-report.json \
  --output .harness/dashboard/US-EXAMPLE-index.html
```

Windows PowerShell:

```powershell
.\scripts\bin\harness-cli.exe governance report `
  --output .harness\reports\US-EXAMPLE-governance-report.json

.\scripts\bin\harness-cli.exe governance dashboard `
  --report .harness\reports\US-EXAMPLE-governance-report.json `
  --output .harness\dashboard\US-EXAMPLE-index.html
```

Expected output:

```text
Governance report: .harness/reports/US-EXAMPLE-governance-report.json
Governance dashboard: .harness/dashboard/US-EXAMPLE-index.html
```

Open the HTML file locally. It should show the story, gate status, proof
summary, trace evidence, and maturity summary.

## Provider Troubleshooting

### CodeGraph Unavailable

Symptoms:

```text
provider executable missing
Context ingest: inconclusive
```

What to do:

- Install or configure the CodeGraph CLI.
- Rerun `harness-cli codegraph impact`.
- Keep the story gate honest: inconclusive CodeGraph evidence does not satisfy
  a required CodeGraph proof.

### NotebookLM Auth Or Session Missing

Symptoms:

```text
Profile 'default' not found
unauthenticated session
Context ingest: inconclusive
```

What to do:

- Authenticate through the provider outside Harness, for example `nlm login`.
- Check provider auth with `nlm login --check`.
- Rerun `harness-cli notebooklm brief`.
- Do not store Google credentials, cookies, tokens, browser profiles, or
  provider session files in Harness.

### NotebookLM Output Has No Citations

Symptoms:

```text
summary without cited claims
Context ingest: fail
```

What to do:

- Ask for citation-backed claims.
- Ensure the notebook has relevant sources.
- Rerun the adapter and require citations for every grounded claim.

## What The Agent Should Say At Handoff

Good handoff:

```text
US-EXAMPLE is implemented.
Context pack, architecture check, validation command, trace, story gate, and
governance dashboard all passed.
Provider evidence was not required; missing providers were not treated as pass.
Runtime artifacts stayed under .harness/ and harness.db.
```

Bad handoff:

```text
Looks done.
```

HI-OS works because the handoff carries proof, not vibes.

