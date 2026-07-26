#!/usr/bin/env python3
"""
Phase 2 — Renommage des crates Rust internes sory → sory
NE TOUCHE PAS: dépendances externes, protocole wire, URLs, binaire CLI principal
"""
import os
import re
import sys

BASE = "/home/sory/Bureau/soryos/cosmic-epoch/cosmic-sory-ia/sory-ia/sory-rs"
DESKTOP = "/home/sory/Bureau/soryos/cosmic-epoch/cosmic-sory-ia/sory-desktop"

# List of known external dependencies that should NOT be renamed
# (these don't start with sory- but just in case)
EXTERNAL_DEPS = set()

def rename_in_file(filepath, dry_run=False):
    """Rename sory-internal references in a single file."""
    try:
        with open(filepath, 'r') as f:
            content = f.read()
    except (UnicodeDecodeError, PermissionError):
        return False

    original = content

    if filepath.endswith('.toml'):
        # Handle Cargo.toml files
        lines = content.split('\n')
        new_lines = []
        in_deps_section = False
        in_members = False
        in_internal_deps = False

        for line in lines:
            stripped = line.strip()

            # Track sections
            if stripped == '[workspace]':
                in_members = True
                in_deps_section = False
                in_internal_deps = False
            elif stripped == '[workspace.dependencies]':
                in_members = False
                in_deps_section = True
                in_internal_deps = True
            elif stripped.startswith('[[') or (stripped.startswith('[') and not stripped.startswith('#')):
                in_members = False
                if in_deps_section and stripped.startswith('[') and 'Internal' not in stripped and 'External' not in stripped:
                    in_internal_deps = False
                if '# External' in stripped:
                    in_internal_deps = False
                if '# Internal' in stripped:
                    in_internal_deps = True

            # Track when we hit external deps marker
            if stripped == '# External':
                in_internal_deps = False

            # Workspace members: "sory-xxx" → "sory-xxx"
            if in_members and '"sory-' in stripped:
                line = re.sub(r'"sory-([a-z-]+)"', r'"sory-\1"', line)

            # Internal workspace dependencies: sory-xxx = { path = ... } → sory-xxx
            if in_internal_deps and re.match(r'sory-[a-z-]+ = \{', stripped):
                line = re.sub(r'^sory-([a-z-]+)', r'sory-\1', line)

            # Package name: name = "sory-xxx" → "sory-xxx"
            if re.match(r'^name = "sory-[a-z-]+"$', stripped):
                line = re.sub(r'^name = "sory-([a-z-]+)"', r'name = "sory-\1"', line)

            # Lib name: name = "sory_xxx" → "sory_xxx"
            if re.match(r'^name = "sory_[a-z_]+"$', stripped):
                line = re.sub(r'^name = "sory_([a-z_]+)"', r'name = "sory_\1"', line)

            # Bin name: name = "sory-execve-wrapper" → "sory-execve-wrapper"
            # But NOT name = "sory" (main CLI binary, Phase 3)
            if re.match(r'^name = "sory-[a-z-]+"$', stripped):
                line = re.sub(r'^name = "sory-([a-z-]+)"', r'name = "sory-\1"', line)

            # Dependencies in [dependencies], [dev-dependencies], [build-dependencies]
            # sory-xxx = { workspace = true } → sory-xxx = { workspace = true }
            if re.match(r'sory-[a-z-]+ = \{ workspace', stripped):
                line = re.sub(r'^sory-([a-z-]+)', r'sory-\1', line)

            # sory-xxx = { path = "..." } → sory-xxx = { path = "..." }
            if re.match(r'sory-[a-z-]+ = \{ path', stripped):
                line = re.sub(r'^sory-([a-z-]+)', r'sory-\1', line)

            # sory_xxx = { package = "sory-yyy", ... } → sory_xxx = { package = "sory-yyy", ... }
            if re.match(r'sory_[a-z_]+ = \{ package = "sory-', stripped):
                line = re.sub(r'^sory_([a-z_]+)', r'sory_\1', line)
                line = re.sub(r'package = "sory-([a-z-]+)"', r'package = "sory-\1"', line)

            # Feature references: "sory-xxx" → "sory-xxx"
            if '"sory-' in stripped:
                line = re.sub(r'"sory-([a-z-]+)"', r'"sory-\1"', line)

            # cargo-shear ignored
            if '"sory-' in stripped and 'ignored' in stripped:
                line = re.sub(r'"sory-([a-z-]+)"', r'"sory-\1"', line)

            new_lines.append(line)

        content = '\n'.join(new_lines)

        # Also handle the url reference in comment (keep it, it's external)
        # Don't touch: "https://github.com/openai/sory..."

    elif filepath.endswith('.rs'):
        # Rust source files
        # Replace use sory_xxx:: with use sory_xxx::
        content = re.sub(r'\buse sory_([a-zA-Z_]+)', r'use sory_\1', content)

        # Replace qualified paths: sory_xxx::yyy → sory_xxx::yyy
        content = re.sub(r'\bsory_([a-zA-Z_]+)::', r'sory_\1::', content)

        # Replace extern crate sory_xxx
        content = re.sub(r'\bextern crate sory_([a-zA-Z_]+)', r'extern crate sory_\1', content)

    elif filepath.endswith('BUILD.bazel'):
        content = re.sub(r'sory_([a-zA-Z_]+)', r'sory_\1', content)
        content = re.sub(r'sory-([a-z-]+)', r'sory-\1', content)

    if content != original:
        if not dry_run:
            with open(filepath, 'w') as f:
                f.write(content)
        return True
    return False


