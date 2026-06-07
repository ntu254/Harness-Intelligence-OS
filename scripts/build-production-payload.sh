#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Usage: build-production-payload.sh --version <MAJOR.MINOR.PATCH> [options]

Build the deterministic production-clean HI-OS ZIP and SHA256 asset.

Options:
      --version <version>  Required payload version.
      --manifest <path>    Override packaging/production-include.toml.
      --out-dir <path>     Override dist output directory.
  -h, --help               Show this help.
EOF
}

fail() {
  printf 'Error: %s\n' "$*" >&2
  exit 1
}

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
version=""
manifest=""
out_dir=""

while [ "$#" -gt 0 ]; do
  case "$1" in
    --version)
      [ "$#" -ge 2 ] || fail "$1 requires a version"
      version="$2"
      shift 2
      ;;
    --manifest)
      [ "$#" -ge 2 ] || fail "$1 requires a path"
      manifest="$2"
      shift 2
      ;;
    --out-dir)
      [ "$#" -ge 2 ] || fail "$1 requires a path"
      out_dir="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      fail "Unknown option: $1"
      ;;
  esac
done

[ -n "$version" ] || fail "--version is required"

args=("$repo_root/scripts/build-production-payload.py" --version "$version")
[ -z "$manifest" ] || args+=(--manifest "$manifest")
[ -z "$out_dir" ] || args+=(--out-dir "$out_dir")

if command -v python3 >/dev/null 2>&1; then
  python_cmd=(python3)
elif command -v python >/dev/null 2>&1; then
  python_cmd=(python)
elif command -v py.exe >/dev/null 2>&1; then
  python_cmd=(py.exe -3)
else
  fail "Python 3.11 or newer is required"
fi

cd "$repo_root"
"${python_cmd[@]}" "${args[@]}"
