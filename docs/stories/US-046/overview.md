# US-046: Establish HI-OS Sovereign Identity

## Status

Implemented / High-Risk

## Issue

- GitHub issue: #23
- Milestone: HI-OS v0.7.0 Adoption Ready

## Goal

Make HI-OS the explicit product identity for the repository and generated
governance evidence.

## Problem

HI-OS adoption docs now present the project as Harness Intelligence OS, but
identity is still spread across README prose, Git remote metadata, and
`harness-release.toml`. A new user or agent should be able to query one tracked
identity contract and see the same identity in reports, dashboards, and release
verification defaults.

## In Scope

- Add tracked `hios.toml` sovereign identity config.
- Add `harness-cli identity`.
- Include HI-OS identity in governance report JSON.
- Render HI-OS identity in the static governance dashboard.
- Check `release verify` default origin against tracked identity.
- Document Decision 0013.
- Keep README/docs aligned on HI-OS as the primary identity.

## Out Of Scope

- v0.7 release/tag/installer pin changes.
- US-047 legacy pruning, archive, or delete work.
- CodeGraph, NotebookLM, or MCP behavior changes.
- Credentials, sessions, or provider state.
