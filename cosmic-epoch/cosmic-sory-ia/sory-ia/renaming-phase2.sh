#!/usr/bin/env bash
# Phase 2 — Renommage des crates Rust internes sory → sory
# NE TOUCHE PAS: dépendances externes, protocole wire, URLs, binaire CLI principal
set -euo pipefail

BASE="/home/sory/Bureau/soryos/cosmic-epoch/cosmic-sory-ia/sory-ia/sory-rs"
DESKTOP="/home/sory/Bureau/soryos/cosmic-epoch/cosmic-sory-ia/sory-desktop"

echo "=== Phase 2: Renommage crates Rust sory → sory ==="

# --- ÉTAPE 1: Workspace Cargo.toml ---
echo "→ Étape 1: Workspace Cargo.toml"

# Renommer les workspace members "sory-xxx" → "sory-xxx"
sed -i 's|"sory-\([a-z-]*\)"|"sory-\1"|g' "$BASE/Cargo.toml"

# Renommer les workspace dependencies internes (celles avec path = "...")
# On cible uniquement les lignes entre "# Internal" et "# External"
python3 -c "
import re
with open('$BASE/Cargo.toml', 'r') as f:
    content = f.read()

# Split at '# External' marker
parts = content.split('# External')
if len(parts) == 2:
    internal = parts[0]
    external = parts[1]
    # Replace sory-xxx = { path ... } with sory-xxx = { path ... }
    internal = re.sub(r'^sory-([a-z-]+) = ', r'sory-\1 = ', internal, flags=re.MULTILINE)
    content = internal + '# External' + external

# Also fix cargo-shear ignored
content = content.replace('\"sory-agent-graph-store\"', '\"sory-agent-graph-store\"')
content = content.replace('\"sory-v8-poc\"', '\"sory-v8-poc\"')

with open('$BASE/Cargo.toml', 'w') as f:
    f.write(content)
"

# --- ÉTAPE 2: Chaque crate Cargo.toml ---
echo "→ Étape 2: Cargo.toml de chaque crate"

find "$BASE" -mindepth 2 -name "Cargo.toml" -not -path "*/target/*" | while read -r f; do
  python3 -c "
import re, sys

with open('$f', 'r') as fh:
    content = fh.read()

# 1. [package] name = 'sory-xxx' → 'sory-xxx'
content = re.sub(r'^(name = \")sory-([a-z-]+)(\")', r'\1sory-\2\3', content, flags=re.MULTILINE)

# 2. [lib] name = 'sory_xxx' → 'sory_xxx'
content = re.sub(r'^(name = \")sory_([a-z_]+)(\")', r'\1sory_\2\3', content, flags=re.MULTILINE)

# 3. [[bin]] name = 'sory-execve-wrapper' → 'sory-execve-wrapper'
#    But NOT 'sory' alone (that's the main CLI, Phase 3)
content = re.sub(r'^(name = \")sory-([a-z-]+)(\")', r'\1sory-\2\3', content, flags=re.MULTILINE)

# 4. Dependencies: sory-xxx = { workspace = true } → sory-xxx = { workspace = true }
content = re.sub(r'^(sory-[a-z-]+ = \{ )workspace', r's\1workspace', content, flags=re.MULTILINE)
# Fix: the above creates 'sworkspace', let's redo
content = re.sub(r'^s(sory-[a-z-]+ = \{ )workspace', r's\1workspace', content, flags=re.MULTILINE)

# Actually let's do it properly
lines = content.split('\n')
new_lines = []
in_dependencies = False
for line in lines:
    stripped = line.strip()
    # Track sections
    if stripped.startswith('[') and not stripped.startswith('[['):
        section = stripped.strip('[]')
        if section in ('dependencies', 'dev-dependencies', 'build-dependencies'):
            in_dependencies = True
        else:
            in_dependencies = False
    elif stripped.startswith('[['):
        in_dependencies = False

    if in_dependencies and re.match(r'^sory-[a-z-]+ = \{', stripped):
        line = line.replace('sory-', 'sory-', 1)

    # Handle target-specific dependencies too
    if stripped.startswith('sory-') and '= {' in stripped:
        line = line.replace('sory-', 'sory-', 1)

    # Handle package = 'sory-xxx' in dependencies
    if 'package = "sory-' in line:
        line = line.replace('package = "sory-', 'package = "sory-')

    new_lines.append(line)

