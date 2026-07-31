# Dépendances Git vendor — miroirs `sory-x`

Document de référence pour la migration des `Cargo.toml` de SoryOS : remplacer les URLs upstream (`pop-os`, `wash2`, etc.) par les forks **`sory-x`**, tout en gardant les mêmes `rev` / `tag` / `branch`.

**État au 2026-07-31**

- [x] 24 dépôts clonés localement dans `~/Bureau/soryos.bak/` (hors `cosmic-epoch/`)
- [x] 24 dépôts publiés sur GitHub sous `https://github.com/sory-x/<nom>`
- [x] Historique complet poussé (branches + tags)
- [ ] Migration des `Cargo.toml` du monorepo (`cosmic-epoch/`, `libcosmic/`)
- [ ] Régénération des `Cargo.lock` concernés
- [ ] Validation CI (`build-and-publish.yml`)

---

## Script de republication

Si un miroir doit être resynchronisé depuis les clones locaux :

```bash
./scripts/push-vendor-repos-to-sory-x.sh
# log : scripts/push-vendor-repos.log
```

Chaque clone local a deux remotes :

| Remote   | Rôle                                      |
|----------|-------------------------------------------|
| `origin` | Upstream d'origine (pop-os, wash2, …)     |
| `sory-x` | Miroir GitHub `https://github.com/sory-x/…` |

---

## Inventaire des 24 miroirs `sory-x`

### pop-os (10)

| Repo | URL clone `sory-x` | Pin Cargo actuel |
|------|--------------------|------------------|
| cosmic-protocols | `https://github.com/sory-x/cosmic-protocols.git` | `rev = "32283d7"` |
| dbus-settings-bindings | `https://github.com/sory-x/dbus-settings-bindings.git` | `rev = "eed01dd"` (libcosmic) ; souvent sans rev ailleurs |
| freedesktop-icons | `https://github.com/sory-x/freedesktop-icons.git` | `rev = "ab4c57b"` |
| cosmic-mime-apps | `https://github.com/sory-x/cosmic-mime-apps.git` | HEAD |
| launch-pad | `https://github.com/sory-x/launch-pad.git` | HEAD |
| cosmic-syntax-theme | `https://github.com/sory-x/cosmic-syntax-theme.git` | HEAD |
| xdg-shell-wrapper | `https://github.com/sory-x/xdg-shell-wrapper.git` | HEAD |
| winit | `https://github.com/sory-x/winit.git` | `tag = "cosmic-0.14"` |
| softbuffer | `https://github.com/sory-x/softbuffer.git` | `tag = "cosmic-4.0"` |
| window_clipboard | `https://github.com/sory-x/window_clipboard.git` | `tag = "sctk-0.20"` |

### Tiers OS (14)

| Repo | URL clone `sory-x` | Pin Cargo actuel |
|------|--------------------|------------------|
| atspi | `https://github.com/sory-x/atspi.git` | HEAD (workspace patch dans cosmic-settings) |
| iced_video_player | `https://github.com/sory-x/iced_video_player.git` | HEAD |
| client-toolkit | `https://github.com/sory-x/client-toolkit.git` | HEAD |
| accesskit | `https://github.com/sory-x/accesskit.git` | `tag = "cosmic-0.14"` |
| sysinfo | `https://github.com/sory-x/sysinfo.git` | `branch = "redox-0.39"` |
| spdx | `https://github.com/sory-x/spdx.git` | HEAD |
| appstream | `https://github.com/sory-x/appstream.git` | HEAD |
| rust-atomicwrites | `https://github.com/sory-x/rust-atomicwrites.git` | HEAD |
| smithay | `https://github.com/sory-x/smithay.git` | `rev = "1ed69cb"` (cosmic-comp) ; HEAD ailleurs |
| id-tree | `https://github.com/sory-x/id-tree.git` | HEAD |
| xdg-mime-rs | `https://github.com/sory-x/xdg-mime-rs.git` | HEAD |
| locales-rs | `https://github.com/sory-x/locales-rs.git` | HEAD |
| cryoglyph | `https://github.com/sory-x/cryoglyph.git` | `rev = "e429a025df36ab8145708acb309080ae3deec17a"` (libcosmic/iced) |
| pipewire-rs | `https://github.com/sory-x/pipewire-rs.git` | HEAD |

**Hors scope OS (sory-ia)** : `rust-sdks`, `nucleo`, `crossterm`, `ratatui`, `tokio-tungstenite`, `tungstenite-rs`, `rules_rust` — à traiter séparément si besoin.

---

## Ce qui ne doit PAS passer par `sory-x` git

