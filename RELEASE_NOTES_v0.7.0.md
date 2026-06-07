# HI-OS v0.7.0: Adoption Ready

HI-OS v0.7.0 turns the governed development system into a clear, installable,
production-clean distribution for new users and coding agents.

## Added

- Five-minute README quickstart.
- Clean clone maintainer walkthrough.
- End-to-end agent workflow example from intake through dashboard.
- Agent instruction packs for Codex, Claude Code, and Cursor.
- Command cookbook and troubleshooting guide.
- Tracked sovereign HI-OS identity through `hios.toml`.
- Production-clean, platform-neutral distribution payload:
  - deterministic ZIP layout;
  - internal per-file SHA256 manifest;
  - external ZIP SHA256 asset;
  - manifest-driven required and forbidden paths.
- Version-aware trusted release verification:
  - ten CLI distribution assets for releases before v0.7;
  - twelve required assets for v0.7 and later;
  - production payload download and SHA256 verification.

## Distribution

The public release contains:

- five native CLI binaries;
- five binary SHA256 files;
- `hios-production-v0.7.0.zip`;
- `hios-production-v0.7.0.zip.sha256`.

The production payload contains HI-OS operating docs, configs, schemas,
installers, adoption material, agent packs, templates, durable decisions, and
verifiers. It excludes Rust source, historical story packets, archived plans,
local databases, runtime evidence, build output, and untracked specifications.

## Verified

- Rust formatting, tests, and Clippy.
- Adoption, governance report, MCP contract, and friction taxonomy verifiers.
- Deterministic production payload generation across Python, PowerShell, and
  Bash entrypoints.
- Clean production payload installation.
- Public release asset discovery, binary SHA256, payload SHA256, version, and
  smoke execution.
- Public installer smoke.
- Governance report and static dashboard.
- High-Risk Detailed trace and story governance gate.

## Notes

- Releases v0.2.0 through v0.6.0 and their assets remain unchanged.
- The production payload does not embed a platform binary. Its installer
  obtains the CLI from the trusted GitHub release asset chain.
- Provider unavailability remains `inconclusive`, never `pass`.
