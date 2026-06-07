# US-045 Design

## Release Contract

Releases before 0.7.0 keep their immutable ten-asset contract:

- five platform binaries;
- five binary SHA256 files.

Release 0.7.0 and later require twelve assets:

- the ten CLI assets;
- `hios-production-vX.Y.Z.zip`;
- `hios-production-vX.Y.Z.zip.sha256`.

`harness-cli release verify` selects the contract from the requested semantic
version. For v0.7+, it downloads the production ZIP and checksum, verifies the
SHA256, then continues with host binary version and smoke checks.

## Workflow

```text
verify source
  -> build five native CLI assets
  -> build deterministic production payload
  -> publish twelve assets
  -> release verify 0.7.0
  -> clean public installer smoke
  -> governance report/dashboard
  -> story gate
```

## Guardrails

- Older release assets remain unchanged.
- Payload checksum failure is a release verification failure.
- Provider unavailability does not block release unless provider proof is
  explicitly required by this story.
- The production payload does not embed a platform CLI binary.
