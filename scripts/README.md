# Scripts

This directory contains harness automation tools.

## Harness CLI

The Rust Harness CLI is the primary interface for the durable layer. Installed
projects use the prebuilt binary at `scripts/bin/harness-cli` on macOS/Linux or
`scripts/bin/harness-cli.exe` on Windows for normal Harness work.

```bash
scripts/bin/harness-cli init          # Create the database
scripts/bin/harness-cli identity      # Show tracked HI-OS product identity
scripts/bin/harness-cli intake ...    # Record a feature intake classification
scripts/bin/harness-cli story ...     # Add or update a story (test matrix row)
scripts/bin/harness-cli story update --id US-001 --unit 1 --integration 1 --e2e 0 --platform 0
scripts/bin/harness-cli arch-check --story US-001
scripts/bin/harness-cli story verify US-001  # Run proof command and governance gate
scripts/bin/harness-cli context ingest --story US-001 --source codegraph --file impact.json
scripts/bin/harness-cli codegraph impact --story US-001 --mode changed-files --changed-files changed-files.txt
scripts/bin/harness-cli friction add ... # Capture structured friction
scripts/bin/harness-cli backlog suggest  # Suggest backlog candidates from friction
scripts/bin/harness-cli rules suggest    # Suggest rule improvements from friction
scripts/bin/harness-cli decision ...  # Add a decision or run its verification
scripts/bin/harness-cli backlog ...   # Add or close a backlog item
scripts/bin/harness-cli trace ...     # Record and auto-score an agent execution trace
scripts/bin/harness-cli score-trace   # Score a trace against TRACE_SPEC.md tiers
scripts/bin/harness-cli query ...     # Query harness data, including backlog --open/--closed
scripts/bin/harness-cli query matrix --numeric  # Show proof flags as 1/0
scripts/bin/harness-cli migrate       # Apply pending schema migrations
scripts/bin/harness-cli --version     # Print the installed CLI version
```

Run `scripts/bin/harness-cli help` or `scripts/bin/harness-cli query help` for
full usage. On Windows, use the same commands through
`.\scripts\bin\harness-cli.exe`.

Proof flags on `story update` are numeric booleans: use `1` for yes and `0` for
no. `arch-check --story <id>` scans `harness-architecture.toml` and stores the
result on the story. `story verify <id>` runs the configured `verify_command`
and then enforces the governance gate. Record proof flags, architecture proof,
context, and the linked trace before the final verify.

Backlog `--risk` uses Harness lanes, not severity words: use `tiny`, `normal`,
or `high-risk`. Use `tiny` instead of `low`. `query matrix` defaults to
human-readable `yes`/`no`; use `query matrix --numeric` when copying values into
`story update`.

The schema lives in `scripts/schema/` and is version-controlled. The database
file (`harness.db`) is `.gitignore`d.

Requires: the prebuilt Rust CLI at `scripts/bin/harness-cli` on macOS/Linux or
`scripts/bin/harness-cli.exe` on Windows.

Direct database inspection may still use SQLite tools, but normal Harness use
should go through the Rust CLI.

### Rust CLI Commands

Current migrated commands:

```bash
scripts/bin/harness-cli init
scripts/bin/harness-cli identity
scripts/bin/harness-cli migrate
scripts/bin/harness-cli import brownfield
scripts/bin/harness-cli intake ...
scripts/bin/harness-cli story add ...
scripts/bin/harness-cli story update ...
scripts/bin/harness-cli story verify ...
scripts/bin/harness-cli context ingest ...
scripts/bin/harness-cli arch-check ...
scripts/bin/harness-cli decision add ...
scripts/bin/harness-cli decision verify ...
scripts/bin/harness-cli backlog add ...
scripts/bin/harness-cli backlog close ...
scripts/bin/harness-cli backlog suggest ...
scripts/bin/harness-cli friction add ...
scripts/bin/harness-cli rules suggest ...
scripts/bin/harness-cli trace ...
scripts/bin/harness-cli score-trace
scripts/bin/harness-cli query matrix
scripts/bin/harness-cli query backlog
scripts/bin/harness-cli query decisions
scripts/bin/harness-cli query intakes
scripts/bin/harness-cli query traces
scripts/bin/harness-cli query friction
scripts/bin/harness-cli query friction-events
scripts/bin/harness-cli query stats
scripts/bin/harness-cli query sql ...
scripts/bin/harness-cli release verify ...
```

`scripts/bin/harness-cli import brownfield` seeds or refreshes the durable database
from existing HI-OS markdown in `docs/TEST_MATRIX.md`,
`docs/decisions/`, and `docs/HARNESS_BACKLOG.md`. This keeps already-installed
Harness repos on the Rust CLI path without losing their populated operating
docs.

## Installer

