<div align="center">

# Harness Intelligence OS

<p align="center">
  <a href="./README.md"><strong>English</strong></a>
  ·
  <a href="./README.vi.md">Tiếng Việt</a>
</p>

### Agent-ready operating layer for software repositories

**HI-OS helps humans and AI coding agents move from intent to verified work without letting agents jump straight into edits.**

<br />

<p>
  <strong>intake</strong>
  &nbsp;→&nbsp;
  <strong>context pack</strong>
  &nbsp;→&nbsp;
  <strong>validation proof</strong>
  &nbsp;→&nbsp;
  <strong>trace</strong>
  &nbsp;→&nbsp;
  <strong>governance dashboard</strong>
</p>

<br />

<table>
  <tr>
    <td><strong>Product</strong></td>
    <td>Harness Intelligence OS</td>
  </tr>
  <tr>
    <td><strong>Short name</strong></td>
    <td>HI-OS</td>
  </tr>
  <tr>
    <td><strong>Canonical origin</strong></td>
    <td><code>ntu254/Harness-Intelligence-OS</code></td>
  </tr>
  <tr>
    <td><strong>Current line</strong></td>
    <td><code>v0.7 Adoption Ready</code></td>
  </tr>
</table>

<br />

<strong>Installable. Understandable. Debuggable. Production-clean. Publicly verifiable.</strong>

</div>

---

## Table Of Contents

