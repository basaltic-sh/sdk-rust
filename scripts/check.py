#!/usr/bin/env python3
"""Run checks against public runtime code; no generator or API access is needed."""
import os
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
for args in [
    ["cargo", "fmt", "--all", "--", "--check"],
    ["cargo", "clippy", "--locked", "--all-targets", "--", "-D", "warnings"],
    ["cargo", "test", "--locked", "--all-targets"],
    ["cargo", "test", "--locked", "--doc"],
    ["cargo", "doc", "--locked", "--no-deps"],
]:
    subprocess.run(args, cwd=ROOT, check=True, env={**os.environ, "RUSTDOCFLAGS": "-D warnings"})
