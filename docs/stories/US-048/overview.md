# US-048: Build Production-Clean HI-OS Distribution Payload

## Status

Implemented / High-Risk

## Issue

- GitHub issue: #25
- Milestone: HI-OS v0.7.0 Adoption Ready

## Goal

Build a minimal, auditable, installable HI-OS production payload without
shipping the full development repository.

## In Scope

- Define a tracked production include/exclude manifest.
- Build `dist/hios-production-v0.7.0.zip`.
- Generate `dist/hios-production-v0.7.0.zip.sha256`.
- Provide PowerShell and Bash build entrypoints.
- Validate required and forbidden files.
- Validate internal file hashes and external ZIP checksum.
- Prove an extracted payload can install HI-OS into a clean target.
- Prove the installed CLI, governance report, and dashboard work.

## Out Of Scope

- Bumping the Harness CLI to 0.7.0.
- Changing the installer release pin.
- Publishing a GitHub release or tag.
- Building all five public platform binaries.
- Modifying MCP/provider behavior.

Those release actions remain US-045.
