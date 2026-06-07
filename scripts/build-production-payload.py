#!/usr/bin/env python3
"""Build a deterministic production-clean HI-OS ZIP and SHA256 asset."""

from __future__ import annotations

import argparse
from pathlib import Path
import sys

from production_payload_lib import collect_source_files, load_contract, write_archive


ROOT = Path(__file__).resolve().parents[1]


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--version", required=True)
    parser.add_argument(
        "--manifest",
        type=Path,
        default=ROOT / "packaging" / "production-include.toml",
    )
    parser.add_argument("--out-dir", type=Path, default=ROOT / "dist")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    manifest = args.manifest.resolve()
    out_dir = args.out_dir.resolve()
    contract = load_contract(manifest, args.version)
    files = collect_source_files(ROOT, contract)
    archive = out_dir / f"{contract.name}-v{args.version}.zip"
    digest = write_archive(archive, args.version, contract, files)
    print(f"Production payload files: {len(files)}")
    print(f"Built: {archive}")
    print(f"SHA256: {digest}")
    print(f"Checksum: {archive}.sha256")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (OSError, ValueError) as error:
        print(f"Production payload build failed: {error}", file=sys.stderr)
        raise SystemExit(1)