The upstream installer applies the HI-OS operating files and folder
structure to a target project directory. It defaults to the current directory,
accepts a target path, and asks interactive users whether to `1. Merge`,
`2. Override`, or `3. Stop` when the target already contains `AGENTS.md`,
`docs/`, or `scripts/`.
Non-interactive installs stop on those protected paths unless `--merge` or
`--override` is provided. Use `--merge` as the safe update path for repositories
that already have Harness: it keeps existing files in place and creates only
missing Harness files. Add `--refresh-agent-shim` when an older install has the
full generated Harness guide in `AGENTS.md` and should move to the small stable
shim. Use `--override` only when replacing the protected Harness surface is
intentional.

```bash
curl -fsSL "https://raw.githubusercontent.com/ntu254/Harness-Intelligence-OS/main/scripts/install-harness.sh?$(date +%s)" | bash -s -- --yes
```

```powershell
& ([scriptblock]::Create((irm "https://raw.githubusercontent.com/ntu254/Harness-Intelligence-OS/main/scripts/install-harness.ps1"))) -Yes
```

```bash
curl -fsSL "https://raw.githubusercontent.com/ntu254/Harness-Intelligence-OS/main/scripts/install-harness.sh?$(date +%s)" | bash -s -- --merge --yes
```

```powershell
& ([scriptblock]::Create((irm "https://raw.githubusercontent.com/ntu254/Harness-Intelligence-OS/main/scripts/install-harness.ps1"))) -Merge -Yes
```

```bash
curl -fsSL "https://raw.githubusercontent.com/ntu254/Harness-Intelligence-OS/main/scripts/install-harness.sh?$(date +%s)" | bash -s -- --merge --refresh-agent-shim --yes
```

```powershell
& ([scriptblock]::Create((irm "https://raw.githubusercontent.com/ntu254/Harness-Intelligence-OS/main/scripts/install-harness.ps1"))) -Merge -RefreshAgentShim -Yes
```

`--refresh-agent-shim` backs up `AGENTS.md` before changing it. If the existing
file is recognized as the old generated operating guide, the installer
replaces it with the current shim. Otherwise it appends or replaces only the
marked `<!-- HARNESS:BEGIN -->` block so project-specific instructions remain
in place.

The installer must stay limited to harness files. Do not use it to scaffold
application source folders, package scripts, CI, tests, platform shells, or fake
validation commands. The installer script is not part of the installed project
payload.

By default the installer also downloads the prebuilt Rust Harness CLI for the
current platform into `scripts/bin/harness-cli` on macOS/Linux or
`scripts/bin/harness-cli.exe` on Windows, then verifies its `.sha256` checksum.
A source branch can pin the release used by the installer through
`scripts/harness-cli-release-tag`; HI-OS v0.7 pins `harness-cli-v0.7.0` so
branch installs receive the trusted-distribution CLI. Set
`HARNESS_CLI_RELEASE_TAG` to override that tag, or set
`HARNESS_CLI_BASE_URL` to point at an alternate artifact
directory, such as a local `file:///.../dist` directory created by
`scripts/build-harness-cli-release.sh`.

## Production-Clean Payload

Build the platform-neutral production payload:

```bash
bash scripts/build-production-payload.sh --version 0.7.0
```

```powershell
.\scripts\build-production-payload.ps1 -Version 0.7.0
```

Both entrypoints call the same Python implementation and produce:

```text
dist/hios-production-v0.7.0.zip
dist/hios-production-v0.7.0.zip.sha256
```

Verify the external checksum, internal file hashes, required files, forbidden
paths, and exact source bytes:

```bash
python scripts/verify-production-payload.py --version 0.7.0 --source-check
```

The contract lives in `packaging/production-include.toml`. The payload excludes
Rust source, historical story packets, archived plans, local databases,
`.harness/`, CodeGraph indexes, build output, and `spec.md`. It does not embed a
platform binary; the packaged installer obtains the CLI through the trusted
release asset chain.

## Schema Migrations

Migration files live under `scripts/schema/` and are named `NNN-description.sql`
where `NNN` is a zero-padded version number. Run `scripts/bin/harness-cli migrate` to
apply pending migrations.

## Release Verification

Verify an existing public Harness CLI release through the complete trusted
distribution chain:

```powershell
.\scripts\bin\harness-cli.exe release verify --version 0.7.0
```

The tracked product identity comes from `hios.toml`. The default public origin
and tag prefix come from `harness-release.toml`; `release verify` checks that
the release origin remains aligned with the HI-OS identity origin.
The command checks release metadata, all expected platform assets, the
production payload and SHA256 for v0.7+, the selected binary download, SHA256,
reported version, and a non-mutating smoke command.
It writes a detailed JSON report under `.harness/release/` and a summary row to
SQLite. A trust failure is `fail`; network or GitHub unavailability is
`inconclusive`. Both exit non-zero and neither satisfies story governance.