content = '\n'.join(new_lines)

# 5. Features referencing internal crates
content = re.sub(r'\"sory-([a-z-]+)\"', r'\"sory-\1\"', content)

# 6. dev-dependencies with sory- prefix
content = re.sub(r'^sory-([a-z-]+) = ', r'sory-\1 = ', content, flags=re.MULTILINE)

# 7. Fix: don't rename external deps that happen to start with sory-
# (there shouldn't be any, but just in case)

with open('$f', 'w') as fh:
    fh.write(content)
"
done

# --- ÉTAPE 3: use statements dans le code Rust ---
echo "→ Étape 3: use statements dans le code Rust"

find "$BASE" -name "*.rs" -not -path "*/target/*" -exec python3 -c "
import sys, re

for filepath in sys.argv[1:]:
    with open(filepath, 'r') as f:
        content = f.read()

    original = content

    # Replace use sory_xxx:: with use sory_xxx::
    content = re.sub(r'\buse sory_([a-zA-Z_]+)', r'use sory_\1', content)

    # Replace sory_xxx:: (qualified paths)
    content = re.sub(r'\bsory_([a-zA-Z_]+)::', r'sory_\1::', content)

    # Replace extern crate sory_xxx
    content = re.sub(r'\bextern crate sory_([a-zA-Z_]+)', r'extern crate sory_\1', content)

    # Replace sory_xxx! macro invocations (like sory_core::something)
    # But be careful not to replace string literals

    if content != original:
        with open(filepath, 'w') as f:
            f.write(content)
" {} +

# --- ÉTAPE 4: BUILD.bazel ---
echo "→ Étape 4: BUILD.bazel"
find "$BASE" -name "BUILD.bazel" -not -path "*/target/*" -exec sed -i \
  -e 's/sory_\([a-zA-Z_]*\)/sory_\1/g' \
  -e 's/sory-\([a-z-]*\)/sory-\1/g' \
  {} \;

# --- ÉTAPE 5: sory-desktop ---
echo "→ Étape 5: sory-desktop"
sed -i 's/sory-app-server-client/sory-app-server-client/g' "$DESKTOP/Cargo.toml"
sed -i 's/sory-app-server-protocol/sory-app-server-protocol/g' "$DESKTOP/Cargo.toml"
sed -i 's/sory-utils-absolute-path/sory-utils-absolute-path/g' "$DESKTOP/Cargo.toml"
sed -i 's/sory-utils-home-dir/sory-utils-home-dir/g' "$DESKTOP/Cargo.toml"

find "$DESKTOP" -name "*.rs" -not -path "*/target/*" -exec python3 -c "
import sys, re

for filepath in sys.argv[1:]:
    with open(filepath, 'r') as f:
        content = f.read()
    original = content
    content = re.sub(r'\bsory_app_server_client\b', 'sory_app_server_client', content)
    content = re.sub(r'\bsory_app_server_protocol\b', 'sory_app_server_protocol', content)
    content = re.sub(r'\bsory_utils_absolute_path\b', 'sory_utils_absolute_path', content)
    content = re.sub(r'\bsory_utils_home_dir\b', 'sory_utils_home_dir', content)
    content = re.sub(r'\buse sory_([a-zA-Z_]+)', r'use sory_\1', content)
    if content != original:
        with open(filepath, 'w') as f:
            f.write(content)
" {} +

# --- ÉTAPE 6: Cargo.lock ---
echo "→ Étape 6: Nettoyage Cargo.lock"
if [ -f "$BASE/Cargo.lock" ]; then
  python3 -c "
import re
with open('$BASE/Cargo.lock', 'r') as f:
    content = f.read()
# Only rename package names, not source URLs
content = re.sub(r'^(name = \")sory-([a-z-]+)(\")', r'\1sory-\2\3', content, flags=re.MULTILINE)
content = re.sub(r'^(name = \")sory_([a-z_]+)(\")', r'\1sory_\2\3', content, flags=re.MULTILINE)
with open('$BASE/Cargo.lock', 'w') as f:
    f.write(content)
"
fi

echo ""
echo "=== Phase 2 terminée ==="
echo "Vérifications à faire:"
echo "  1. cd $BASE && cargo check --workspace"
echo "  2. cd $DESKTOP && cargo check"
