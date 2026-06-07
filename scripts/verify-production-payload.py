#!/usr/bin/env python3
"""Verify a production-clean HI-OS ZIP, manifest, and SHA256 asset."""

from __future__ import annotations

import argparse
from hashlib import sha256
import json
from pathlib import Path
import sys
from zipfile import BadZipFile, ZipFile

from production_payload_lib import (
    INTERNAL_MANIFEST,
    collect_source_files,
    load_contract,
    matches_any,
    parse_checksum,
    safe_archive_relative,
)


ROOT = Path(__file__).resolve().parents[1]


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--version", required=True)
    parser.add_argument(
        "--manifest",
        type=Path,
        default=ROOT / "packaging" / "production-include.toml",
    )
    parser.add_argument("--archive", type=Path)
    parser.add_argument("--checksum", type=Path)
    parser.add_argument(
        "--source-check",
        action="store_true",
        help="Require archive paths and bytes to match the current source tree.",
    )
    return parser.parse_args()


def verify() -> tuple[Path, int, str]:
    args = parse_args()
    contract = load_contract(args.manifest.resolve(), args.version)
    archive = (
        args.archive.resolve()
        if args.archive
        else ROOT / "dist" / f"{contract.name}-v{args.version}.zip"
    )
    checksum = (
        args.checksum.resolve()
        if args.checksum
        else archive.with_name(f"{archive.name}.sha256")
    )

    expected_hash, checksum_name = parse_checksum(checksum)
    if checksum_name != archive.name:
        raise ValueError(
            f"checksum names '{checksum_name}', expected archive '{archive.name}'"
        )
    actual_hash = sha256(archive.read_bytes()).hexdigest()
    if actual_hash != expected_hash:
        raise ValueError(
            f"ZIP SHA256 mismatch: expected {expected_hash}, got {actual_hash}"
        )

    with ZipFile(archive) as payload:
        names = payload.namelist()
        if len(names) != len(set(names)):
            raise ValueError("archive contains duplicate entries")
        if any(name.endswith("/") for name in names):
            raise ValueError("archive should contain files only, not directory entries")

        relative_names = [
            safe_archive_relative(name, contract.archive_root) for name in names
        ]
        manifest_relative = INTERNAL_MANIFEST
        if manifest_relative not in relative_names:
            raise ValueError(f"archive is missing {INTERNAL_MANIFEST}")

        manifest_name = f"{contract.archive_root}/{INTERNAL_MANIFEST}"
        internal = json.loads(payload.read(manifest_name))
        if internal.get("schema_version") != "1.0.0":
            raise ValueError("internal manifest schema_version must be 1.0.0")
        if internal.get("artifact_type") != "hios-production-payload":
            raise ValueError("internal manifest artifact_type is invalid")
        if internal.get("version") != args.version:
            raise ValueError("internal manifest version does not match requested version")

        entries = internal.get("files")
        if not isinstance(entries, list):
            raise ValueError("internal manifest files must be an array")
        if internal.get("file_count") != len(entries):
            raise ValueError("internal manifest file_count does not match files")

        recorded_paths: list[str] = []
        archive_files: dict[str, bytes] = {}
        for entry in entries:
            if not isinstance(entry, dict):
                raise ValueError("internal manifest contains a non-object file entry")
            path = entry.get("path")
            expected = entry.get("sha256")
            if not isinstance(path, str) or not isinstance(expected, str):
                raise ValueError("internal manifest file entry is incomplete")
            archive_name = f"{contract.archive_root}/{path}"
            content = payload.read(archive_name)
            actual = sha256(content).hexdigest()
            if actual != expected:
                raise ValueError(f"internal SHA256 mismatch for {path}")
            recorded_paths.append(path)
            archive_files[path] = content

        if recorded_paths != sorted(recorded_paths):
            raise ValueError("internal manifest files must be sorted")
        if len(recorded_paths) != len(set(recorded_paths)):
            raise ValueError("internal manifest contains duplicate file paths")

        expected_relative = sorted([INTERNAL_MANIFEST, *recorded_paths])
        if sorted(relative_names) != expected_relative:
            raise ValueError("archive entries do not match internal manifest")

        missing = sorted(set(contract.required) - set(recorded_paths))
        if missing:
            raise ValueError(f"archive is missing required files: {', '.join(missing)}")
        forbidden = sorted(
            path for path in recorded_paths if matches_any(path, contract.forbidden)
        )
        if forbidden:
            raise ValueError(
                f"archive contains forbidden files: {', '.join(forbidden)}"
            )

        if args.source_check:
            source_files = collect_source_files(ROOT, contract)
            if set(source_files) != set(archive_files):
                missing_from_archive = sorted(set(source_files) - set(archive_files))
                extra_in_archive = sorted(set(archive_files) - set(source_files))
                raise ValueError(
                    "archive/source path mismatch; "
                    f"missing={missing_from_archive}, extra={extra_in_archive}"
                )
            changed = sorted(
                path
                for path, content in source_files.items()
                if archive_files[path] != content
            )
            if changed:
                raise ValueError(
                    f"archive bytes differ from source: {', '.join(changed)}"
                )

    return archive, len(recorded_paths), actual_hash


def main() -> int:
    archive, count, digest = verify()
    print(f"Production payload verified: {archive}")
    print(f"Payload files: {count}")
    print(f"ZIP SHA256: {digest}")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (BadZipFile, KeyError, OSError, ValueError, json.JSONDecodeError) as error:
        print(f"Production payload verification failed: {error}", file=sys.stderr)
        raise SystemExit(1)
