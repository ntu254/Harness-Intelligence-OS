# US-045 Execution Plan

1. Open issue #26, intake #37, and the High-Risk story.
2. Add the version-aware twelve-asset release contract.
3. Update the release workflow to build and publish the production payload.
4. Bump CLI version and installer pin to 0.7.0.
5. Add release notes and update current-version adoption examples.
6. Run local Rust, documentation, schema, payload, installer, and governance
   validation.
7. Verify the previous public v0.6.0 release.
8. Commit and push release preparation.
9. Tag `harness-cli-v0.7.0` and wait for the release workflow.
10. Verify all twelve public assets and run clean public installer smoke.
11. Record Detailed trace, pass the story gate, close issue and milestone.

## Stop Conditions

Stop publication if:

- any content story is incomplete;
- local validation fails;
- payload output is not deterministic;
- the workflow cannot produce all twelve assets;
- public checksum, version, smoke, or installer verification fails.
