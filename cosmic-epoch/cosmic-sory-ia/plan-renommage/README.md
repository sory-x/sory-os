# Plan de renommage « codex » → Sory IA

> ⚠️ Ce plan documente **tous les changements** de noms possibles, classés par risque.
> Ne pas exécuter sans validation. Chaque phase est conçue pour être réversible.

---

## Table des matières

1. [Zones de noms identifiées](#1-zones-de-noms-identifiées)
2. [Phases de renommage](#2-phases-de-renommage)
3. [Zones EXTERNES à ne pas toucher](#3-zones-externes-à-ne-pas-toucher)
4. [Phase 1 — Surface (sans risque)](#phase-1--surface-sans-risque)
5. [Phase 2 — Crates Rust (risque maîtrisé)](#phase-2--crates-rust-risque-maîtrisé)
6. [Phase 3 — Interface externe (risque élevé)](#phase-3--interface-externe-risque-élevé)
7. [Scripts de migration automatique](#7-scripts-de-migration-automatique)

---

## 1. Zones de noms identifiées

| Zone | Type | Risque | Changer | Dépend de |
|---|---|---|---|---|
| **Crate packages** (Cargo.toml) | Interne | ⚠️ | `codex-*` → `sory-*` | Tous les `Cargo.toml` du workspace |
| **Lib names** (Cargo.toml) | Interne | ⚠️ | `codex_*` → `sory_*` | Tous les `use codex_*` dans le code |
| **Binaire CLI** (`[[bin]] name`) | Externe | 🔴 | `codex` → `sory` | Scripts, doc, users |
| **NPM package** (package.json) | Externe | 🔴 | `@openai/codex` → `@soryos/sory-ia` | Registre npm |
| **Variables d'env** | Externe | 🔴 | `CODEX_HOME` | Runtime, daemon |
| **Chemin données** | Externe | 🔴 | `~/.codex/` | Daemon existant |
| **Commentaires/docstrings** | Interne | 🟢 | Texte libre | Rien |
| **Messages log** | Interne | 🟢 | Chaînes formatées | Rien |
| **Messages TUI** | Interne | 🟢 | Textes affichés | Rien |
| **Noms de variables** | Interne | 🟢 | Identifiants Rust | Compilation |
| **Protocole wire** | Externe | 🔴 | Méthodes JSON-RPC | Compatibilité réseau |
| **URLs GitHub** | Externe | 🔴 | `openai/codex` | Git remote |
| **Nom build Bazel** | Interne | ⚠️ | `codex-*` → `sory-*` | BUILD.bazel files |

---

## 2. Phases de renommage

```
Phase 1 ──► Surface (logs, TUI, commentaires)
  │               🟢 Aucun risque
  ▼
Phase 2 ──► Crates Rust internes + Bazel
  │               ⚠️ Nécessite sync avec sory-desktop
  ▼
Phase 3 ──► Interface externe (CLI, env, npm)
               🔴 Risque élevé, nécessite migration
```

---

## 3. Zones EXTERNES à ne pas toucher

Ces éléments sont soit utilisés par d'autres projets/outils, soit font partie d'un contrat d'interface stable.

### 3.1 Dépendances du sory-desktop

Les 3 crates que `sory-desktop` importe via `path` :

```toml
# sory-desktop/Cargo.toml
codex-app-server-client = { path = "../sory-ia/codex-rs/app-server-client" }
codex-app-server-protocol = { path = "../sory-ia/codex-rs/app-server-protocol" }
codex-utils-absolute-path = { path = "../sory-ia/codex-rs/utils/absolute-path" }
```

**Si on renomme ces 3 crates** → on doit **aussi** mettre à jour les `path` dans `sory-desktop/Cargo.toml`.

**Si on renomme leur `name = "codex-..."`** → on doit **aussi** mettre à jour toutes les crates internes qui les importent par `codex-* = { workspace = true }`.

### 3.2 Protocole wire (JSON-RPC)

Les méthodes comme `thread/start`, `turn/start`, `initialize`, `configRequirements/read` font partie du **protocole sur le fil**. Les renommer casserait la compatibilité avec toute instance du daemon.

➡️ **NE PAS TOUCHER** les `method` dans les messages JSON-RPC.

### 3.3 Variable d'environnement `CODEX_HOME`

Lue par le daemon et par `sory-desktop` (via `codex-utils-home-dir`). Si on la renomme en `SORY_IA_HOME` :

- Il faut garder `CODEX_HOME` comme fallback pour les installations existantes
- Mettre à jour `SORY_IA_HOME` comme variable primaire dans `utils/home-dir`
- Ajouter un warning si `CODEX_HOME` est utilisé sans `SORY_IA_HOME`

### 3.4 URLs GitHub et npm

```json
// codex-cli/package.json
"repository": { "url": "git+https://github.com/openai/codex.git" }
"name": "@openai/codex"
```

Ces URLs pointent vers le dépôt upstream. Si Sory IA a son propre fork GitHub :
- Changer le `repository.url`
- Changer le `name` npm
- MAIS ça ne casse pas la compilation locale

---

## Phase 1 — Surface (sans risque)

🟢 **Aucun impact fonctionnel.** Peut être fait immédiatement.

### 1.1 Commentaires et docstrings

```bash
# Chercher les occurrences dans le code Rust
rg "Codex|codex" codex-rs --type rust -l | wc -l
# → probablement 5000+ fichiers
```

Stratégie : ne remplacer que les commentaires/docstrings, pas les identifiants.
Difficile à automatiser sans casser le code → **à faire manuellement par fichier** ou avec un script très précis.

### 1.2 Messages de log

```bash
# Chercher les messages de log contenant "codex"
rg "log::(info|warn|error|debug)!.*codex" codex-rs -n
```

**Remplaçable sans risque :** `"Codex runtime"` → `"Sory IA runtime"`, `"Codex daemon"` → `"Sory IA daemon"`, etc.

### 1.3 Textes TUI

Les textes affichés dans l'interface terminal (dans `codex-rs/tui/`).

```bash
rg "Codex|codex" codex-rs/tui/src/ --type rust
```

**Remplaçable sans risque :** tout ce qui est affiché à l'utilisateur (titres, messages, prompts).

### Script Phase 1

```bash
#!/usr/bin/env bash
# Phase 1 — Renommage surface (sans risque)
# Ne touche que les commentaires, logs, et textes TUI

set -euo pipefail
BASE="/home/sory/Bureau/soryos/cosmic-epoch/cosmic-sory-ia/sory-ia/codex-rs"

echo "=== Phase 1: Renommage surface codex → Sory IA ==="

# 1. Commentaires et docstrings (uniquement les lignes commentaires)
#    ATTENTION: ne touche PAS les noms de fonctions/variables
find "$BASE" -name "*.rs" -exec sed -i \
  -e 's/\/\/.*Codex/\/\/ Sory IA/g' \
  -e 's/\/\/.*codex/\/\/ sory-ia/g' \
  {} \;

echo "✓ Commentaires Rust mis à jour"
```

⚠️ **Attention :** le sed ci-dessus est trop agressif. Préférer des remplacements ciblés.
Voir le fichier `scripts/phase1-surface.sh` pour une version plus sûre.

---

## Phase 2 — Crates Rust (risque maîtrisé)

⚠️ **Impact :** nécessite de modifier TOUS les `Cargo.toml` du workspace ET le code Rust qui utilise `use codex_*::...`.

### 2.1 Principe

Chaque crate a 2 noms :
1. **Package name** (`[package] name = "codex-core"`) → utilisé dans les dépendances workspace
2. **Lib name** (`[lib] name = "codex_core"`) → utilisé dans les `use codex_core::...` du code

### 2.2 Arbre des dépendances

```
codex-app-server-client ──► codex-app-server-protocol ◄── sory-desktop
          │                       │
          ├── codex-app-server    ├── codex-protocol
          ├── codex-core          ├── codex-shell-command
          ├── codex-config        ├── codex-utils-absolute-path
          ├── codex-exec-server   └── codex-experimental-api-macros
          ├── codex-feedback
          ├── codex-protocol
          ├── codex-uds
          └── codex-utils-absolute-path
```

Le workspace a **~100 crates** interconnectées. Renommer une crate nécessite de mettre à jour TOUTES les crates qui la référencent.

### 2.3 Procédure

#### Étape A — Changer le `[package] name`

Pour chaque crate dans le workspace :
```toml
# Avant
[package]
name = "codex-core"

# Après
[package]
name = "sory-core"
```

#### Étape B — Changer le `[lib] name`

```toml
# Avant
[lib]
name = "codex_core"

# Après
[lib]
name = "sory_core"
```

#### Étape C — Mettre à jour les dépendances workspace

Dans `Cargo.toml` du workspace :
```toml
# Avant
codex-core = { path = "core" }

# Après
sory-core = { path = "core" }
```

#### Étape D — Mettre à jour toutes les dépendances inter-crates

Dans CHAQUE `Cargo.toml` des ~100 crates :
```toml
# Avant
codex-core = { workspace = true }

# Après
sory-core = { workspace = true }
```

#### Étape E — Mettre à jour les `use` dans le code Rust

```rust
// Avant
use codex_core::config::Config;

// Après
use sory_core::config::Config;
```

#### Étape F — Mettre à jour les BUILD.bazel

```python
# Avant
codex_core = "//core:codex_core"

# Après
sory_core = "//core:sory_core"
```

#### Étape G — Mettre à jour sory-desktop

```toml
# Avant
codex-app-server-client = { path = "../sory-ia/codex-rs/app-server-client" }

# Après
sory-app-server-client = { path = "../sory-ia/codex-rs/app-server-client" }
```

### 2.4 Script automatisé Phase 2

```bash
#!/usr/bin/env bash
# Phase 2 — Renommage des crates Rust
# 1. workspace Cargo.toml
# 2. Chaque crate Cargo.toml
# 3. Tous les use dans le code

set -euo pipefail
BASE="/home/sory/Bureau/soryos/cosmic-epoch/cosmic-sory-ia"

# Mise à jour du workspace Cargo.toml
sed -i 's/codex-\(app-server-client\|app-server-protocol\|utils-absolute-path\|[a-z-]*\)/sory-\1/g' \
  "$BASE/sory-ia/codex-rs/Cargo.toml"

# Mise à jour de toutes les dépendances croisées
find "$BASE/sory-ia/codex-rs" -name "Cargo.toml" -exec sed -i \
  's/codex-\([a-z-]*\)/sory-\1/g' {} \;

# Mise à jour des lib names dans le code
find "$BASE/sory-ia/codex-rs" -name "*.rs" -exec sed -i \
  's/use codex_\([a-zA-Z_]*\)/use sory_\1/g' {} \;

# Mise à jour du sory-desktop
sed -i 's/codex-/sory-/g' "$BASE/sory-desktop/Cargo.toml"

echo "✓ Phase 2 terminée — vérifier avec cargo check"
```

---

## Phase 3 — Interface externe (risque élevé)

🔴 **Nécessite coordination.** Impact utilisateurs et outils existants.

### 3.1 Binaire CLI `codex` → `sory-ia`

```toml
# codex-rs/cli/Cargo.toml
[[bin]]
name = "codex"          # → "sory-ia"
```

**Conséquences :**
- Tous les scripts qui appellent `codex ...` ne fonctionneront plus
- La documentation, les tutoriels, les intégrations CI/CD doivent être mis à jour
- Le npm package `@openai/codex` doit être migré

**Recommandation :** créer un alias `sory-ia` en parallèle de `codex`, puis déprécier `codex`.

```bash
# Option 1: Alias temporaire
alias codex='sory-ia'

# Option 2: Binaires jumeaux (hardlink)
ln /usr/local/bin/codex /usr/local/bin/sory-ia
```

### 3.2 Variable d'environnement `CODEX_HOME` → `SORY_IA_HOME`

Dans `utils/home-dir` :

```rust
// 1. Lire SORY_IA_HOME d'abord
if let Some(home) = env::var_os("SORY_IA_HOME") {
    return Some(PathBuf::from(home));
}
// 2. Fallback vers CODEX_HOME avec warning
if let Some(home) = env::var_os("CODEX_HOME") {
    log::warn!("CODEX_HOME est déprécié, utilisez SORY_IA_HOME");
    return Some(PathBuf::from(home));
}
// 3. Fallback vers ~/.codex avec warning
log::warn!("~/.codex est déprécié, définissez SORY_IA_HOME");
home.join(".codex")
```

### 3.3 Chemin des données `~/.codex/` → `~/.sory-ia/`

**Risque :** les installations existantes ont leurs données dans `~/.codex/`.
Migration : créer un lien symbolique `~/.sory-ia/` → `~/.codex/`.

---

## 7. Scripts de migration automatique

Dossier proposé : `sory-ia/scripts/renaming/`

| Script | Phase | Action |
|---|---|---|
| `phase1-surface.sh` | 1 | Logs, TUI, commentaires |
| `phase2-crates.sh` | 2 | Cargo.toml, lib names, use |
| `phase3-external.sh` | 3 | CLI, env vars, paths |
| `verify.sh` | — | Vérifie qu'aucun `codex` ne subsiste |
| `rollback.sh` | — | Annule toutes les modifications |

---

## Règles de vérification

Avant chaque commit de renommage :

```bash
# 1. Vérifier qu'aucun nom cassé ne subsiste
rg "use codex_" codex-rs --type rust

# 2. Vérifier que le workspace compile
cd codex-rs && cargo check --workspace

# 3. Vérifier que sory-desktop compile
cd ../sory-desktop && cargo check

# 4. Vérifier les tests
cd ../sory-ia/codex-rs && cargo test -p codex-cli
```

---

## Arbre de décision

```
Vous voulez renommer ?
        │
        ├── Des commentaires / logs / TUI ?
        │   └──► Phase 1 — Aucun risque
        │
        ├── Des crates Rust internes ?
        │   ├── Sans les 3 crates sory-desktop ?
        │   │   └──► Phase 2 partielle — ⚠️ tester cargo check
        │   └── Avec les 3 crates sory-desktop ?
        │       └──► Phase 2 complète — ⚠️ nécessite sync
        │
        └── De l'interface externe ?
            ├── Binaire CLI ?
            │   └──► Phase 3a — 🔴 impact utilisateur
            ├── Variables d'env / chemins ?
            │   └──► Phase 3b — 🔴 migration nécessaire
            └── URLs / npm ?
                └──► Phase 3c — 🔴 coordination
```
