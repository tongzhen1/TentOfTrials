#!/usr/bin/env python3
# LEGACY: This file references legacy behavior and is tracked by legacy_caps_audit.py.
"""Verify files mentioning legacy also include LEGACY in a comment."""

from __future__ import annotations

import argparse
from pathlib import Path


SKIP_DIRS = {
    ".git",
    ".github",
    "__pycache__",
    "node_modules",
    "target",
    "dist",
    "build",
}
TEXT_SUFFIXES = {
    ".c",
    ".css",
    ".go",
    ".h",
    ".hpp",
    ".hs",
    ".js",
    ".json",
    ".lua",
    ".md",
    ".py",
    ".rs",
    ".sql",
    ".toml",
    ".tsx",
    ".ts",
    ".yaml",
    ".yml",
}
COMMENT_MARKERS = ("#", "//", "/*", "*", "--", "<!--")


def iter_files(root: Path) -> list[Path]:
    return [
        path
        for path in root.rglob("*")
        if path.is_file()
        and path.suffix.lower() in TEXT_SUFFIXES
        and not any(part in SKIP_DIRS for part in path.parts)
    ]


def read_text(path: Path) -> str:
    return path.read_text(encoding="utf-8", errors="ignore")


def has_legacy_comment(text: str) -> bool:
    for line in text.splitlines():
        stripped = line.lstrip()
        if "LEGACY" in stripped and stripped.startswith(COMMENT_MARKERS):
            return True
    return False


def find_violations(root: Path) -> list[Path]:
    violations: list[Path] = []

    for path in iter_files(root):
        text = read_text(path)
        if "legacy" in text.lower() and not has_legacy_comment(text):
            violations.append(path)

    return violations


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "root",
        nargs="?",
        default=Path(__file__).resolve().parents[1],
        type=Path,
        help="Repository root to audit. Defaults to this script's repository.",
    )
    args = parser.parse_args()

    root = args.root.resolve()
    violations = find_violations(root)
    if violations:
        print("Files mentioning legacy without a LEGACY comment:")
        for path in violations:
            print(f"- {path.relative_to(root)}")
        return 1

    print("All files mentioning legacy include LEGACY in a comment.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
