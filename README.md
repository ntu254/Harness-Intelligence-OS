# Harness Intelligence OS

An agent-ready operating layer for software repositories.

HI-OS helps humans and coding agents move from intent to verified work without
letting the agent jump straight into edits. It gives a repo a durable workflow:

```text
intake
  -> context
  -> validation proof
  -> trace
  -> governance dashboard
```

The app is what users touch. HI-OS is what agents touch.

## What HI-OS Does

Coding agents are fast, but most repositories do not tell them enough before
they start changing code. Important product rules live in chat history,
validation expectations are vague, and release trust is often checked after the
fact.

HI-OS makes the repository answer practical questions first:

- What is the requested work?
- Which product or architecture rules apply?
- How risky is the change?
- What context should the agent read?
- What proof will show the work is done?
- Which release or dashboard evidence can a human audit later?

The result is a repository workflow where agents operate through intake,
story packets, context packs, architecture checks, validation gates, traces,
release verification, and governance reports instead of free-form handoff.

## 5-Minute Quickstart

Use this when you want to install HI-OS into another project.

### 1. Install

From the target project directory:

```bash
curl -fsSL "https://raw.githubusercontent.com/ntu254/Harness-Intelligence-OS/main/scripts/install-harness.sh?$(date +%s)" | bash -s -- --yes
```

On Windows PowerShell:

```powershell
& ([scriptblock]::Create((irm "https://raw.githubusercontent.com/ntu254/Harness-Intelligence-OS/main/scripts/install-harness.ps1"))) -Yes
```

If the project already has `AGENTS.md`, `docs/`, or `scripts/`, use merge mode:

```bash
curl -fsSL "https://raw.githubusercontent.com/ntu254/Harness-Intelligence-OS/main/scripts/install-harness.sh?$(date +%s)" | bash -s -- --merge --yes
```

```powershell
& ([scriptblock]::Create((irm "https://raw.githubusercontent.com/ntu254/Harness-Intelligence-OS/main/scripts/install-harness.ps1"))) -Merge -Yes
```

Claude Code users should add `--claude` so `CLAUDE.md` imports the Harness
instructions:

```bash
curl -fsSL "https://raw.githubusercontent.com/ntu254/Harness-Intelligence-OS/main/scripts/install-harness.sh?$(date +%s)" | bash -s -- --claude --yes
```

The installer copies the Harness docs and downloads the trusted prebuilt CLI to
`scripts/bin/harness-cli` on macOS/Linux or `scripts/bin/harness-cli.exe` on
Windows.

### 2. Initialize The Local Harness Database

```bash
scripts/bin/harness-cli init
scripts/bin/harness-cli query matrix
```

On Windows:

```powershell
.\scripts\bin\harness-cli.exe init
.\scripts\bin\harness-cli.exe query matrix
```

`harness.db` and `.harness/` are local runtime evidence. They are ignored and
should not be committed.

### 3. Create The First Work Item

```bash
scripts/bin/harness-cli intake \
  --type "change request" \
  --summary "Describe the requested change" \
  --lane normal \
  --story US-001 \
  --docs "docs/product/overview.md"

scripts/bin/harness-cli story add \
  --id US-001 \
  --title "First verified change" \
  --lane normal \
  --contract docs/product/overview.md \
  --verify "your-test-command"
```

On Windows, use `.\scripts\bin\harness-cli.exe` with the same arguments.

### 4. Generate Context, Validate, Trace

```bash
scripts/bin/harness-cli context --story US-001
scripts/bin/harness-cli arch-check --story US-001
scripts/bin/harness-cli story update --id US-001 --unit 1 --integration 1 --e2e 0 --platform 0
scripts/bin/harness-cli trace --summary "Completed US-001" --story US-001 --outcome completed
scripts/bin/harness-cli story verify US-001
```

`story verify` runs the configured proof command and then enforces the
governance gate. The story is not done until both pass.

### 5. Export Governance Evidence

```bash
scripts/bin/harness-cli governance report --output .harness/reports/governance-report.json
scripts/bin/harness-cli governance dashboard --report .harness/reports/governance-report.json --output .harness/dashboard/index.html
```

Open `.harness/dashboard/index.html` locally to inspect story proof,
governance gate status, release evidence, friction, and maturity summary.

Starting from a fresh clone of this repository? Use the full walkthrough:

- `docs/adoption/clean-clone-walkthrough.md`

## Core Workflow

HI-OS is deliberately boring in the best way: every serious task follows the
same path.

```text
Human request
  -> intake classification
  -> story packet
  -> context pack
  -> implementation
  -> validation proof
  -> trace
  -> story governance gate
  -> governance dashboard
```

Key commands:

```bash
scripts/bin/harness-cli intake --summary "<work>" --lane normal --story US-001
scripts/bin/harness-cli context --story US-001
scripts/bin/harness-cli arch-check --story US-001
scripts/bin/harness-cli story verify US-001
scripts/bin/harness-cli trace --summary "<what happened>" --story US-001 --outcome completed
scripts/bin/harness-cli governance report --output .harness/reports/governance-report.json
scripts/bin/harness-cli governance dashboard --report .harness/reports/governance-report.json --output .harness/dashboard/index.html
```

Agents should read `AGENTS.md`, `docs/HARNESS.md`,
`docs/FEATURE_INTAKE.md`, `docs/ARCHITECTURE.md`, and
`docs/CONTEXT_RULES.md` before work. The local `harness-cli query matrix`
shows what proof exists and what is still missing.

## Trusted Distribution

HI-OS publishes prebuilt Harness CLI binaries with SHA256 assets for:

- macOS arm64;
- macOS x64;
- Linux x64;
- Linux arm64;
- Windows x64.

Verify the public release chain:

```bash
scripts/bin/harness-cli release verify --version 0.6.0
```

On Windows:

```powershell
.\scripts\bin\harness-cli.exe release verify --version 0.6.0
```

`release verify` checks release metadata, 5 platform binaries, 5 SHA256 files,
download, checksum, binary version, and a smoke command. Network or GitHub
availability failures are `inconclusive`, not `pass`.

The default public origin is `ntu254/Harness-Intelligence-OS`, configured in
`harness-release.toml`.

## Governance Dashboard

The governance report and dashboard make local proof inspectable:

- story status and proof columns;
- governance gate pass/fail state;
- release verification summary;
- friction events;
- maturity score;
- static HTML dashboard output.

```bash
scripts/bin/harness-cli governance report --output .harness/reports/governance-report.json
scripts/bin/harness-cli governance dashboard --report .harness/reports/governance-report.json --output .harness/dashboard/index.html
```

The dashboard is static HTML with no external assets.

## CodeGraph And NotebookLM

HI-OS v0.4+ supports file-based provider boundaries for external intelligence.
Providers do not write directly into Harness SQLite.

CodeGraph impact evidence:

```bash
scripts/bin/harness-cli codegraph impact --story US-001 --mode changed-files --changed-files .harness/context/changed-files.txt
```

NotebookLM grounded brief evidence:

```bash
scripts/bin/harness-cli notebooklm brief --story US-001 --notebook <notebook-id-or-alias> --query "Find citation-backed context for this story."
```

Rules:

- Provider unavailable: `inconclusive`, not `pass`.
- Missing citations or malformed provider output: `fail`.
- Passing artifacts go through `context ingest`.
- Harness never stores Google credentials, cookies, tokens, browser profiles,
  or provider session files.

## Repository Map

```text
AGENTS.md                         stable agent entrypoint
docs/HARNESS.md                   human-agent operating model
docs/FEATURE_INTAKE.md            risk lanes and intake rules
docs/ARCHITECTURE.md              architecture boundary rules
docs/CONTEXT_RULES.md             what agents should read and when
docs/agents/                      Cursor, Claude Code, and Codex packs
docs/adoption/                    first-run adoption walkthroughs
docs/examples/                    end-to-end example workflows
docs/demo/                        example transformation from idea to story
docs/stories/                     story packets and validation evidence
docs/decisions/                   durable decisions
docs/schemas/                     artifact and report schemas
scripts/bin/harness-cli           installed CLI path in target projects
scripts/README.md                 command and installer details
```

For source-clone setup of this repo, start here:

- `docs/adoption/clean-clone-walkthrough.md`

For a complete agent workflow example, read:

- `docs/examples/full-agent-workflow.md`

For agent-specific operating notes, read:

- `docs/agents/codex.md`
- `docs/agents/claude-code.md`
- `docs/agents/cursor.md`

For command details, use:

- `scripts/README.md`
- `scripts/bin/harness-cli help`

## Current Milestones

Completed foundation:

- v0.1: Context-grounded auto intake.
- v0.2: Blocking governance gate.
- v0.3: Trusted distribution and evidence trail.
- v0.4: MCP artifact contracts and provider adapters.
- v0.5: Structured friction learning loop.
- v0.6: Governance report, maturity summary, and static dashboard.

Current adoption work:

- v0.7: Adoption Ready.

The v0.7 focus is not new feature surface. It is making HI-OS easy to install,
understand, use, debug, and trust.

## Contributing

Useful contributions include:

- testing the clean clone walkthrough on a new machine;
- improving docs for first-time users;
- adding real project examples;
- reporting agent failure cases caused by missing repo context;
- improving validation patterns for different stacks;
- comparing behavior across Codex, Claude Code, Cursor, and other agents.

See `CONTRIBUTING.md` for contribution notes.

## Short Description

HI-OS is an agent-ready repository operating layer for Codex, Claude Code,
Cursor, and other coding agents: intake, context packs, story gates, release
verification, governance dashboards, and durable traces.