- [What HI-OS Is](#what-hi-os-is)
- [What HI-OS Gives You](#what-hi-os-gives-you)
- [Sovereign Identity](#sovereign-identity)
- [5-Minute Quickstart](#5-minute-quickstart)
- [First Run](#first-run)
- [Create Your First Verified Story](#create-your-first-verified-story)
- [Core Workflow](#core-workflow)
- [Governance Dashboard](#governance-dashboard)
- [Trusted Distribution](#trusted-distribution)
- [Production-Clean Payload](#production-clean-payload)
- [CodeGraph And NotebookLM Evidence](#codegraph-and-notebooklm-evidence)
- [Repository Map](#repository-map)
- [Where To Start](#where-to-start)
- [Current Milestones](#current-milestones)
- [Contributing](#contributing)

---

## What HI-OS Is

HI-OS is a **repository operating layer for AI coding agents**.

The app is what users touch.  
**HI-OS is what agents touch.**

Most repositories let agents read scattered files, infer intent, and edit code too early. HI-OS changes that by giving the repository a governed workflow:

```text
human request
  → intake classification
  → story packet
  → context pack
  → implementation
  → validation proof
  → trace
  → story governance gate
  → governance dashboard
```

Instead of free-form handoff, HI-OS makes the repository answer these questions first:

```text
What is the requested work?
Which product or architecture rules apply?
How risky is the change?
What context should the agent read?
What proof will show the work is done?
Which release or dashboard evidence can a human audit later?
```

---

## What HI-OS Gives You

<table>
  <tr>
    <th>Capability</th>
    <th>Purpose</th>
  </tr>
  <tr>
    <td><strong>Intake</strong></td>
    <td>Classify work before implementation.</td>
  </tr>
  <tr>
    <td><strong>Risk lanes</strong></td>
    <td>Separate Tiny, Normal, and High-Risk changes.</td>
  </tr>
  <tr>
    <td><strong>Story packets</strong></td>
    <td>Give each work item a durable contract.</td>
  </tr>
  <tr>
    <td><strong>Context packs</strong></td>
    <td>Give agents the right context without reading the whole repo.</td>
  </tr>
  <tr>
    <td><strong>Architecture checks</strong></td>
    <td>Prevent boundary violations before handoff.</td>
  </tr>
  <tr>
    <td><strong>Story verify</strong></td>
    <td>Block completion until required proof exists.</td>
  </tr>
  <tr>
    <td><strong>Release verify</strong></td>
    <td>Verify binaries, SHA256 assets, payload ZIP, version, and smoke command.</td>
  </tr>
  <tr>
    <td><strong>Governance dashboard</strong></td>
    <td>Export local proof into a static HTML dashboard.</td>
  </tr>
  <tr>
    <td><strong>CodeGraph / NotebookLM evidence</strong></td>
    <td>Use file-based provider boundaries for external intelligence.</td>
  </tr>
  <tr>
    <td><strong>Friction learning</strong></td>
    <td>Capture blockers and convert repeated friction into backlog signals.</td>
  </tr>
</table>

---

## Sovereign Identity

HI-OS has one tracked product identity.

```text
Harness Intelligence OS
short name: HI-OS
repository: ntu254/Harness-Intelligence-OS
default release origin: ntu254/Harness-Intelligence-OS
```

The identity lives in:

```text
hios.toml
```

Check it with:

```bash
scripts/bin/harness-cli identity
```

On Windows:

```powershell
.\scripts\bin\harness-cli.exe identity
```

Governance reports, dashboards, and release verification use this identity to make sure release trust stays aligned with the canonical origin.

---

## 5-Minute Quickstart

Use this when installing HI-OS into another project.

<details open>
<summary><strong>macOS / Linux</strong></summary>

```bash
curl -fsSL "https://raw.githubusercontent.com/ntu254/Harness-Intelligence-OS/main/scripts/install-harness.sh?$(date +%s)" | bash -s -- --yes
```

For merge mode:

```bash
curl -fsSL "https://raw.githubusercontent.com/ntu254/Harness-Intelligence-OS/main/scripts/install-harness.sh?$(date +%s)" | bash -s -- --merge --yes
```

For Claude Code users:

```bash
curl -fsSL "https://raw.githubusercontent.com/ntu254/Harness-Intelligence-OS/main/scripts/install-harness.sh?$(date +%s)" | bash -s -- --claude --yes
```

</details>

<details>
<summary><strong>Windows PowerShell</strong></summary>

```powershell
& ([scriptblock]::Create((irm "https://raw.githubusercontent.com/ntu254/Harness-Intelligence-OS/main/scripts/install-harness.ps1"))) -Yes
```

For merge mode:

```powershell
& ([scriptblock]::Create((irm "https://raw.githubusercontent.com/ntu254/Harness-Intelligence-OS/main/scripts/install-harness.ps1"))) -Merge -Yes
```

</details>

The installer copies the Harness operating docs and downloads the trusted prebuilt CLI to:

```text
scripts/bin/harness-cli
scripts/bin/harness-cli.exe
```

---

## First Run

Initialize the local Harness database:

```bash
scripts/bin/harness-cli init
scripts/bin/harness-cli query matrix
```

On Windows:

```powershell
.\scripts\bin\harness-cli.exe init
.\scripts\bin\harness-cli.exe query matrix
```

Local runtime evidence is ignored and should not be committed:

```text
harness.db
.harness/
```

---

## Create Your First Verified Story

Create intake and story metadata:

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

On Windows, use the same arguments with:

```powershell
.\scripts\bin\harness-cli.exe
```

Generate context, validate, trace, and verify:

```bash
scripts/bin/harness-cli context --story US-001
scripts/bin/harness-cli arch-check --story US-001
scripts/bin/harness-cli story update --id US-001 --unit 1 --integration 1 --e2e 0 --platform 0
scripts/bin/harness-cli trace --summary "Completed US-001" --story US-001 --outcome completed
scripts/bin/harness-cli story verify US-001
```

`story verify` runs the configured proof command and enforces the governance gate.

A story is not done until both pass.

---

## Core Workflow

Every serious task follows the same path:

```text
Human request
  → intake classification
  → story packet
  → context pack
  → implementation
  → validation proof
  → trace
  → story governance gate
  → governance dashboard
```

Common commands:

```bash
scripts/bin/harness-cli intake --summary "<work>" --lane normal --story US-001
scripts/bin/harness-cli context --story US-001
scripts/bin/harness-cli arch-check --story US-001
scripts/bin/harness-cli story verify US-001
scripts/bin/harness-cli trace --summary "<what happened>" --story US-001 --outcome completed
scripts/bin/harness-cli governance report --output .harness/reports/governance-report.json
scripts/bin/harness-cli governance dashboard --report .harness/reports/governance-report.json --output .harness/dashboard/index.html
```

Agents should read these before implementation:

```text
AGENTS.md
docs/HARNESS.md
docs/FEATURE_INTAKE.md
docs/ARCHITECTURE.md
docs/CONTEXT_RULES.md
```

Use this to inspect current proof state:

```bash
scripts/bin/harness-cli query matrix
```

---

## Governance Dashboard

Export governance evidence:

```bash
scripts/bin/harness-cli governance report \
  --output .harness/reports/governance-report.json

scripts/bin/harness-cli governance dashboard \
  --report .harness/reports/governance-report.json \
  --output .harness/dashboard/index.html
```

Open locally:

```text
.harness/dashboard/index.html
```

The dashboard shows:

<table>
  <tr>
    <th>Section</th>
    <th>What It Shows</th>
  </tr>
  <tr>
    <td>Story proof</td>
    <td>Unit, integration, E2E, platform, and custom proof status.</td>
  </tr>
  <tr>
    <td>Governance gate</td>
    <td>Pass/fail state for each story.</td>
  </tr>
  <tr>
    <td>Release evidence</td>
    <td>Trusted distribution verification results.</td>
  </tr>
  <tr>
    <td>Friction</td>
    <td>Captured blockers and workflow improvement signals.</td>
  </tr>
  <tr>
    <td>Maturity</td>
    <td>Current governance maturity summary.</td>
  </tr>
</table>

The dashboard is static HTML with no external assets.

---

## Trusted Distribution

HI-OS publishes prebuilt Harness CLI binaries and SHA256 assets for:

```text
macOS arm64
macOS x64
Linux x64
Linux arm64
Windows x64
```

Verify the public release chain:

```bash
scripts/bin/harness-cli release verify --version 0.7.0
```

On Windows:

```powershell
.\scripts\bin\harness-cli.exe release verify --version 0.7.0
```

`release verify` checks:

```text
release metadata
5 platform binaries
5 binary SHA256 files
production payload ZIP
production payload SHA256
host binary version
smoke command
```

Network or GitHub availability failures are:

```text
inconclusive
```

not:

```text
pass
```

The default public origin is:

```text
ntu254/Harness-Intelligence-OS
```

It is configured in:

```text
harness-release.toml
```

and aligned with tracked identity in:

```text
hios.toml
```

---

## Production-Clean Payload

For production distribution, HI-OS builds a platform-neutral payload ZIP.

It contains the installer source and operating contract, without:

```text
Rust source
historical story packets
archived plans
runtime evidence
local harness.db
.harness runtime files
```

Build and verify:

```bash
bash scripts/build-production-payload.sh --version 0.7.0
python scripts/verify-production-payload.py --version 0.7.0 --source-check
```

Generated files live under:

```text
dist/
```

Public release upload remains part of release hardening.

---

## CodeGraph And NotebookLM Evidence

HI-OS uses file-based provider boundaries.

External providers do **not** write directly into Harness SQLite.

### CodeGraph impact evidence

```bash
scripts/bin/harness-cli codegraph impact \
  --story US-001 \
  --mode changed-files \
  --changed-files .harness/context/changed-files.txt
```

### NotebookLM grounded brief evidence

```bash
scripts/bin/harness-cli notebooklm brief \
  --story US-001 \
  --notebook <notebook-id-or-alias> \
  --query "Find citation-backed context for this story."
```

Provider rules:

<table>
  <tr>
    <th>Condition</th>
    <th>Result</th>
  </tr>
  <tr>
    <td>Provider unavailable</td>
    <td><code>inconclusive</code></td>
  </tr>
  <tr>
    <td>Network/session unavailable</td>
    <td><code>inconclusive</code></td>
  </tr>
  <tr>
    <td>Missing citations</td>
    <td><code>fail</code></td>
  </tr>
  <tr>
    <td>Malformed provider output</td>
    <td><code>fail</code></td>
  </tr>
  <tr>
    <td>Valid artifact + successful ingest</td>
    <td><code>pass</code></td>
  </tr>
</table>

Harness never stores:

```text
Google credentials
cookies
tokens
browser profiles
provider session files
```

---

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
docs/troubleshooting.md           installer, provider, gate, and dashboard fixes
docs/COMMAND_COOKBOOK.md          grouped command examples
docs/demo/                        example transformation from idea to story
docs/archive/                     historical phase plans and early specs
docs/stories/                     story packets and validation evidence
docs/decisions/                   durable decisions
docs/schemas/                     artifact and report schemas
packaging/production-include.toml production payload contract
scripts/bin/harness-cli           installed CLI path in target projects
scripts/README.md                 command and installer details
```

---

## Where To Start

<table>
  <tr>
    <th>Need</th>
    <th>Read</th>
  </tr>
  <tr>
    <td>Clean clone setup</td>
    <td><code>docs/adoption/clean-clone-walkthrough.md</code></td>
  </tr>
  <tr>
    <td>Full agent workflow</td>
    <td><code>docs/examples/full-agent-workflow.md</code></td>
  </tr>
  <tr>
    <td>Codex instructions</td>
    <td><code>docs/agents/codex.md</code></td>
  </tr>
  <tr>
    <td>Claude Code instructions</td>
    <td><code>docs/agents/claude-code.md</code></td>
  </tr>
  <tr>
    <td>Cursor instructions</td>
    <td><code>docs/agents/cursor.md</code></td>
  </tr>
  <tr>
    <td>Troubleshooting</td>
    <td><code>docs/troubleshooting.md</code></td>
  </tr>
  <tr>
    <td>Command examples</td>
    <td><code>docs/COMMAND_COOKBOOK.md</code></td>
  </tr>
  <tr>
    <td>Installer details</td>
    <td><code>scripts/README.md</code></td>
  </tr>
</table>

For command details:

```bash
scripts/bin/harness-cli help
```

---

## Current Milestones

<table>
  <tr>
    <th>Version</th>
    <th>Theme</th>
  </tr>
  <tr>
    <td>v0.1</td>
    <td>Context-grounded auto intake</td>
  </tr>
  <tr>
    <td>v0.2</td>
    <td>Blocking governance gate</td>
  </tr>
  <tr>
    <td>v0.3</td>
    <td>Trusted distribution and evidence trail</td>
  </tr>
  <tr>
    <td>v0.4</td>
    <td>MCP artifact contracts and provider adapters</td>
  </tr>
  <tr>
    <td>v0.5</td>
    <td>Structured friction learning loop</td>
  </tr>
  <tr>
    <td>v0.6</td>
    <td>Governance report, maturity summary, and static dashboard</td>
  </tr>
  <tr>
    <td>v0.7</td>
    <td>Adoption Ready</td>
  </tr>
</table>

HI-OS v0.7 is the completed adoption line:

```text
installable
understandable
debuggable
production-clean
publicly verifiable
```

---

## Contributing

Useful contributions include:

```text
testing the clean clone walkthrough
improving first-time user docs
adding real project examples
reporting agent failure cases caused by missing repo context
improving validation patterns for different stacks
comparing behavior across Codex, Claude Code, Cursor, and other agents
```

See:

```text
CONTRIBUTING.md
```

---

## Short Description

**HI-OS is an agent-ready repository operating layer for Codex, Claude Code, Cursor, and other coding agents: intake, context packs, story gates, release verification, governance dashboards, and durable traces.**