Stories opt into the blocking evidence requirement explicitly:

```powershell
.\scripts\bin\harness-cli.exe story update --id US-021 --release-proof 1
```

## Governance Report and Dashboard

Generate the governance report and static dashboard:

```powershell
.\scripts\bin\harness-cli.exe governance report --output .harness/reports/governance-report.json
python scripts/verify-governance-report-schema.py .harness/reports/governance-report.json
.\scripts\bin\harness-cli.exe governance dashboard --report .harness/reports/governance-report.json --output .harness/dashboard/index.html
```

## Context Ingest

Validate a file-based MCP artifact without calling the provider:

```powershell
.\scripts\bin\harness-cli.exe context ingest `
  --story US-024 `
  --source codegraph `
  --file .harness/context/codegraph-impact.json
```

Supported sources are `codegraph` and `notebooklm`. The command validates the
US-023 contract and provenance, writes a `context-ingest-result` JSON report,
and stores a summary in SQLite. Only `pass` maps context into the latest linked
intake. `fail` and `inconclusive` remain durable audit evidence and exit
non-zero.

Stories can require passing source evidence explicitly:

```powershell
.\scripts\bin\harness-cli.exe story update --id US-024 --codegraph-ingest 1
.\scripts\bin\harness-cli.exe story update --id US-024 --notebooklm-ingest 1
```

## CodeGraph Adapter

Initialize the local CodeGraph CLI index:

```powershell
codegraph init .
```

Generate and ingest impact evidence from changed files:

```powershell
.\scripts\bin\harness-cli.exe codegraph impact `
  --story US-025 `
  --mode changed-files `
  --changed-files .harness/context/US-025-changed-files.txt `
  --depth 5
```

Or analyze a symbol:

```powershell
.\scripts\bin\harness-cli.exe codegraph impact `
  --story US-025 `
  --mode symbol `
  --symbol validate_context_artifact `
  --depth 2
```

The adapter uses the local `codegraph` executable with no authentication. It
stores raw provider JSON and a normalized US-023 artifact, then invokes the
US-024 ingest boundary. Missing executables and provider command failures are
`inconclusive`; malformed provider JSON is `fail`. Neither satisfies story
governance.

## Future Command Contract

Expected future checks:

```text
validate:quick
  format, lint, typecheck, unit tests, architecture check

test:integration
  backend contract and integration checks

test:e2e
  user-visible end-to-end flows

test:platform
  platform shell smoke checks, if the project has a native shell

test:release
  full suite, log checks, and performance smoke
```

## Release Packaging

Build the current-platform Rust CLI release artifact from the source repo:

```bash
scripts/build-harness-cli-release.sh
```

The script writes `dist/harness-cli-<platform>` plus `.sha256` checksums. The
Windows artifact includes the `.exe` suffix. Supported labels are:

- `macos-arm64`
- `macos-x64`
- `linux-x64`
- `linux-arm64`
- `windows-x64`

For cross-compilation, pass a Cargo target triple:

```bash
scripts/build-harness-cli-release.sh --target x86_64-unknown-linux-gnu
```

GitHub releases are produced by
`.github/workflows/harness-cli-release.yml`. Push a tag matching `v*` or
`harness-cli-v*` to run the verification job, build all supported targets on
native hosted runners, and upload these release assets:

- `harness-cli-macos-arm64`
- `harness-cli-macos-arm64.sha256`
- `harness-cli-macos-x64`
- `harness-cli-macos-x64.sha256`
- `harness-cli-linux-x64`
- `harness-cli-linux-x64.sha256`
- `harness-cli-linux-arm64`
- `harness-cli-linux-arm64.sha256`
- `harness-cli-windows-x64.exe`
- `harness-cli-windows-x64.exe.sha256`
- `hios-production-v0.7.0.zip`
- `hios-production-v0.7.0.zip.sha256`

## MCP Artifact Contract Verification

US-023 defines the versioned file boundary for future MCP adapters. Validate
the three Draft 2020-12 schemas and their semantic fixtures with:

```bash
python -m pip install -r scripts/requirements-contracts.txt
python scripts/verify-mcp-artifact-contracts.py
```

This verifier does not call an MCP provider or ingest data. It checks only the
tracked artifact contracts and representative pass, fail, and inconclusive
states.

## Adoption Documentation Verification

US-039 adds the clean clone adoption walkthrough. Validate the walkthrough and
its index links with:

```bash
python scripts/verify-adoption-docs.py
```

This verifier checks documentation coverage only. It does not create runtime
Harness evidence, call CodeGraph, call NotebookLM, or verify a public release.