Ces crates sont **déjà dans le monorepo** `cosmic-epoch/` — migration en `path`, pas en fork :

| Upstream pop-os encore en git | Remplacer par |
|-------------------------------|---------------|
| `cosmic-bg` | `../cosmic-bg/config` |
| `cosmic-comp` | `../cosmic-comp/cosmic-comp-config` |
| `cosmic-idle` | `../cosmic-idle/cosmic-idle-config` |
| `cosmic-panel` (config) | `../cosmic-panel/cosmic-panel-config` |
| `cosmic-randr` | `../cosmic-randr/shell` |
| `cosmic-files` | `../cosmic-files` |
| `cosmic-settings` (subscriptions) | `../cosmic-settings/subscriptions/…` |
| `cosmic-settings-daemon` | `../cosmic-settings-daemon/…` |

**Cas particulier** : `cosmic-settings-network-manager-subscription` — ajouté dans `cosmic-settings/subscriptions/network-manager/` (depuis pop-os `ae5c750`), utilisé en `path` par `cosmic-initial-setup`.

**libcosmic** : `cosmic-panel-config` peut aussi passer en `path = "../../cosmic-epoch/cosmic-panel/cosmic-panel-config"` au lieu de git pop-os.

---

## Plan de migration `Cargo.toml` (à faire ensuite)

### Étape 1 — Remplacements directs `git` → `sory-x`

Remplacer l'URL en conservant **exactement** le même `rev`, `tag`, `branch`, `package`, `features`, `optional`.

Exemple :

```toml
# Avant
cctk = { git = "https://github.com/pop-os/cosmic-protocols", package = "cosmic-client-toolkit", rev = "32283d7" }

# Après
cctk = { git = "https://github.com/sory-x/cosmic-protocols", package = "cosmic-client-toolkit", rev = "32283d7" }
```

Table de correspondance URL :

| Ancienne base | Nouvelle base |
|---------------|---------------|
| `https://github.com/pop-os/` | `https://github.com/sory-x/` |
| `https://github.com/wash2/` | `https://github.com/sory-x/` |
| `https://github.com/jackpot51/` | `https://github.com/sory-x/` |
| `https://github.com/smithay/smithay` | `https://github.com/sory-x/smithay` |
| `https://github.com/Drakulix/id-tree` | `https://github.com/sory-x/id-tree` |
| `https://github.com/ebassi/xdg-mime-rs` | `https://github.com/sory-x/xdg-mime-rs` |
| `https://github.com/AerynOS/locales-rs` | `https://github.com/sory-x/locales-rs` |
| `https://github.com/iced-rs/cryoglyph` | `https://github.com/sory-x/cryoglyph` |
| `https://gitlab.freedesktop.org/pipewire/pipewire-rs` | `https://github.com/sory-x/pipewire-rs` |

### Étape 2 — `[patch]` globaux (option recommandée)

Pour éviter de modifier chaque crate individuellement, on peut centraliser dans les workspaces racines :

**Fichiers candidats** :

- `libcosmic/Cargo.toml`
- `libcosmic/iced/Cargo.toml`
- `cosmic-epoch/cosmic-settings/Cargo.toml`
- `cosmic-epoch/cosmic-applets/Cargo.toml`
- `cosmic-epoch/cosmic-panel/Cargo.toml`
- `cosmic-epoch/cosmic-osd/Cargo.toml`
- `cosmic-epoch/xdg-desktop-portal-cosmic/Cargo.toml`

Exemple de patch :

```toml
[patch."https://github.com/pop-os/cosmic-protocols"]
cosmic-client-toolkit = { git = "https://github.com/sory-x/cosmic-protocols", rev = "32283d7" }
cosmic-protocols = { git = "https://github.com/sory-x/cosmic-protocols", rev = "32283d7" }

[patch."https://github.com/pop-os/dbus-settings-bindings"]
# … une entrée par crate du workspace dbus-settings-bindings utilisé
```

> **Attention** : après migration vers `sory-x`, il faudra aussi des `[patch."https://github.com/sory-x/…"]` si des sous-dépendances pointent encore vers l'ancienne URL, ou remplacer toutes les URLs explicitement.

### Étape 3 — Fichiers prioritaires (CI)

