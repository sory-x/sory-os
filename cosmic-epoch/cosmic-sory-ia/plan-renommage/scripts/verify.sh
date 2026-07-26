#!/usr/bin/env bash
# Vérifie les résidus "codex" après renommage
# Usage: bash plan-renommage/scripts/verify.sh

set -euo pipefail

BASE="/home/sory/Bureau/soryos/cosmic-epoch/cosmic-sory-ia"
CODEX_RS="$BASE/sory-ia/codex-rs"

echo "=== Vérification des résidus 'codex' ==="
echo ""

# Types de résidus à chercher
echo "--- Résidus dans les noms de crates (Cargo.toml name) ---"
rg "^name = \"codex" "$CODEX_RS" --include "Cargo.toml" -l || echo "✓ Aucun"

echo ""
echo "--- Résidus dans les lib names ---"
rg "^name = \"codex_" "$CODEX_RS" --include "Cargo.toml" -l || echo "✓ Aucun"

echo ""
echo "--- Résidus dans les dépendances (workspace = true) ---"
rg "codex-.*=.*workspace" "$CODEX_RS" --include "Cargo.toml" -l || echo "✓ Aucun"

echo ""
echo "--- Résidus dans les 'use codex_' ---"
rg "use codex_" "$CODEX_RS" --type rust -l || echo "✓ Aucun"

echo ""
echo "--- Résidus dans les BUILD.bazel ---"
rg "codex" "$CODEX_RS" --glob "BUILD.bazel" -l || echo "✓ Aucun"

echo ""
echo "=== Vérification terminée ==="
