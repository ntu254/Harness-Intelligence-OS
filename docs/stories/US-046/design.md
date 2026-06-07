# US-046 Design: Sovereign Identity Boundary

## Identity Contract

US-046 introduces `hios.toml`:

```toml
[identity]
product_name = "Harness Intelligence OS"
short_name = "HI-OS"
slug = "hios"
repository = "ntu254/Harness-Intelligence-OS"
default_release_origin = "ntu254/Harness-Intelligence-OS"
```

This is a source-controlled product identity contract. It is deliberately
separate from runtime evidence under `.harness/`.

## CLI Surface

`harness-cli identity` reads `hios.toml`, validates required fields, and prints
the configured identity. It does not initialize or mutate the Harness database.

## Governance Evidence

Governance reports include an `identity` object with the tracked HI-OS product
identity. Dashboard export renders that identity next to the repository and
release-origin evidence.

The report still includes Git repository metadata because commit and branch are
runtime state. Product identity and Git metadata answer different questions:

- identity: what product/repository this HI-OS installation claims to be;
- repository: what Git checkout generated the report.

## Release Alignment

`harness-release.toml` remains the release policy source. `release verify`
continues to default to the release config origin, but it validates that
`hios.toml.identity.default_release_origin` and `hios.toml.identity.repository`
match `harness-release.toml.origin`.

If identity and release policy disagree, release verification fails before
contacting GitHub.

## Guardrails

- No credential, token, cookie, session, or provider state is stored.
- Governance report and dashboard are read-only outputs.
- Legacy repository-harness artifact pruning belongs to US-047.
- No release tag, installer pin, or public asset is changed by US-046.
