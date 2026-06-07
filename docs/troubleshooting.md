# Troubleshooting

This guide helps diagnose common HI-OS adoption failures without weakening the
governance model.

Core rule:

```text
fail means the proof found a real trust or validation problem.
inconclusive means the proof could not run or could not reach the dependency.
Neither should be rewritten into pass.
```

## Quick Triage

Start with the story matrix:

```bash
scripts/bin/harness-cli query matrix
```

Windows:

```powershell
.\scripts\bin\harness-cli.exe query matrix
```

Then inspect the specific missing evidence:

```bash
scripts/bin/harness-cli context --story US-XXX
scripts/bin/harness-cli arch-check --story US-XXX
scripts/bin/harness-cli story verify US-XXX
```

Runtime evidence usually lives under `.harness/`. The local durable database is
`harness.db`. Both are ignored and should not be committed.

## Installer Fails

Common symptoms:

```text
AGENTS.md, docs/, or scripts/ already exists
checksum mismatch
failed to download harness-cli
unsupported platform
PowerShell execution or TLS error
```

What to check:

1. Existing Harness surface:

   ```bash
   ls AGENTS.md docs scripts
   ```

   Use `--merge --yes` when updating an existing Harness repo:

   ```bash
   curl -fsSL "https://raw.githubusercontent.com/ntu254/Harness-Intelligence-OS/main/scripts/install-harness.sh?$(date +%s)" | bash -s -- --merge --yes
   ```

   Windows:

   ```powershell
   & ([scriptblock]::Create((irm "https://raw.githubusercontent.com/ntu254/Harness-Intelligence-OS/main/scripts/install-harness.ps1"))) -Merge -Yes
   ```

2. Preview changes:

   ```bash
   curl -fsSL "https://raw.githubusercontent.com/ntu254/Harness-Intelligence-OS/main/scripts/install-harness.sh?$(date +%s)" | bash -s -- --dry-run
   ```

   Windows:

   ```powershell
   & ([scriptblock]::Create((irm "https://raw.githubusercontent.com/ntu254/Harness-Intelligence-OS/main/scripts/install-harness.ps1"))) -DryRun
   ```

3. Release pin and assets:

   ```text
   scripts/harness-cli-release-tag
   harness-cli-v0.7.0
   ```

4. If the download path is unavailable, retry later or override the release tag
   only when you intentionally trust that release:

   ```bash
   HARNESS_CLI_RELEASE_TAG=harness-cli-v0.7.0 <install command>
   ```

Do not bypass checksum verification. A checksum mismatch is a trust failure,
not a warning.

## Release Verify Fails

Run:

```bash
scripts/bin/harness-cli release verify --version 0.7.0
```

Windows:

```powershell
.\scripts\bin\harness-cli.exe release verify --version 0.7.0
```

Common symptoms:

```text
release not found
missing asset
checksum mismatch
binary version mismatch
smoke command failed
network unavailable
GitHub unavailable
```

How to interpret:

- `fail`: release metadata, assets, checksum, binary version, or smoke command
  contradicted the expected release contract.
- `inconclusive`: network or GitHub availability prevented verification.
- Neither state satisfies story governance when release proof is required.

What to check:

1. Confirm origin and tag prefix:

   ```text
   harness-release.toml
   origin = "ntu254/Harness-Intelligence-OS"
   tag_prefix = "harness-cli-v"
   ```

2. Confirm a v0.7+ release has 12 assets:

   ```text
   5 platform binaries
   5 binary SHA256 files
   1 production payload ZIP
   1 payload SHA256 file
   ```

3. Inspect the JSON evidence under:

   ```text
   .harness/release/
   ```

4. If the failure is a trust mismatch, fix the release process and publish a
   new version. Do not mutate old release assets.

## CodeGraph Unavailable

Common symptoms:

```text
codegraph executable missing
provider command failed
Context ingest: inconclusive
```

What to check:

```bash
codegraph --help
codegraph init .
scripts/bin/harness-cli codegraph impact --story US-XXX --mode changed-files --changed-files .harness/context/changed-files.txt
```

Windows:

```powershell
codegraph --help
codegraph init .
.\scripts\bin\harness-cli.exe codegraph impact --story US-XXX --mode changed-files --changed-files .harness\context\changed-files.txt
```

Interpretation:

- Missing executable or provider command failure is `inconclusive`.
- Malformed provider output is `fail`.
- Only a passing ingested artifact satisfies `--codegraph-ingest 1`.

