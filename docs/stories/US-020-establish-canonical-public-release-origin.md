# US-020 Establish Canonical Public Release Origin

## Status

planned

## Lane

normal

## Product Contract

HI-OS has one documented canonical public release origin that the installer,
release workflow, documentation, and future release verification can treat as
authoritative.

## Relevant Product Docs

- `RELEASE_NOTES_v0.2.0.md`
- `docs/decisions/0008-canonical-public-release-origin.md`
- `scripts/README.md`

## Acceptance Criteria

- The canonical public release origin is documented.
- The installer target origin is documented.
- The relationship between private development releases and public
  distribution releases is explained.
- The v0.3 release verification scope is defined.
- No `release verify` implementation starts before the origin decision is
  accepted.

## Design Notes

- Commands: no new command in this story.
- Queries: inspect release metadata and installer configuration only.
- API: GitHub repository and release URLs are decision inputs.
- Tables: no schema changes.
- Domain rules: one origin must be authoritative for public distribution.
- UI surfaces: release documentation and installer output.

## Validation

| Layer | Expected proof |
| --- | --- |
| Unit | Not applicable; decision-only story. |
| Integration | Installer and workflow origins agree after an option is accepted. |
| E2E | Deferred until the canonical public origin is available. |
| Platform | No platform changes in this story. |
| Release | Decision review confirms one authoritative public origin. |

## Harness Delta

Backlog #1 becomes the first prerequisite for HI-OS v0.3.0. Release
verification design remains blocked until decision `0008` is accepted.

## Evidence

- HI-OS v0.2.0 is released privately with verified artifacts.
- The private origin hosts `harness-cli-v0.2.0`.
- The current public installer target does not host that release.
