# US-020 Establish Canonical Public Release Origin

## Status

implemented

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

- [x] The canonical public release origin is documented.
- [x] The installer target origin is documented.
- [x] The relationship between development staging and public distribution
  releases is explained.
- [x] The v0.3 release verification scope is defined.
- [x] No `release verify` implementation started before the origin decision was
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

Decision `0008` establishes one canonical public origin. Release verification
remains a separate future story.

## Evidence

- Decision `0008` accepted `ntu254/Harness-Intelligence-OS`.
- Installer source and release asset defaults use the accepted origin.
- Development staging uses branches, pull requests, workflow artifacts, and
  prereleases in the same public repository.
- No `release verify` command was implemented.