Do not manually write CodeGraph output into Harness SQLite. The adapter must
produce a file artifact and pass through `context ingest`.

## NotebookLM Auth Or Session Fails

Common symptoms:

```text
nlm executable missing
Profile 'default' not found
unauthenticated session
notebook not found
network unavailable
Context ingest: inconclusive
```

What to check:

```bash
nlm --version
nlm login --check
nlm notebook list
scripts/bin/harness-cli notebooklm brief --story US-XXX --notebook <notebook-id-or-alias> --query "Find citation-backed context for this story."
```

Windows:

```powershell
nlm --version
nlm login --check
nlm notebook list
.\scripts\bin\harness-cli.exe notebooklm brief --story US-XXX --notebook <notebook-id-or-alias> --query "Find citation-backed context for this story."
```

Correct recovery:

- Authenticate through the provider outside Harness, for example `nlm login`.
- Keep NotebookLM sessions and credentials provider-managed.
- Add relevant sources to the notebook and rerun the adapter.

Guardrail:

```text
Harness must not store Google credentials, cookies, tokens, browser profiles,
or provider session files.
```

## NotebookLM Output Fails Validation

Common symptoms:

```text
summary without cited claims
citation references unknown source
missing provenance
Context ingest: fail
```

What to check:

- Does every grounded claim have citations?
- Do citations point to known sources?
- Does provenance include provider, source, and artifact metadata?
- Does the artifact validate against
  `docs/schemas/notebooklm-brief.schema.json`?

Correct recovery:

```bash
scripts/bin/harness-cli notebooklm brief --story US-XXX --notebook <notebook> --query "Return only citation-backed claims relevant to US-XXX."
```

Uncited summaries are not grounded evidence.

## Governance Gate Fails

Run:

```bash
scripts/bin/harness-cli story verify US-XXX
```

Common missing evidence:

```text
linked intake missing
context pack missing
architecture check missing or failed
mechanical verify command missing or failed
proof flags missing
trace missing
required release proof missing
required CodeGraph proof missing
required NotebookLM proof missing
```

What to check:

```bash
scripts/bin/harness-cli query matrix
scripts/bin/harness-cli query matrix --numeric
scripts/bin/harness-cli context --story US-XXX
scripts/bin/harness-cli arch-check --story US-XXX
scripts/bin/harness-cli query traces
```

Correct recovery:

1. Run the missing proof command.
2. Update proof flags with numeric booleans:

   ```bash
   scripts/bin/harness-cli story update --id US-XXX --unit 1 --integration 1 --e2e 0 --platform 1
   ```

3. Record a trace.
4. Rerun `story verify`.

Do not weaken the story gate to make it pass.

## Governance Report Or Dashboard Fails

Generate report:

```bash
scripts/bin/harness-cli governance report --output .harness/reports/governance-report.json
python scripts/verify-governance-report-schema.py .harness/reports/governance-report.json
```

Generate dashboard:

```bash
scripts/bin/harness-cli governance dashboard --report .harness/reports/governance-report.json --output .harness/dashboard/index.html
```

Windows:

```powershell
.\scripts\bin\harness-cli.exe governance report --output .harness\reports\governance-report.json
python scripts\verify-governance-report-schema.py .harness\reports\governance-report.json
.\scripts\bin\harness-cli.exe governance dashboard --report .harness\reports\governance-report.json --output .harness\dashboard\index.html
```

Common symptoms:

```text
schema validation failed
report file not found
dashboard input report missing
HTML output path cannot be written
unexpected maturity score
```

What to check:

- `harness.db` exists and migrations have run.
- The report JSON validates against
  `docs/schemas/governance-report.schema.json`.
- The dashboard command points at the report that was just generated.
- `.harness/reports/` and `.harness/dashboard/` are writable.
- Runtime `.harness/` output is ignored and should not be committed.

If the dashboard shows a lower maturity score than expected, inspect the report
JSON and story matrix. The dashboard reflects evidence; it should not hide
missing proof.

## When To Record Friction

If the same failure is confusing or repeated, record it:

```bash
scripts/bin/harness-cli friction add --story US-XXX --type unclear_validation --summary "Short description" --evidence "What was hard to diagnose"
scripts/bin/harness-cli backlog suggest
scripts/bin/harness-cli rules suggest
```

Friction records do not change policy automatically. They create reviewable
evidence for later improvements.
