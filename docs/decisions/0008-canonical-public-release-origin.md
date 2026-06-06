# 0008 Canonical Public Release Origin

Date: 2026-06-06

## Status

Proposed

## Context

HI-OS v0.2.0 is released privately with verified binaries, SHA256 assets, and
passing governance checks. It is not public ship-ready because the installer
defaults to a public upstream repository that does not host the v0.2.0 release.

HI-OS v0.3.0 cannot implement authoritative release verification until one
public distribution origin is accepted. Otherwise the command could validate a
tag, asset set, checksum, or installer path that is not the official release
contract.

## Decision

No option is accepted yet.

The selected option must define:

- The canonical public repository and release URL.
- The repository used by default installer source URLs.
- The repository used by default CLI asset URLs.
- How private development releases relate to public distribution releases.
- Who promotes a private release into the canonical public origin.

Implementation of `release verify` must not begin until this decision becomes
Accepted.

## Alternatives Considered

1. Make `ntu254/Harness-Intelligence-OS` the canonical public origin.
   This creates one repository for development and distribution, but requires
   making the current private repository public or publishing an equivalent
   public repository.
2. Keep `hoangnb24/repository-harness` as the canonical public origin.
   This preserves current installer URLs, but requires an accepted upstream
   publication path and release ownership.
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

- Option 1 changes repository visibility or ownership expectations.
- Option 2 depends on upstream coordination.
- Option 3 adds promotion and synchronization operations.

## Follow-Up

- Select and accept one option.
- Document the installer and release URL contract.
- Update Backlog #1 with the accepted outcome.
- Only then design `harness-cli release verify --version <version>`.
