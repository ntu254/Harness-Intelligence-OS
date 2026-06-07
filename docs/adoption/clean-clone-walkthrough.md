# Clean Clone Walkthrough

> Production users do not need the full development clone. HI-OS release
> hardening publishes a production-clean ZIP built from
> `packaging/production-include.toml`. This walkthrough remains the maintainer
> path for validating a source clone.

## Production Payload Path

Maintainers can build and verify the same platform-neutral payload that release
hardening publishes:

```bash
bash scripts/build-production-payload.sh --version 0.7.0
python scripts/verify-production-payload.py --version 0.7.0 --source-check
```

On Windows PowerShell:

```powershell
.\scripts\build-production-payload.ps1 -Version 0.7.0
python scripts/verify-production-payload.py --version 0.7.0 --source-check
```

Extract `dist/hios-production-v0.7.0.zip`, enter its single root directory,
then run the packaged installer against an empty project. The installer copies
the operating contract and downloads the platform CLI from the trusted release
asset chain. The ZIP deliberately excludes Rust source, story history, local
databases, `.harness/`, `target/`, and `spec.md`.

This walkthrough proves the first useful Harness loop from a clean clone:

```text
clone
  -> build or install the Harness CLI
  -> initialize local durable state
  -> create a local demo intake and story
  -> generate context
  -> run architecture and story governance gates
  -> export governance report and dashboard
  -> optionally verify the public release
```

It is intentionally local. Runtime files such as `harness.db`, `.harness/`,
`target/`, and `dist/` are ignored and should not be committed.

## Prerequisites

- Git.
- Rust toolchain with Cargo.
- Python 3 for repository verifiers.
- PowerShell on Windows or a POSIX shell on macOS/Linux.
- Network access only for `release verify` or installer tests.
- Optional provider CLIs:
  - CodeGraph CLI for `harness-cli codegraph impact`.
  - `notebooklm-mcp-cli` / `nlm` for `harness-cli notebooklm brief`.

CodeGraph and NotebookLM are not required for this walkthrough. If those
providers are missing or unavailable, Harness records `inconclusive`, not
`pass`, for provider-dependent evidence.

## 1. Clone

```bash
git clone https://github.com/ntu254/Harness-Intelligence-OS.git
cd Harness-Intelligence-OS
```

## 2. Build The CLI From Source

A clean source clone does not contain `scripts/bin/harness-cli` because
installed binaries are ignored. Build the local development CLI:

```bash
cargo build --package harness-cli --release
./target/release/harness-cli --version
```

On Windows PowerShell:

```powershell
cargo build --package harness-cli --release
.\target\release\harness-cli.exe --version
```

Expected version for the v0.7 public release line:

```text
harness-cli 0.7.0
```

## 3. Initialize Local Durable State

`harness.db` is local and ignored, so a clean clone starts without durable
records. Initialize it, then import any markdown-backed brownfield state:

```bash
./target/release/harness-cli init
./target/release/harness-cli import brownfield
./target/release/harness-cli query matrix
```

On Windows PowerShell:

```powershell
.\target\release\harness-cli.exe init
.\target\release\harness-cli.exe import brownfield
.\target\release\harness-cli.exe query matrix
```

It is valid for the matrix to be empty after import. Historical runtime rows
from another maintainer's `harness.db` are not committed to the repo.

## 4. Create A Local Walkthrough Story

Create a throwaway local story so the governance gate has something real to
check. This updates only ignored runtime state unless you edit tracked docs.

```bash
./target/release/harness-cli intake \
  --type "harness improvement" \
  --summary "Try the clean clone walkthrough" \
  --lane tiny \
  --story US-DEMO \
  --docs "README.md,docs/HARNESS.md" \
  --notes "Local walkthrough smoke row; do not commit runtime database."

./target/release/harness-cli story add \
  --id US-DEMO \
  --title "Try the clean clone walkthrough" \
  --lane tiny \
  --contract README.md \
  --verify "./target/release/harness-cli --version" \
  --notes "Local clean clone smoke story."
```

On Windows PowerShell:

```powershell
.\target\release\harness-cli.exe intake `
  --type "harness improvement" `
  --summary "Try the clean clone walkthrough" `
  --lane tiny `
  --story US-DEMO `
  --docs "README.md,docs/HARNESS.md" `
  --notes "Local walkthrough smoke row; do not commit runtime database."

.\target\release\harness-cli.exe story add `
  --id US-DEMO `
  --title "Try the clean clone walkthrough" `
  --lane tiny `
  --contract README.md `
  --verify ".\target\release\harness-cli.exe --version" `
  --notes "Local clean clone smoke story."
```

## 5. Generate Context And Architecture Evidence

```bash
./target/release/harness-cli context --story US-DEMO
./target/release/harness-cli arch-check --story US-DEMO
```

On Windows PowerShell:

```powershell
.\target\release\harness-cli.exe context --story US-DEMO
.\target\release\harness-cli.exe arch-check --story US-DEMO
```

