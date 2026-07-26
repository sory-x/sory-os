from __future__ import annotations

import os
from pathlib import Path

PACKAGE_NAME = "openai-sory-cli-bin"


def bundled_sory_path() -> Path:
    exe = "sory.exe" if os.name == "nt" else "sory"
    path = Path(__file__).resolve().parent / "bin" / exe
    if not path.is_file():
        raise FileNotFoundError(
            f"{PACKAGE_NAME} is installed but missing its packaged sory binary at {path}"
        )
    return path


__all__ = ["PACKAGE_NAME", "bundled_sory_path"]
