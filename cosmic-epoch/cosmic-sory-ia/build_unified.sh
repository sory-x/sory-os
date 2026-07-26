#!/bin/bash
# Script de build unifié pour cosmic-sory-ia
# Utilise le workspace Cargo unifié : UN seul cargo build, UN seul target/ partagé
# entre sory-rs (moteur IA), sory-desktop (UI) et libcosmic.
#
# Temps estimé : 2-3h selon votre machine
# Espace disque : ~15-20 Go (au lieu de 30-40 Go avec 2 target/ séparés)
set -e

echo "🚀 Build unifié de cosmic-sory-ia"
echo "================================"
echo "  • UI            : sory-desktop (COSMIC + libcosmic)"
echo "  • Moteur IA     : sory-rs (path deps, workspace interne)"
echo "  • Target unique : target/"
echo

# Vérifier que nous sommes dans le bon dossier
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR"

if [ ! -f "Cargo.toml" ] || ! grep -q 'name = "cosmic-sory-ia"' Cargo.toml 2>/dev/null; then
    # Fallback: vérifier la présence du workspace
    if ! grep -q '\[workspace\]' Cargo.toml 2>/dev/null; then
        echo "❌ Erreur : ce script doit être lancé depuis cosmic-sory-ia/"
        echo "   (racine contenant le Cargo.toml workspace unifié)"
        exit 1
    fi
fi

# Configuration réseau Cargo (utile pour les gros builds)
export CARGO_NET_RETRY=10
export CARGO_NET_TIMEOUT=60
export CARGO_INCREMENTAL=1

# Timing
SECONDS=0

# Préparation des répertoires
RELEASE_DIR="target/release"
mkdir -p "$RELEASE_DIR"

# ──────────────────────────────────────────────
#  Build unique de TOUT le workspace
# ──────────────────────────────────────────────
# Les options de release sont définies dans Cargo.toml [profile.release] :
#   codegen-units = 16, lto = "thin", strip = true
#   (thin LTO évite les OOM avec 3.7 Go de RAM sur 115 crates)
echo "🔨 cargo build --release --workspace"
echo "   (TOUS les membres : sory-rs + sory-desktop + libcosmic, dans le même target/)"
echo

cargo build --release --workspace

echo
echo "✅ Build terminé en $(($SECONDS / 60)) min $(($SECONDS % 60)) sec"
echo

# ──────────────────────────────────────────────
#  Copie des artefacts
# ──────────────────────────────────────────────
echo "📦 Copie des binaires vers $RELEASE_DIR/"

COPIED=0

# Vérification des binaires (Cargo les met directement dans target/release/)
echo "  📍 target/release/ (workspace racine)"

if [ -f "$RELEASE_DIR/sory" ]; then
    echo "  ✅ sory-cli"
    COPIED=$((COPIED + 1))
else
    echo "  ⚠️  sory-cli non généré"
fi

if [ -f "$RELEASE_DIR/sory-desktop" ]; then
    echo "  ✅ sory-desktop"
    COPIED=$((COPIED + 1))
else
    echo "  ⚠️  sory-desktop non généré"
fi

if [ "$COPIED" -eq 0 ]; then
    echo "❌ Aucun binaire trouvé. Vérifiez les chemins ci-dessus."
    exit 1
fi

# ──────────────────────────────────────────────
#  Script de lancement
# ──────────────────────────────────────────────
echo
echo "📝 Création du script de lancement..."

cat > "$RELEASE_DIR/run_sory.sh" << 'RUNEOF'
#!/bin/bash
# Script de lancement pour Sory IA Desktop

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

# Configurez SORY_IA_RUNTIME_COMMAND si vous avez le binaire CLI sory :
# export SORY_IA_RUNTIME_COMMAND="/chemin/vers/sory"

"$SCRIPT_DIR/sory-desktop"
RUNEOF
chmod +x "$RELEASE_DIR/run_sory.sh"

echo "  ✅ $RELEASE_DIR/run_sory.sh"

# ──────────────────────────────────────────────
#  Résumé final
# ──────────────────────────────────────────────
TOTAL_TIME=$SECONDS
echo
echo "═══════════════════════════════════════════"
echo "✨ Build terminé avec succès !"
echo "═══════════════════════════════════════════"
echo "  ⏱️  Temps total        : $(($TOTAL_TIME / 60)) min $(($TOTAL_TIME % 60)) sec"
echo "  📂 Target unique      : target/"
echo "  📦 Binaires copiés    : $COPIED"
echo ""
echo "🚀 Lancer l'interface graphique :"
echo "   ./target/release/run_sory.sh"
echo ""
echo "💡 Lancer le CLI sory (moteur IA) :"
echo "   ./target/release/sory"
echo ""
echo "🔧 Rebuild rapide :"
echo "   cargo build -p sory-desktop"
echo "   cargo build -p sory-cli"
echo ""
echo "📋 Voir les binaires :"
echo "   ls -lh target/release/"
echo "═══════════════════════════════════════════"