The context pack is written under `.harness/context/`.

## 6. Record Proof And Trace

Mark the local story as implemented only after the context and architecture
commands pass:

```bash
./target/release/harness-cli story update \
  --id US-DEMO \
  --status implemented \
  --unit 1 \
  --integration 1 \
  --e2e 0 \
  --platform 1 \
  --evidence "Clean clone walkthrough smoke pass."

./target/release/harness-cli trace \
  --summary "Clean clone walkthrough smoke" \
  --story US-DEMO \
  --agent "Human or agent name" \
  --outcome completed \
  --duration 5 \
  --tokens 0 \
  --friction "none" \
  --actions "initialized local db; imported markdown state; added demo intake and story; generated context; ran architecture check" \
  --read "README.md; docs/HARNESS.md" \
  --changed ".harness/context/US-DEMO-context.md; harness.db" \
  --decisions "runtime smoke story stays local" \
  --errors "none" \
  --notes "clean clone smoke uses ignored runtime artifacts"
```

On Windows PowerShell:

```powershell
.\target\release\harness-cli.exe story update `
  --id US-DEMO `
  --status implemented `
  --unit 1 `
  --integration 1 `
  --e2e 0 `
  --platform 1 `
  --evidence "Clean clone walkthrough smoke pass."

.\target\release\harness-cli.exe trace `
  --summary "Clean clone walkthrough smoke" `
  --story US-DEMO `
  --agent "Human or agent name" `
  --outcome completed `
  --duration 5 `
  --tokens 0 `
  --friction "none" `
  --actions "initialized local db; imported markdown state; added demo intake and story; generated context; ran architecture check" `
  --read "README.md; docs/HARNESS.md" `
  --changed ".harness/context/US-DEMO-context.md; harness.db" `
  --decisions "runtime smoke story stays local" `
  --errors "none" `
  --notes "clean clone smoke uses ignored runtime artifacts"
```

## 7. Run The Story Gate

```bash
./target/release/harness-cli story verify US-DEMO
```

On Windows PowerShell:

```powershell
.\target\release\harness-cli.exe story verify US-DEMO
```

Expected result:

```text
Story US-DEMO verification: pass
Story US-DEMO governance gate: pass
```

## 8. Export Governance Evidence

```bash
./target/release/harness-cli governance report \
  --output .harness/reports/US-DEMO-governance-report.json

./target/release/harness-cli governance dashboard \
  --report .harness/reports/US-DEMO-governance-report.json \
  --output .harness/dashboard/US-DEMO-index.html
```

On Windows PowerShell:

```powershell
.\target\release\harness-cli.exe governance report `
  --output .harness\reports\US-DEMO-governance-report.json

.\target\release\harness-cli.exe governance dashboard `
  --report .harness\reports\US-DEMO-governance-report.json `
  --output .harness\dashboard\US-DEMO-index.html
```

Open the HTML file locally to inspect the static dashboard.

## 9. Verify The Public Release

This step uses the network and GitHub release assets. It proves trusted
distribution for the public CLI release:

```bash
./target/release/harness-cli release verify --version 0.7.0
```

On Windows PowerShell:

```powershell
.\target\release\harness-cli.exe release verify --version 0.7.0
```

Expected checks:

- release metadata found;
- 5 platform binaries, 5 binary SHA256 files, the production ZIP, and its
  SHA256 discovered;
- production payload SHA256 verified;
- selected platform binary downloaded;
- SHA256 verified;
- binary reports the requested version;
- smoke command passes.

Network or GitHub availability problems are `inconclusive`, not `pass`.

## Optional Provider Evidence

After the local walkthrough passes, projects can add provider-backed context:

```bash
./target/release/harness-cli codegraph impact --story US-DEMO --mode changed-files --changed-files .harness/context/changed-files.txt
./target/release/harness-cli notebooklm brief --story US-DEMO --notebook <notebook-id-or-alias> --query "Find citation-backed context for this story."
```

On Windows PowerShell:

```powershell
.\target\release\harness-cli.exe codegraph impact --story US-DEMO --mode changed-files --changed-files .harness\context\changed-files.txt
.\target\release\harness-cli.exe notebooklm brief --story US-DEMO --notebook <notebook-id-or-alias> --query "Find citation-backed context for this story."
```

Provider rules:

- Missing provider executable: `inconclusive`.
- Unauthenticated or unavailable NotebookLM session: `inconclusive`.
- Missing citations or malformed provider output: `fail`.
- Only passing ingested artifacts satisfy explicit story governance proof.
- Harness must not store Google credentials, cookies, tokens, browser profiles,
  or provider session files.

## What To Commit

Commit only intentional source changes:

- docs;
- source code;
- schema or verifier scripts;
- release notes when a release story needs them.

Do not commit:

- `harness.db`, `harness.db-wal`, or `harness.db-shm`;
- `.harness/`;
- `.codegraph/`;
- `target/`;
- `dist/`;
- downloaded `scripts/bin/harness-cli` binaries.
