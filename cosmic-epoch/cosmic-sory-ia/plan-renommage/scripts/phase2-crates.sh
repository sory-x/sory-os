#!/usr/bin/env bash
# Phase 2 — Renommage des crates Rust codex-* → sory-*
# ⚠️ À exécuter depuis la racine de cosmic-sory-ia
# Usage: bash plan-renommage/scripts/phase2-crates.sh

set -euo pipefail

BASE="/home/sory/Bureau/soryos/cosmic-epoch/cosmic-sory-ia"
CODEX_RS="$BASE/sory-ia/codex-rs"
SORY_DESKTOP="$BASE/sory-desktop"

echo "=== Phase 2: Renommage crates codex-* → sory-* ==="
echo ""

# Étape 1: Workspace Cargo.toml (liste des membres + dépendances)
echo "[1/6] Workspace Cargo.toml..."
sed -i 's/codex-/sory-/g' "$CODEX_RS/Cargo.toml"

# Étape 2: Chaque crate Cargo.toml
echo "[2/6] Tous les Cargo.toml des crates..."
find "$CODEX_RS" -name "Cargo.toml" -exec sed -i 's/codex-/sory-/g' {} \;

# Étape 3: Lib name dans le code (use codex_* → use sory_*)
echo "[3/6] Noms de lib dans le code Rust..."
find "$CODEX_RS" -name "*.rs" -exec sed -i 's/\bcodex_\b/sory_/g' {} \;

# Étape 4: BUILD.bazel
echo "[4/6] Fichiers Bazel..."
find "$CODEX_RS" -name "BUILD.bazel" -o -name "*.bzl" | while read f; do
    sed -i 's/codex_/sory_/g' "$f"
    sed -i 's/codex-/sory-/g' "$f"
done

# Étape 5: sory-desktop Cargo.toml
echo "[5/6] sory-desktop Cargo.toml..."
sed -i 's/codex-/sory-/g' "$SORY_DESKTOP/Cargo.toml"

# Étape 6: Autres fichiers (Cargo.lock, configs)
echo "[6/6] Autres fichiers de configuration..."
find "$CODEX_RS" -name "*.toml" -o -name "*.lock" | while read f; do
    sed -i 's/codex-/sory-/g' "$f"
done

echo ""
echo "=== Phase 2 terminée ==="
echo "⚠️  Lancer 'cargo check --workspace' dans codex-rs pour vérifier"
echo "⚠️  Lancer 'cargo check' dans sory-desktop pour vérifier"
echo ""
echo "Si erreur de compilation:"
echo "  rg 'codex_'  codex-rs --type rust  # cherche les noms oubliés"
echo "  rg 'codex-'  codex-rs --type rust  # cherche les noms oubliés"
