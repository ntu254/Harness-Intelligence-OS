# US-048 Validation

US-048 proves HI-OS can be packaged and installed without cloning the full
development repository.

## Acceptance Criteria

- Production manifest exists.
- ZIP and ZIP SHA256 are generated.
- PowerShell and Bash entrypoints produce the canonical artifact.
- Required core files are present.
- Development history, source code, runtime state, and `spec.md` are absent.
- Internal and external SHA256 verification passes.
- Rebuilding with unchanged inputs produces the same ZIP hash.
- Clean extraction and installer smoke pass.
- Installed CLI version command passes.
- Governance report/dashboard pass in the clean target.
- Full repository validation and Detailed trace pass.

## Evidence

### Payload

- Canonical contract: `packaging/production-include.toml`.
- Artifact: `dist/hios-production-v0.7.0.zip`.
- External checksum asset:
  `dist/hios-production-v0.7.0.zip.sha256`.
- Final source-matched ZIP SHA256:
  `42532efb4c5f0a1da81d50dabc6d006ba820ec11ad9e9e0111edc78b98ed12db`.
- Payload contains 73 tracked operating files plus its internal manifest.
- Python, PowerShell, and Bash entrypoints produced the same ZIP hash.
- A deliberately incorrect external checksum was rejected.
- Decision 0014 mechanical verification passed.

### Clean Install

The ZIP was extracted under `C:\tmp\hios-us048`, outside the source repo. A
separate local HTTP endpoint served the Windows CLI binary and SHA256 as a
release-asset boundary.

- Packaged installer copied 70 files into an empty target.
- Installer downloaded and verified `harness-cli-windows-x64.exe`.
- Installed CLI reported `harness-cli 0.6.0`.
- `harness-cli identity` reported the tracked HI-OS identity and public origin.
- Database initialization passed.
- Governance report schema verification passed.
- Static governance dashboard export passed.
- Installed adoption, MCP contract, and friction taxonomy verifiers passed.
- Installed agent packs, cookbook, troubleshooting guide, and full workflow
  example were present.

The 0.6.0 CLI result is intentional. US-048 does not change the current CLI
version or installer pin. US-045 will bump to 0.7.0, rebuild the payload, and
publish it.

### Repository Validation

- `cargo fmt --check`: pass.
- `cargo test --workspace`: 50 passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- Adoption docs verifier: pass.
- MCP artifact contract verifier: pass.
- Friction taxonomy verifier: pass.
- Production payload source verification: pass.
- Decision 0014 verification: pass.
- US-048 architecture check: pass.
- Public release verify for v0.6.0: 10 assets, checksum, version, and smoke pass.
- Trace #47: Detailed 3/3, meeting the High-Risk requirement.
- US-048 mechanical verification: pass.
- US-048 governance gate: pass.
- Final governance report/dashboard: 28 gates pass, 0 fail, trusted maturity
  score 93.

### Scope Confirmation

- No CLI version bump.
- No installer release pin change.
- No release or tag publication.
- No MCP/provider behavior change.
- `spec.md` remains outside the tracked payload and commit.
