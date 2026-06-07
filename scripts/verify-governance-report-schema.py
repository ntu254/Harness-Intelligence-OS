#!/usr/bin/env python3
import copy
import json
import sys
from pathlib import Path

from jsonschema import Draft202012Validator, FormatChecker


ROOT = Path(__file__).resolve().parents[1]
SCHEMA = ROOT / "docs" / "schemas" / "governance-report.schema.json"
DOC = ROOT / "docs" / "GOVERNANCE_REPORT.md"
DECISION = ROOT / "docs" / "decisions" / "0012-governance-report-schema.md"
IDENTITY_DECISION = ROOT / "docs" / "decisions" / "0013-hi-os-sovereign-identity.md"


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
        "identity",
        "story",
        "release",
        "friction",
    ]:
        if required not in text:
            raise AssertionError(f"{required} missing from {path}")

identity_decision_text = IDENTITY_DECISION.read_text(encoding="utf-8")
for required in [
    "hios.toml",
    "identity",
    "release",
    "governance",
    "ntu254/Harness-Intelligence-OS",
]:
    if required not in identity_decision_text:
        raise AssertionError(f"{required} missing from {IDENTITY_DECISION}")

report = {
    "schema_version": "1.1.0",
    "artifact_type": "governance-report",
    "report_id": "77777777-7777-4777-8777-777777777777",
    "generated_at": "2026-06-07T00:00:00Z",
    "identity": {
        "product_name": "Harness Intelligence OS",
        "short_name": "HI-OS",
        "slug": "hios",
        "repository": "ntu254/Harness-Intelligence-OS",
        "default_release_origin": "ntu254/Harness-Intelligence-OS",
    },
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
    "maturity_summary": {
        "score": 85,
        "level": "trusted",
        "gate_pass_percent": 93,
        "validation_pass_percent": 100,
        "release_verified": True,
        "open_governance_gaps": 2,
        "notes": ["Release verification passed."],
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
invalid.pop("identity")
expect_invalid(validator, invalid, "missing identity")

invalid = copy.deepcopy(report)
invalid["identity"]["default_release_origin"] = ""
expect_invalid(validator, invalid, "empty identity release origin")

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
invalid["maturity_summary"]["score"] = 101
expect_invalid(validator, invalid, "maturity score over 100")

invalid = copy.deepcopy(report)
invalid["maturity_summary"]["level"] = "perfect"
expect_invalid(validator, invalid, "invalid maturity level")

invalid = copy.deepcopy(report)
invalid["stories"][0]["risk_lane"] = "urgent"
expect_invalid(validator, invalid, "invalid risk lane")

invalid = copy.deepcopy(report)
invalid["unexpected"] = True
expect_invalid(validator, invalid, "additional root property")

invalid = copy.deepcopy(report)
invalid["stories"][0].pop("gate_result")
expect_invalid(validator, invalid, "missing gate result")

for report_path in sys.argv[1:]:
    with Path(report_path).open(encoding="utf-8") as handle:
        generated_report = json.load(handle)
    expect_valid(validator, generated_report, f"generated report {report_path}")

print("Governance report schema and semantic fixtures passed.")
