# 0008 Canonical Public Release Origin

Date: 2026-06-06

## Status

Accepted

## Context

HI-OS v0.2.0 was initially released privately with verified binaries, SHA256
assets, and passing governance checks. The installer defaulted to a different
public upstream repository that did not host the v0.2.0 release.

HI-OS v0.3.0 cannot implement authoritative release verification until one
public distribution origin is accepted. Otherwise the command could validate a
tag, asset set, checksum, or installer path that is not the official release
contract.

## Decision

Accept `ntu254/Harness-Intelligence-OS` as the canonical public source and
release origin.

- Canonical repository: `https://github.com/ntu254/Harness-Intelligence-OS`
- Installer source:
  `https://raw.githubusercontent.com/ntu254/Harness-Intelligence-OS/main`
- CLI assets:
  `https://github.com/ntu254/Harness-Intelligence-OS/releases/download/<tag>`
- Development and staging use branches, pull requests, workflow artifacts, and
  prereleases inside the same public repository.
- Stable installer pins reference accepted release tags in this repository.

The repository must remain public while it is the canonical installer and
release authority.

## Alternatives Considered

1. Make `ntu254/Harness-Intelligence-OS` the canonical public origin.
   Accepted. This creates one repository for source, development, and
   distribution and removes origin ambiguity.
2. Keep the former upstream repository as the canonical public origin.
   Rejected because it would preserve split ownership and require an external
   publication path.
3. Split private development and public distribution origins.
   The private origin remains development/staging, while a separate public
   repository becomes the installer and release authority. This preserves
   private development but requires an explicit, auditable promotion process.

## Consequences

Positive:

- Public installer and release verification can share one authoritative source.
- Private staging and public distribution responsibilities become explicit.
- v0.3 evidence can prove the complete distribution chain.

Tradeoffs:

- Development work and release assets are publicly visible.
- Option 2 depends on upstream coordination.
- Option 3 adds promotion and synchronization operations.

## Follow-Up

- Keep installer defaults and repository metadata aligned with this origin.
- Verify unauthenticated source and release downloads.
- Close Backlog #1 after public installer smoke proof.
- Create a separate story before implementing
  `harness-cli release verify --version <version>`.
