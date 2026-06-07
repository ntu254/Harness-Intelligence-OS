# US-048 Design: Production Payload Contract

## Distribution Layers

```text
source repository
  -> production payload ZIP
  -> installer copies HI-OS operating files
  -> installer obtains platform CLI from trusted release assets
```

The production ZIP is platform-neutral. It carries installer source and the
HI-OS operating contract, not a platform binary.

## Canonical Manifest

`packaging/production-include.toml` contains:

- include patterns;
- exclusion patterns;
- required files;
- forbidden paths.

The include list covers every file required by the current installer, plus
adoption docs, agent packs, examples, schemas, and validators.

## Builder

`scripts/build-production-payload.py` is the canonical implementation.
PowerShell and Bash scripts are thin entrypoints.

The builder:

1. Parses TOML with the Python standard library.
2. Expands and sorts included files.
3. Applies exclusions.
4. Rejects symlinks and missing required files.
5. Writes a deterministic internal JSON manifest.
6. Writes a deterministic ZIP with fixed timestamps and permissions.
7. Writes the ZIP SHA256 asset.

## Verifier

`scripts/verify-production-payload.py` checks:

- external ZIP SHA256;
- one expected archive root;
- no path traversal or duplicate entries;
- internal manifest version and file count;
- every internal file SHA256;
- required files;
- forbidden paths;
- optional exact comparison with the source manifest.

## Clean Install

The smoke test extracts the ZIP outside the repository, runs the packaged
installer into another empty directory, supplies a trusted CLI artifact
separately, initializes Harness state, and exports governance report/dashboard
evidence.
