# Projet SoryOS — Mémoire persistante

## Identité
- **Créateur** : Ibrahima Sory Keita, Conakry (Guinée)
- Intérêts : programmation, systèmes d'exploitation, IA, produits tech, mathématiques, physique, chimie, musique, courses auto, motos
- Machine principale : Lenovo ThinkPad T13 2nd Gen (Intel Celeron 3865U, 4 Go RAM, ~238 Go SSD)
- Stockage externe : Seagate 500 Go, supports USB
- OS de travail habituel : Linux Mint 22.3, adapté vers SoryOS

## Projets principaux

### SoryOS
Écosystème logiciel complet, pas juste une distribution Linux. Objectif : plateforme native centrée sur Sory IA, avec :
- Son propre système (base Debian/Ubuntu, inspiré Pop!_OS / Linux Mint)
- Son propre bureau (COSMIC Desktop adapté, libcosmic, design system "Deep Navy Glass")
- Sa bibliothèque UI (libcosmic, fork de pop-os/libcosmic)
- Son design system (palette, typographie, espacement, coins, ombres, motion)
- Ses applications (Files, Settings, Store, Terminal, Browser, Notifications en Rust)
- Ses outils développeur (SoryCode/SoryGenius)
- Ses services (Sory IA)
- Son dépôt APT : https://sory-x.github.io/soryos-apt/
- Objectif : OS universel supportant applications Linux natives + Windows .exe (Wine intégré)

### SoryCode / SoryGenius
Plateforme de développement assistée par IA. Architecture multi-agents :
- Agent général de projet coordonne des agents managers responsables d'applications
- Chaque app peut avoir son équipe : Planner, Writer, Tester, Reviewer, Fixer
- Environnement proche d'un IDE moderne : éditeur Monaco, terminal, explorateur fichiers, aperçu, marketplace, panneau IA
- Architecture principalement en Rust
- Explore Codex CLI, Gemini CLI, OpenCode, Cursor Agent, modèles IA locaux

### Sory IA
Modèles d'IA pour les outils SoryOS. Recherche de solutions gratuites/ouvertes :
- Modèles explorés : Qwen (GGUF), DeepSeek, Mistral, GLM, Devstral, Step, etc.
- Outils : Hugging Face CLI, Ollama, llama.cpp
- Objectif : IA intégrée nativement dans l'OS

## Architecture technique
- **Langage principal** : Rust (apps, composants système, outils)
- **UI Toolkit** : Iced + libcosmic (forké de pop-os/libcosmic, adapté SoryOS)
- **Desktop** : COSMIC Epoch 1.4.0 (forké de pop-os/cosmic-epoch)
- **Build ISO** : Makefile, debootstrap, xorriso, squashfs (forké de pop-os/iso)
- **Paquets** : Dépôt APT avec GitHub Pages, signatures GPG
- **Design** : "Deep Navy Glass" — design system complet (couleurs, typo, radius, shadow, motion)

## Conventions et workflows

### Commandes
- Build ISO : `make` ou `make iso` depuis `iso/`
- Nettoyage : `make clean`, `make distclean`
- Test QEMU : `make qemu_bios`, `make qemu_uefi`
- Générer seeds germinate : `make germinate`
- APT local : `./scripts/build-packages.sh && ./scripts/sign-repository.sh`
- Build tout depuis soryos-apt : CI GitHub Actions (matrix build 28 composants COSMIC)

### Architecture des dossiers
- `/home/sory/Bureau/soryos/` — workspace racine
  - `cosmic-epoch/` — monorepo COSMIC desktop (~30 composants Rust)
  - `iso/` — builder ISO (Makefile, chroot, scripts, configs)
  - `libcosmic/` — toolkit GUI Rust (fork pop-os/libcosmic)
  - `soryos-apt/` — dépôt APT (pool, dists, keyrings, templates, scripts, docs)

### Liste des composants COSMIC dans cosmic-epoch
- cosmic-comp (compositeur Wayland)
- cosmic-session, cosmic-greeter (session/login)
- cosmic-panel, cosmic-applets, cosmic-launcher, cosmic-notifications, cosmic-osd
- cosmic-files, cosmic-edit, cosmic-term, cosmic-store, cosmic-player
- cosmic-settings, cosmic-settings-daemon
- cosmic-bg, cosmic-idle, cosmic-randr, cosmic-screenshot, cosmic-monitor
- cosmic-applibrary, cosmic-workspaces-epoch
- cosmic-icons, cosmic-wallpapers, cosmic-sound-theme, cosmic-initial-setup
- xdg-desktop-portal-cosmic, pop-launcher, simple-wrapper
- **cosmic-sory-ia** (fork local Codex + UI COSMIC)

### Problèmes connus (ISO)
1. Origin/Label APT dans iso.mk = "Ubuntu" au lieu de "SoryOS" (casse pinning)
2. GRUB theme Pop non remplacé
3. Pas d'installateur dans LIVE_PKGS
4. Clé GPG copiée avec `|| true` (silencieux si absent)
5. chroot.sh détecte langues avec `XDG_CURRENT_DESKTOP=GNOME`
6. Fichiers morts (system76-power.conf, prime-discrete) encore copiés
7. Pas de splash plymouth SoryOS
8. Aucun .deb COSMIC dans pool APT pour le moment

## Objectifs à long terme
- OS universel : Linux natif + compatibilité Windows (Wine) transparente
- Écosystème indépendant : dépôt APT, bureau, apps, outils dev, IA
- Pas de simple copie de Pop!_OS — créer une identité propre SoryOS
- Apprendre des projets existants (Pop!_OS, Mint, Ubuntu/Debian) sans copier
- Infrastructure gratuite/ouverte : GitHub Pages pour APT, solutions IA locales/libres
