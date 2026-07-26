#!/bin/bash

# Script de remplacement automatique de "codex" par "sory" dans les fichiers source
# Pour sory-ia et sory-desktop

set -e

echo "Début du remplacement de 'codex' par 'sory'..."

# Variables
PROJECT_ROOT="/home/sory/Bureau/soryos/cosmic-epoch/cosmic-sory-ia"
SORY_IA_DIR="$PROJECT_ROOT/sory-ia"
SORY_DESKTOP_DIR="$PROJECT_ROOT/sory-desktop"

# Fonction pour remplacer dans les fichiers Rust
replace_in_rust_files() {
    local dir="$1"
    echo "Traitement des fichiers Rust dans $dir..."
    
    find "$dir" -name "*.rs" -type f -exec sed -i 's/codex/sory/gI' {} \;
    find "$dir" -name "*.toml" -type f -exec sed -i 's/codex/sory/gI' {} \;
}

# Fonction pour remplacer dans les fichiers TypeScript/JavaScript
replace_in_js_files() {
    local dir="$1"
    echo "Traitement des fichiers JS/TS dans $dir..."
    
    find "$dir" -name "*.ts" -type f -exec sed -i 's/codex/sory/gI' {} \;
    find "$dir" -name "*.js" -type f -exec sed -i 's/codex/sory/gI' {} \;
    find "$dir" -name "*.json" -type f -exec sed -i 's/codex/sory/gI' {} \;
}

# Fonction pour remplacer dans les fichiers Python
replace_in_python_files() {
    local dir="$1"
    echo "Traitement des fichiers Python dans $dir..."
    
    find "$dir" -name "*.py" -type f -exec sed -i 's/codex/sory/gI' {} \;
}

# Fonction pour remplacer dans les fichiers Markdown
replace_in_md_files() {
    local dir="$1"
    echo "Traitement des fichiers Markdown dans $dir..."
    
    find "$dir" -name "*.md" -type f -exec sed -i 's/codex/sory/gI' {} \;
}

# Fonction pour remplacer dans les fichiers YAML
replace_in_yaml_files() {
    local dir="$1"
    echo "Traitement des fichiers YAML dans $dir..."
    
    find "$dir" -name "*.yaml" -type f -exec sed -i 's/codex/sory/gI' {} \;
    find "$dir" -name "*.yml" -type f -exec sed -i 's/codex/sory/gI' {} \;
}

# Traitement de sory-ia
echo "=== Traitement de sory-ia ==="
replace_in_rust_files "$SORY_IA_DIR/sory-rs"
replace_in_js_files "$SORY_IA_DIR/sdk/typescript"
replace_in_python_files "$SORY_IA_DIR/sdk/python"
replace_in_md_files "$SORY_IA_DIR"
replace_in_yaml_files "$SORY_IA_DIR"

# Traitement de sory-desktop
echo "=== Traitement de sory-desktop ==="
replace_in_rust_files "$SORY_DESKTOP_DIR"
replace_in_md_files "$SORY_DESKTOP_DIR"

# Remplacements spécifiques pour les crates
echo "=== Remplacements spécifiques ==="

# Remplacer les noms de crates dans Cargo.toml
find "$SORY_IA_DIR/sory-rs" -name "Cargo.toml" -type f -exec sed -i 's/codex-/sory-/gI' {} \;

# Remplacer les noms de modules dans les fichiers Rust
find "$SORY_IA_DIR/sory-rs" -name "*.rs" -type f -exec sed -i 's/codex_/sory_/gI' {} \;

echo "=== Remplacement terminé ==="
echo "Veuillez vérifier les changements avec git diff"