# SoryOS — Mémoire de session (agent)

> **Fichier de continuité entre sessions Cursor.**  
> Toute session (actuelle ou future) doit **lire ce fichier en premier** au début d’un travail sur ce dépôt, puis **le mettre à jour en fin de tâche** (voir section « Protocole agent »).

Dernière mise à jour : **2026-07-29** (fix cosmic-comp / cosmic-files / greeter / initial-setup)

---

## Protocole agent (OBLIGATOIRE)

1. **Au début** : lire ce fichier en entier avant d’agir sur le dépôt.
2. **Pendant** : noter mentalement objectif, blocages, décisions.
3. **À la fin de chaque tâche** (fix, commit, push, diagnostic CI, etc.) :
   - mettre à jour **« État actuel »**, **« Derniers commits »**, **« CI »**, **« Prochaines étapes »** ;
   - mettre à jour **« Commandes »** si une nouvelle commande utile apparaît ;
   - ajouter une ligne dans **« Journal »** (date + résumé) ;
   - conserver ce protocole tel quel (ne pas le supprimer).
4. **Ne pas** dupliquer tout l’historique : garder ~15 entrées de journal max ; résumer l’ancien en une phrase si besoin.
5. **Build local** : machine dev souvent à **3,7 Go RAM** et disque **>95 %** — éviter `cargo build` complet ; préférer CI GitHub ou `cargo check -j1` ciblé.

---

## Commandes (référence complète)

> Racine dépôt : `cd /home/sory/Bureau/soryos.bak`  
> APT : `cd /home/sory/Bureau/soryos.bak/soryos-apt`  
> COSMIC : `cd /home/sory/Bureau/soryos.bak/cosmic-epoch`  
> Doc détaillée APT : `soryos-apt/docs/COMMANDS.md`

### Git & GitHub

```bash
# État du dépôt
git status --short
git diff --stat
git log -5 --oneline

# Commit (uniquement si demandé par l'utilisateur)
git add <fichiers>
git commit -m "$(cat <<'EOF'
Message ici.
EOF
)"
git push origin main

# Sous-modules
git submodule update --init --recursive
```

### CI GitHub — suivi & logs

Workflow principal : **Build & Publish SoryOS APT** (`.github/workflows/build-and-publish.yml`)

```bash
# Lister les runs récents
gh run list --workflow=build-and-publish.yml --limit 10

# Suivre le run en cours (temps réel)
gh run watch --workflow=build-and-publish.yml

# Suivre un run précis
gh run watch <RUN_ID> --workflow=build-and-publish.yml

# Détail d’un run
gh run view <RUN_ID>

# Logs complets d’un run
gh run view <RUN_ID> --log

# Logs d’un job précis
gh run view <RUN_ID> --job <JOB_ID> --log

# Relancer la CI manuellement (sans commit)
gh workflow run build-and-publish.yml --ref main

# Auth GitHub CLI
gh auth status
gh auth login
```

### Déclencher la CI

```bash
# Automatique : push sur main
git push origin main

# Manuel depuis GitHub Actions UI ou :
gh workflow run build-and-publish.yml --ref main
```

### Build APT complet (local — lourd, éviter sur machine 3,7 Go RAM)

```bash
cd /home/sory/Bureau/soryos.bak/soryos-apt

# Pipeline complet recommandé avant push (signing + tests)
./scripts/init-signing-key.sh
./scripts/build-packages.sh
./scripts/sign-repository.sh
./scripts/test-local-repo.sh
./scripts/apt-smoke-test.sh
./scripts/apt-signed-smoke-test.sh

# Packages d’intégration seulement
./scripts/build-packages.sh

# Tous les composants COSMIC (28 paquets, très long)
./scripts/build-cosmic-local.sh --all

# Un ou plusieurs composants COSMIC
./scripts/build-cosmic-local.sh cosmic-applets
./scripts/build-cosmic-local.sh cosmic-applibrary cosmic-bg

# Lister les composants disponibles
./scripts/build-cosmic-local.sh --list

# Log du dernier build COSMIC local
tail -100 logs/build-cosmic-local.log
grep -i fail logs/build-cosmic-local.log
```

### Build composant COSMIC individuel (Debian)

```bash
cd /home/sory/Bureau/soryos.bak/cosmic-epoch/<composant>

# Build .deb (comme la CI)
dpkg-buildpackage -us -uc -b -d

# Avec just (si justfile présent)
just build-debug
just build-release
just build-vendored          # après vendor.tar extrait
just vendor                  # hors chroot, génère vendor.tar
just --unstable vendor       # cosmic-initial-setup
just vendor-extract
just check
just install
```

### Rust / libcosmic (léger — préférer sur machine limitée)

