#!/usr/bin/env python3
"""Shared production payload contract and archive helpers."""

from __future__ import annotations

from dataclasses import dataclass
from fnmatch import fnmatchcase
from hashlib import sha256
import json
from pathlib import Path, PurePosixPath
import re
import tomllib
from typing import Iterable
from zipfile import ZIP_DEFLATED, ZipFile, ZipInfo


INTERNAL_MANIFEST = "hios-production-manifest.json"
ZIP_TIMESTAMP = (1980, 1, 1, 0, 0, 0)
VERSION_PATTERN = re.compile(r"^[0-9]+\.[0-9]+\.[0-9]+$")


@dataclass(frozen=True)
class ProductionContract:
    name: str
    archive_root: str
    include: tuple[str, ...]
    exclude: tuple[str, ...]
    required: tuple[str, ...]
    forbidden: tuple[str, ...]


def validate_version(version: str) -> str:
    if not VERSION_PATTERN.fullmatch(version):
        raise ValueError("version must use MAJOR.MINOR.PATCH, for example 0.7.0")
    return version


def load_contract(path: Path, version: str) -> ProductionContract:
    validate_version(version)
    with path.open("rb") as handle:
        document = tomllib.load(handle)
    payload = document.get("payload")
    if not isinstance(payload, dict):
        raise ValueError(f"{path}: missing [payload] table")

    def string_value(name: str) -> str:
        value = payload.get(name)
        if not isinstance(value, str) or not value.strip():
            raise ValueError(f"{path}: payload.{name} must be a non-empty string")
        return value

    def string_list(name: str) -> tuple[str, ...]:
        value = payload.get(name)
        if not isinstance(value, list) or not value:
            raise ValueError(f"{path}: payload.{name} must be a non-empty string list")
        if any(not isinstance(item, str) or not item.strip() for item in value):
            raise ValueError(f"{path}: payload.{name} contains an invalid pattern")
        return tuple(value)

    archive_root = string_value("archive_root").format(version=version)
    validate_relative_path(archive_root, "archive_root")
    return ProductionContract(
        name=string_value("name"),
        archive_root=archive_root,
        include=string_list("include"),
        exclude=string_list("exclude"),
        required=string_list("required"),
        forbidden=string_list("forbidden"),
    )


def validate_relative_path(value: str, label: str) -> None:
    path = PurePosixPath(value)
    if path.is_absolute() or ".." in path.parts or "\\" in value:
        raise ValueError(f"{label} must be a safe relative POSIX path: {value}")


def matches_any(path: str, patterns: Iterable[str]) -> bool:
    return any(fnmatchcase(path, pattern) for pattern in patterns)


def collect_source_files(repo_root: Path, contract: ProductionContract) -> dict[str, bytes]:
    files: dict[str, bytes] = {}
    resolved_root = repo_root.resolve()

    for pattern in contract.include:
        validate_relative_path(pattern, "include pattern")
        matches = sorted(repo_root.glob(pattern))
        matched_files: list[Path] = []
        for path in matches:
            if path.is_file():
                matched_files.append(path)
            elif path.is_dir():
                matched_files.extend(
                    child for child in sorted(path.rglob("*")) if child.is_file()
                )
        if not matched_files:
            raise ValueError(f"include pattern matched no files: {pattern}")
        for path in matched_files:
            if path.is_symlink():
                raise ValueError(f"production payload does not accept symlinks: {path}")
            resolved = path.resolve()
            if not resolved.is_relative_to(resolved_root):
                raise ValueError(f"included file escapes repository root: {path}")
            relative = path.relative_to(repo_root).as_posix()
            validate_relative_path(relative, "included file")
            if matches_any(relative, contract.exclude):
                continue
            files[relative] = path.read_bytes()

    missing = sorted(set(contract.required) - set(files))
    if missing:
        raise ValueError(f"required production files are missing: {', '.join(missing)}")

    forbidden = sorted(
        path for path in files if matches_any(path, contract.forbidden)
    )
    if forbidden:
        raise ValueError(
            f"forbidden files entered production payload: {', '.join(forbidden)}"
        )

    return dict(sorted(files.items()))


def internal_manifest_bytes(version: str, files: dict[str, bytes]) -> bytes:
    manifest = {
        "schema_version": "1.0.0",
        "artifact_type": "hios-production-payload",
        "version": version,
        "file_count": len(files),
        "files": [
            {
                "path": path,
                "sha256": sha256(content).hexdigest(),
            }
            for path, content in files.items()
        ],
    }
    return (
        json.dumps(manifest, indent=2, sort_keys=True, ensure_ascii=True) + "\n"
    ).encode("utf-8")


def zip_info(name: str, executable: bool = False) -> ZipInfo:
    info = ZipInfo(name, ZIP_TIMESTAMP)
    info.compress_type = ZIP_DEFLATED
    info.create_system = 3
    mode = 0o755 if executable else 0o644
    info.external_attr = (mode & 0xFFFF) << 16
    info.flag_bits |= 0x800
    return info


def write_archive(
    archive: Path,
    version: str,
    contract: ProductionContract,
    files: dict[str, bytes],
) -> str:
    archive.parent.mkdir(parents=True, exist_ok=True)
    temporary = archive.with_suffix(f"{archive.suffix}.tmp")
    if temporary.exists():
        temporary.unlink()

    with ZipFile(temporary, "w", compression=ZIP_DEFLATED, compresslevel=9) as output:
        manifest_path = f"{contract.archive_root}/{INTERNAL_MANIFEST}"
        output.writestr(
            zip_info(manifest_path),
            internal_manifest_bytes(version, files),
            compress_type=ZIP_DEFLATED,
            compresslevel=9,
        )
        for relative, content in files.items():
            archive_path = f"{contract.archive_root}/{relative}"
            output.writestr(
                zip_info(archive_path, executable=relative.endswith(".sh")),
                content,
                compress_type=ZIP_DEFLATED,
                compresslevel=9,
            )

    temporary.replace(archive)
    digest = sha256(archive.read_bytes()).hexdigest()
    checksum = archive.with_name(f"{archive.name}.sha256")
    checksum.write_text(f"{digest}  {archive.name}\n", encoding="ascii", newline="\n")
    return digest


def parse_checksum(path: Path) -> tuple[str, str]:
    parts = path.read_text(encoding="ascii").strip().split()
    if len(parts) != 2 or not re.fullmatch(r"[0-9a-fA-F]{64}", parts[0]):
        raise ValueError(f"invalid SHA256 file: {path}")
    return parts[0].lower(), parts[1].lstrip("*")


def safe_archive_relative(name: str, archive_root: str) -> str:
    validate_relative_path(name, "archive entry")
    prefix = f"{archive_root}/"
    if not name.startswith(prefix):
        raise ValueError(f"archive entry is outside expected root '{archive_root}': {name}")
    relative = name[len(prefix) :]
    validate_relative_path(relative, "archive entry")
    return relative
