# SoryOS — Mémoire de session (agent)

> **Fichier de continuité entre sessions Cursor.**  
> Toute session (actuelle ou future) doit **lire ce fichier en premier** au début d’un travail sur ce dépôt, puis **le mettre à jour en fin de tâche** (voir section « Protocole agent »).

Dernière mise à jour : **2026-07-29** (session CI APT / libcosmic)

---

## Protocole agent (OBLIGATOIRE)

1. **Au début** : lire ce fichier en entier avant d’agir sur le dépôt.
2. **Pendant** : noter mentalement objectif, blocages, décisions.
3. **À la fin de chaque tâche** (fix, commit, push, diagnostic CI, etc.) :
   - mettre à jour **« État actuel »**, **« Derniers commits »**, **« CI »**, **« Prochaines étapes »** ;
   - ajouter une ligne dans **« Journal »** (date + résumé) ;
   - conserver ce protocole tel quel (ne pas le supprimer).
4. **Ne pas** dupliquer tout l’historique : garder ~15 entrées de journal max ; résumer l’ancien en une phrase si besoin.
5. **Build local** : machine dev souvent à **3,7 Go RAM** et disque **>95 %** — éviter `cargo build` complet ; préférer CI GitHub ou `cargo check -j1` ciblé.

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
Corriger les échecs de build APT un par un ; dernière poussée : **menu_column** + **config.rs** cosmic-applibrary.

### Derniers commits (main)
| Commit | Résumé |
|--------|--------|
| `9a0f33af` | `menu_column` dans libcosmic ; `cosmic-applibrary/src/config.rs` versionné (plus gitignored) |
| `e88aadad` | Sync API surface libcosmic (blur, popup, frosted, layer-shell) + fixes greeter/monitor/initial-setup/sory-ia |
| `1def811b` | API transparence libcosmic, gstreamer CI, lockfiles partiels |

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
- **libcosmic** : `transparent`, `AppType`, `Auto`, `blur`, `LiveSettings`, `simple_popup` 3 args, `menu_column`, stubs iced_winit (`blur`, `set_padding`, `BlurEnabled`)
- **cosmic-applibrary** : `src/config.rs` était gitignored → absent en CI
- **cosmic-greeter** : `cosmic-config` feature `macro` sur daemon
- **cosmic-monitor** : `rust-version = "1.93"`
- **cosmic-initial-setup** : `just --unstable` pour vendor
- **cosmic-sory-ia** : suppression `debian/compat` (conflit `debhelper-compat`)
- **cosmic-files** : `Cargo.lock` régénéré (local)

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
| 2026-07-29 | CI | Création de `.cursor/PROJECT_MEMORY.md` + règle Cursor `project-memory.mdc` |
| 2026-07-29 | CI | Push `9a0f33af` : `menu_column` + `config.rs` applibrary |
| 2026-07-29 | CI | Push `e88aadad` : sync surface/blur/frosted libcosmic |
| 2026-07-29 | CI | Run `#30445572055` échec 20/20 ; identification dérive libcosmic |
| 2026-07-29 | CI | Push `1def811b` : transparence + deps CI (gstreamer, lockfiles partiels) |

---

## Copier-coller pour nouvelle session

```
Projet SoryOS (sory-x/sory-os). Lis d’abord .cursor/PROJECT_MEMORY.md et mets-le à jour en fin de tâche.
On corrige la CI APT cosmic-epoch. Dernier commit : voir « Derniers commits » dans ce fichier.
Évite les builds cargo locaux (machine 3,7 Go RAM). Utilise la CI GitHub.
```