```bash
# Vérifier libcosmic seul (1 job, moins de RAM)
cd /home/sory/Bureau/soryos.bak/libcosmic
CARGO_BUILD_JOBS=1 cargo check --features wayland

# Vérifier un crate cosmic-epoch
cd /home/sory/Bureau/soryos.bak/cosmic-epoch/cosmic-applets
CARGO_BUILD_JOBS=1 cargo check -p cosmic-applet-audio

# Version Rust
rustc --version
rustup default 1.93

# Régénérer un lockfile (hors CI frozen)
cd /home/sory/Bureau/soryos.bak/cosmic-epoch/cosmic-files
cargo generate-lockfile
```

### Index & publication APT

```bash
cd /home/sory/Bureau/soryos.bak/soryos-apt

# Générer l’index Packages.gz
./scripts/generate-index.sh

# Signer le dépôt
./scripts/sign-repository.sh

# Valider le dépôt local
./scripts/validate-repository.sh
SORYOS_SUITES=stable ./scripts/test-local-repo.sh

# Publier vers sory-x/soryos-apt (CI le fait automatiquement)
./scripts/publish-to-github.py   # nécessite APT_REPO_TOKEN

# Tester le dépôt GitHub Pages
./scripts/apt-pages-smoke-test.sh https://sory-x.github.io/soryos-apt
```

### Tests APT & migration (machine de test)

```bash
cd /home/sory/Bureau/soryos.bak/soryos-apt

# CI locale sans chroot privilégié
SORYOS_SKIP_CHROOT_TESTS=1 ./scripts/ci-local.sh

# CI locale complète (sudo)
sudo ./scripts/ci-local.sh

# Configurer SoryOS APT sur une machine test
sudo ./scripts/configure-soryos-apt.sh

# Installer le desktop stage 1
sudo ./scripts/migrate-stage1-desktop.sh

# Holds critiques (après soryos-system-lock)
sudo soryos-apply-holds
sudo soryos-remove-holds

# Rollback config APT SoryOS
sudo ./scripts/rollback-soryos-apt.sh

# Nettoyage build local APT (ne touche pas /etc/apt)
./scripts/rollback-local.sh

# Build + validation sécurisée
./scripts/build-and-validate.sh
```

### Diagnostic machine (avant build local)

```bash
# Espace disque & RAM
df -h /home/sory/Bureau/soryos.bak
free -h

# Taille des caches cargo
du -sh libcosmic/target cosmic-epoch/*/target 2>/dev/null | sort -hr | head -10
```

### Nettoyage (libère plusieurs Go)

```bash
# Caches cargo (safe si CI fait le build)
rm -rf /home/sory/Bureau/soryos.bak/libcosmic/target
rm -rf /home/sory/Bureau/soryos.bak/cosmic-epoch/cosmic-applets/target
find /home/sory/Bureau/soryos.bak/cosmic-epoch -maxdepth 2 -name target -type d -exec rm -rf {} +

# Nettoyage cargo par crate
cd /home/sory/Bureau/soryos.bak/cosmic-epoch/<composant> && cargo clean
```

### Diagnostic CI / code (grep utiles)

```bash
# Crates avec config.rs gitignored (risque CI)
grep -r "src/config.rs" cosmic-epoch/*/.gitignore

# Crates qui importent mod config
grep -r "mod config" cosmic-epoch/*/src/main.rs

# API libcosmic manquante
grep -r "menu_column\|simple_popup\|LiveSettings" cosmic-epoch/ libcosmic/

# Comparer avec upstream libcosmic
curl -fsSL "https://raw.githubusercontent.com/pop-os/libcosmic/master/src/widget/menu/menu_column.rs"
```

### apt-cache (machine avec dépôt SoryOS configuré)

```bash
sudo apt update
apt-cache policy soryos-desktop cosmic-session cosmic-settings
apt-mark showhold
sudo apt install soryos-desktop
```

### URLs importantes

| Ressource | URL |
|-----------|-----|
| Repo principal | https://github.com/sory-x/sory-os |
| Dépôt APT (Pages) | https://sory-x.github.io/soryos-apt |
| libcosmic upstream | https://github.com/pop-os/libcosmic |

---

## Projet

| Clé | Valeur |
|-----|--------|
| Dépôt | `sory-x/sory-os` — chemin local `/home/sory/Bureau/soryos.bak` |
| Branche principale | `main` |
| Objectif en cours | Faire passer la CI **Build & Publish SoryOS APT** (build Debian de `cosmic-epoch/`) |
| Workflow CI | `.github/workflows/build-and-publish.yml` |
| Script build local CI | `soryos-apt/scripts/build-cosmic-local.sh` |
| libcosmic | Fork vendored dans `libcosmic/` — doit rester aligné avec APIs attendues par `cosmic-epoch` (réf. upstream `pop-os/libcosmic`) |

---

## État actuel

### Tâche active
Corriger les échecs CI restants après fix `cfe65001` (cosmic-comp OK attendu, vérifier cosmic-files, cosmic-edit, greeter, initial-setup).

