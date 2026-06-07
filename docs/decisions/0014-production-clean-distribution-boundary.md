# Decision 0014: Production-Clean Distribution Boundary

## Status

Accepted

## Context

The HI-OS source repository contains implementation code, release workflows,
historical story packets, decisions, archived phase plans, and local governance
evidence. That full source tree is useful for maintainers, but it is not the
right installation surface for production users.

Users need a smaller, auditable payload that contains the HI-OS operating
contract without development history or runtime state.

## Decision

HI-OS distribution has three separate layers:

1. The source repository contains implementation and governance history.
2. Platform-specific Harness CLI binaries and SHA256 files are release assets.
3. A production-clean ZIP contains the installer source payload, core configs,
   schemas, operating docs, adoption docs, agent packs, templates, and durable
   decisions.

The production payload is defined by
`packaging/production-include.toml`. PowerShell and Bash entrypoints call one
canonical Python builder so archive selection and byte layout cannot drift
between platforms.

The ZIP contains an internal deterministic manifest with the payload version
and SHA256 for every included file. A separate `.sha256` asset authenticates
the ZIP itself.

The production payload does not embed a platform CLI binary. The installer
continues to obtain the CLI from the trusted release asset chain. This keeps
one production payload platform-neutral.

## Exclusions

The production payload excludes:

- Rust workspace source and build output;
- historical story packets and archived phase/spec documents;
- local SQLite and `.harness/` evidence;
- CodeGraph indexes and provider runtime artifacts;
- temporary, backup, and previous distribution output;
- untracked input such as `spec.md`.

## Consequences

- Users can install HI-OS without cloning the development repository.
- Release hardening must publish the production ZIP and its SHA256 alongside
  the five CLI binaries and five binary SHA256 files.
- Installer pin/version changes remain release work, not payload-contract work.
- Any new installer-required file must also be added to the production payload
  contract and verifier.