def main():
    dry_run = '--dry-run' in sys.argv
    print(f"=== Phase 2: Renommage crates Rust sory → sory {'(DRY RUN)' if dry_run else ''} ===")

    # 1. Workspace Cargo.toml
    ws_toml = os.path.join(BASE, 'Cargo.toml')
    print(f"→ Workspace: {ws_toml}")
    rename_in_file(ws_toml, dry_run)

    # 2. All crate Cargo.toml files
    count = 0
    for root, dirs, files in os.walk(BASE):
        # Skip target directory
        if 'target' in dirs:
            dirs.remove('target')
        for fname in files:
            if fname == 'Cargo.toml' and root != BASE:
                filepath = os.path.join(root, fname)
                if rename_in_file(filepath, dry_run):
                    count += 1
    print(f"→ {count} crate Cargo.toml files updated")

    # 3. All .rs files
    count = 0
    for root, dirs, files in os.walk(BASE):
        if 'target' in dirs:
            dirs.remove('target')
        for fname in files:
            if fname.endswith('.rs'):
                filepath = os.path.join(root, fname)
                if rename_in_file(filepath, dry_run):
                    count += 1
    print(f"→ {count} .rs files updated")

    # 4. BUILD.bazel files
    count = 0
    for root, dirs, files in os.walk(BASE):
        if 'target' in dirs:
            dirs.remove('target')
        for fname in files:
            if fname == 'BUILD.bazel':
                filepath = os.path.join(root, fname)
                if rename_in_file(filepath, dry_run):
                    count += 1
    print(f"→ {count} BUILD.bazel files updated")

    # 5. sory-desktop
    print(f"→ sory-desktop: {DESKTOP}")
    desktop_toml = os.path.join(DESKTOP, 'Cargo.toml')
    rename_in_file(desktop_toml, dry_run)

    for root, dirs, files in os.walk(DESKTOP):
        if 'target' in dirs:
            dirs.remove('target')
        for fname in files:
            if fname.endswith('.rs'):
                filepath = os.path.join(root, fname)
                rename_in_file(filepath, dry_run)

    print("")
    print("=== Phase 2 terminée ===")
    if not dry_run:
        print("Vérifications à faire:")
        print(f"  1. cd {BASE} && cargo check --workspace")
        print(f"  2. cd {DESKTOP} && cargo check")


if __name__ == '__main__':
    main()