### Derniers commits (main)
| Commit | Résumé |
|--------|--------|
| `cfe65001` | anim sans tokio (cosmic-comp), nav_bar `window_id_maybe`, applets-config macro, initial-setup `--unstable` |
| `754a4a26` | Référence commandes complète dans PROJECT_MEMORY |
| `9a0f33af` | `menu_column` + `config.rs` cosmic-applibrary |

### CI
| Run | Statut | Notes |
|-----|--------|-------|
| `#30496194437` | en cours (au moment de la MAJ) | commit `9a0f33af` |
| `#30495213736` | en cours | commit `e88aadad` |
| `#30445572055` | **échec** (20/20 composants) | avant sync surface complète |

Commande suivi : `gh run watch --workflow=build-and-publish.yml`

### Composants connus OK (run `#30445572055`)
`cosmic-bg`, `cosmic-icons`, `cosmic-idle`, `cosmic-randr`, `cosmic-screenshot`, `cosmic-session`, `cosmic-store`, `cosmic-wallpapers`, `simple-wrapper`

### Corrections récentes (détail)
- **libcosmic** : `anim::subscription` gated sans tokio (fix cosmic-comp E0425), `nav_bar`/`segmented_button` `window_id_maybe` + `on_surface_action`, cfg `corner_radius` dans `apply_live_settings`
- **cosmic-applets-config** : `cosmic-config` feature `macro` (fix greeter `TimeAppletConfig::VERSION`)
- **cosmic-initial-setup** : `just --unstable` sur toutes les recettes cargo (fix modules instables)
- **cosmic-applibrary** : `src/config.rs` versionné
- **cosmic-greeter daemon** : `cosmic-config` feature `macro` (commit antérieur `e88aadad`)

### Échecs CI typiques encore possibles
- Dérive API **libcosmic** vs cosmic-epoch (panel, settings, applets, comp, term, osd, notifications…)
- **Lockfiles** périmés (`--frozen --offline`)
- **`src/config.rs` gitignored** dans d’autres crates (`cosmic-launcher`, `cosmic-bg`, `cosmic-icons` — vérifier si `mod config` dans `main.rs`)
- **Rust 1.95** demandé par certains paquets vs CI **1.93**
- cosmic-settings (`atspi-common`), xdg-desktop-portal-cosmic vendor, soryos-launcher `dh_fixperms`

### Changements locaux NON commités (hors CI)
- `cosmic-launcher/*`, `cosmic-player/*`, `scripts/version-update.sh`
- dossiers non suivis : `shell/`, `cosmic-sory-ia/docs-compare-projet/`

### Contraintes machine dev
- **Ne pas** lancer de builds cargo massifs en local (freeze / disque plein).
- Nettoyage utile : `rm -rf libcosmic/target cosmic-epoch/*/target` (libère plusieurs Go).

---

## Prochaines étapes
1. Attendre résultat CI `#30496194437` ; analyser logs des composants encore en FAIL.
2. Si nouveaux erreurs libcosmic → sync ciblé depuis `pop-os/libcosmic` (pas de rebuild local complet).
3. Scanner les `.gitignore` avec `src/config.rs` pour les crates qui ont `mod config` dans `main.rs`.
4. Committer **uniquement** les fichiers liés au CI (pas launcher/player/doc unrelated).

---

## Journal (récent en premier)

| Date | Session | Action |
|------|---------|--------|
| 2026-07-29 | CI | Push fix `on_surface_action` Setters skip (cosmic-applets E0592) |
| 2026-07-29 | CI | Push `cfe65001` : fix cosmic-comp, nav_bar, greeter macro, initial-setup unstable |
| 2026-07-29 | mémoire | Ajout section **Commandes** complète dans PROJECT_MEMORY.md |
| 2026-07-29 | CI | Création de `.cursor/PROJECT_MEMORY.md` + règle Cursor `project-memory.mdc` |
| 2026-07-29 | CI | Push `9a0f33af` : `menu_column` + `config.rs` applibrary |
| 2026-07-29 | CI | Push `e88aadad` : sync surface/blur/frosted libcosmic |
| 2026-07-29 | CI | Run `#30445572055` échec 20/20 ; identification dérive libcosmic |
| 2026-07-29 | CI | Push `1def811b` : transparence + deps CI (gstreamer, lockfiles partiels) |

---

## Copier-coller pour nouvelle session

```
Projet SoryOS (sory-x/sory-os). Lis d’abord .cursor/PROJECT_MEMORY.md (surtout « Commandes ») et mets-le à jour en fin de tâche.
On corrige la CI APT cosmic-epoch. Dernier commit : voir « Derniers commits » dans ce fichier.
Évite les builds cargo locaux (machine 3,7 Go RAM). Utilise : gh run watch --workflow=build-and-publish.yml
```