| Fichier | Type de changement |
|---------|-------------------|
| `libcosmic/Cargo.toml` | git → sory-x + path cosmic-panel-config |
| `libcosmic/iced/Cargo.toml` | git → sory-x (winit, softbuffer, window_clipboard, cctk, cryoglyph) |
| `libcosmic/cosmic-config/Cargo.toml` | dbus-settings-bindings → sory-x |
| `cosmic-epoch/cosmic-settings/Cargo.toml` | path pour configs monorepo + atspi → sory-x |
| `cosmic-epoch/cosmic-settings/cosmic-settings/Cargo.toml` | dbus + mime + protocols → sory-x |
| `cosmic-epoch/cosmic-greeter/Cargo.toml` | protocols + dbus → sory-x |
| `cosmic-epoch/cosmic-initial-setup/Cargo.toml` | protocols + dbus → sory-x ; path comp/randr ; sync network-manager |
| `cosmic-epoch/cosmic-osd/Cargo.toml` | path settings/daemon (pas git pop-os) |
| `cosmic-epoch/cosmic-term/Cargo.toml` | path cosmic-files |
| `cosmic-epoch/cosmic-comp/Cargo.toml` | protocols + smithay → sory-x |
| `cosmic-epoch/cosmic-files/Cargo.toml` | protocols + mime + xdg-mime → sory-x |
| `cosmic-epoch/cosmic-session/Cargo.toml` | dbus + launch-pad → sory-x |
| `cosmic-epoch/simple-wrapper/…` | xdg-shell-wrapper + smithay + client-toolkit → sory-x |
| `cosmic-epoch/cosmic-settings-daemon/Cargo.toml` | dbus + protocols → sory-x |
| `cosmic-epoch/cosmic-applets/**` | dbus + protocols → sory-x ; path settings où applicable |
| `cosmic-epoch/cosmic-edit/Cargo.toml` | cosmic-syntax-theme → sory-x |
| `cosmic-epoch/cosmic-workspaces-epoch/Cargo.toml` | freedesktop-icons → sory-x |
| `cosmic-epoch/cosmic-player/Cargo.toml` | iced_video_player → sory-x |
| `cosmic-epoch/cosmic-monitor/Cargo.toml` | sysinfo → sory-x |
| `cosmic-epoch/cosmic-store/Cargo.toml` | spdx + appstream + atomicwrites + dbus → sory-x |
| `cosmic-epoch/xdg-desktop-portal-cosmic/Cargo.toml` | protocols + pipewire-rs → sory-x |

### Étape 4 — Lockfiles

Après chaque lot de changements :

```bash
cd cosmic-epoch/<crate>
cargo update -p <package>   # ou cargo generate-lockfile si politique du projet
```

En CI Debian (`--frozen --offline`), chaque crate modifié doit avoir son `Cargo.lock` commité.

### Étape 5 — Vérification

```bash
# URLs externes restantes (hors sory-x et path)
rg 'git = "https://github.com/(pop-os|wash2|jackpot51|smithay|Drakulix|ebassi|AerynOS|iced-rs)' \
  cosmic-epoch libcosmic --glob '**/Cargo.toml'

rg 'gitlab.freedesktop.org/pipewire' cosmic-epoch libcosmic --glob '**/Cargo.toml'

# Build local ciblé (éviter build complet si RAM limitée)
cd soryos-apt && ./scripts/build-cosmic-local.sh cosmic-settings

# CI
gh run watch --workflow=build-and-publish.yml
```

---

## Commandes utiles

### Cloner un vendor depuis `sory-x`

```bash
git clone https://github.com/sory-x/cosmic-protocols.git
git -C cosmic-protocols checkout 32283d7
```

### Lister les refs git encore externes dans le monorepo

```bash
rg 'git = "https://' cosmic-epoch libcosmic --glob '**/Cargo.toml' | rg -v 'sory-x'
```

### Voir tous les miroirs sur GitHub

```bash
gh repo list sory-x --limit 100
```

---

## Checklist reprise (quand on continue)

- [x] Récupérer `cosmic-settings-network-manager-subscription` dans le monorepo
- [ ] Migrer les `path` monorepo (cosmic-settings workspace, term, osd, greeter, initial-setup)
- [ ] Remplacer toutes les URLs vendor par `sory-x` (ou `[patch]` centralisés)
- [ ] Mettre à jour `soryos-apt/sources/sources.yml` avec les 24 vendors si besoin APT
- [ ] Régénérer / committer les `Cargo.lock` impactés
- [ ] Pousser sur `main` et valider CI
- [ ] Mettre à jour `.cursor/PROJECT_MEMORY.md`

---

## Références

- Clones locaux vendor : répertoire racine du repo (`cosmic-protocols/`, `dbus-settings-bindings/`, …) — **ne pas confondre** avec `cosmic-epoch/`
- Script push : `scripts/push-vendor-repos-to-sory-x.sh`
- CI packages : `.github/workflows/build-and-publish.yml`
- Inventaire sources APT : `soryos-apt/sources/sources.yml`
