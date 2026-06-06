#!/usr/bin/env python3
import copy
import json
from pathlib import Path

from jsonschema import Draft202012Validator, FormatChecker


ROOT = Path(__file__).resolve().parents[1]
SCHEMA = ROOT / "docs" / "schemas" / "governance-report.schema.json"
DOC = ROOT / "docs" / "GOVERNANCE_REPORT.md"
DECISION = ROOT / "docs" / "decisions" / "0012-governance-report-schema.md"


def expect_valid(validator, instance, label):
    errors = sorted(validator.iter_errors(instance), key=lambda error: list(error.path))
    if errors:
        raise AssertionError(f"{label} should be valid: {errors[0].message}")


def expect_invalid(validator, instance, label):
    if not list(validator.iter_errors(instance)):
        raise AssertionError(f"{label} should be invalid")


with SCHEMA.open(encoding="utf-8") as handle:
    schema = json.load(handle)
Draft202012Validator.check_schema(schema)
validator = Draft202012Validator(schema, format_checker=FormatChecker())

for path in [DOC, DECISION]:
    text = path.read_text(encoding="utf-8")
    for required in [
        "governance-report",
        "story",
        "release",
        "friction",
    ]:
        if required not in text:
            raise AssertionError(f"{required} missing from {path}")

report = {
    "schema_version": "1.0.0",
    "artifact_type": "governance-report",
    "report_id": "77777777-7777-4777-8777-777777777777",
    "generated_at": "2026-06-07T00:00:00Z",
    "repository": {
        "origin": "ntu254/Harness-Intelligence-OS",
        "commit": "cc1f5e9",
        "branch": "main",
    },
    "story_summary": {
        "total": 15,
        "implemented": 14,
        "in_progress": 1,
        "blocked": 0,
    },
    "gate_summary": {
        "pass": 14,
        "fail": 1,
        "not_run": 0,
    },
    "validation_summary": {
        "commands": [
            {
                "command": "cargo test --workspace",
                "result": "pass",
            }
        ]
    },
    "release_summary": {
        "latest_version": "0.5.0",
        "release_verify_result": "pass",
        "assets_checked": 10,
    },
    "friction_summary": {
        "events": 2,
        "high_severity": 1,
        "open_backlog_suggestions": 1,
        "open_rule_proposals": 1,
    },
    "stories": [
        {
            "story_id": "US-033",
            "status": "implemented",
            "risk_lane": "high_risk",
            "proof": {
                "unit": True,
                "integration": True,
                "e2e": True,
                "platform": True,
            },
            "gate_result": "pass",
            "missing_evidence": [],
            "evidence": "release verify 0.5.0 pass",
        }
    ],
}

expect_valid(validator, report, "complete governance report")

invalid = copy.deepcopy(report)
invalid["artifact_type"] = "dashboard"
expect_invalid(validator, invalid, "wrong artifact type")

invalid = copy.deepcopy(report)
invalid["release_summary"]["release_verify_result"] = "warning"
expect_invalid(validator, invalid, "invalid release result")

invalid = copy.deepcopy(report)
invalid["stories"][0]["gate_result"] = "inconclusive"
expect_invalid(validator, invalid, "story gate cannot be inconclusive")

invalid = copy.deepcopy(report)
invalid["validation_summary"]["commands"][0]["result"] = "warning"
expect_invalid(validator, invalid, "validation command cannot be warning")

invalid = copy.deepcopy(report)
invalid["story_summary"]["total"] = -1
expect_invalid(validator, invalid, "negative count")

invalid = copy.deepcopy(report)
invalid["stories"][0]["risk_lane"] = "urgent"
expect_invalid(validator, invalid, "invalid risk lane")

invalid = copy.deepcopy(report)
invalid["unexpected"] = True
expect_invalid(validator, invalid, "additional root property")

invalid = copy.deepcopy(report)
invalid["stories"][0].pop("gate_result")
expect_invalid(validator, invalid, "missing gate result")

print("Governance report schema and semantic fixtures passed.")
