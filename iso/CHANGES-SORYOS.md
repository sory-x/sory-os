# Changements effectués pour SoryOS

Ce fichier documente toutes les modifications apportées au dossier `iso/`
pour l'adapter de Pop!_OS vers SoryOS.

---

## 1. Makefile

**Fichier :** `Makefile`

| Ligne | Avant | Après |
|-------|-------|-------|
| 2 | `DISTRO_CODE?=pop-os` | `DISTRO_CODE?=soryos` |

---

## 2. Config — dossier `config/`

### 2.1 Renommage

| Avant | Après |
|-------|-------|
| `config/pop-os/` | `config/soryos/` |

### 2.2 `config/soryos/24.04.mk`

| Changement | Détail |
|---|---|
| `DISTRO_NAME` | `Pop_OS` → `SoryOS` |
| Suppression des paramètres ARM | `console=tty0`, `ast.modeset=0` retirés (non applicable) |
| Suppression `GNOME_INITIAL_SETUP_STAMP` | Plus nécessaire (COSMIC, pas GNOME) |
| `DEB822` | Gardé à 1 |
| `APPS_URI` | **Supprimé** — `http://apt.pop-os.org/proprietary` remplacé par `RELEASE_URI` seulement |
| `APPS_KEY` | Supprimé |
| `RELEASE_URI` | `http://apt.pop-os.org/release` → `https://sory-x.github.io/soryos-apt` |
| `RELEASE_KEY` | `pop-keyring-2017-archive.gpg` → `soryos-archive-keyring.gpg` |
| `STAGING_BRANCHES` | Supprimé (pas de staging SoryOS) |
| `DISTRO_PKGS` | `systemd cosmic-term linux-system76 pop-desktop` → `systemd soryos-desktop` |
| `POST_DISTRO_PKGS` | `system76-io-dkms system76-acpi-dkms system76-dkms gcc-14 rsync systemd-boot` → `rsync systemd-boot` |
| `POST_DISTRO_PKGS` NVIDIA | `nvidia-driver-595 amd-ppt-bin` conservé |
| `LIVE_PKGS` | `casper cosmic-initial-setup-casper distinst expect gparted pop-installer pop-installer-casper` → `casper distinst expect gparted` |
| `RM_PKGS` | Allégé : `snapd ubuntu-advantage-tools ubuntu-minimal ubuntu-session ubuntu-wallpapers unattended-upgrades` |
| `MAIN_POOL` | Nettoyage complet : retrait de `pop-hp-*`, `system76-driver`, `system76-firmware-daemon`, `system76-wallpapers`, `system76-power`, `system76-oled`, `gnome-shell-extension-system76-power`, `firmware-manager`, `hidpi-daemon`, `pop-hp-vendor*`, `python3-*` |
| `MAIN_POOL` NVIDIA | `system76-driver-nvidia` conservé |
| `RESTRICTED_POOL` | Conservé (`amd64-microcode`, `intel-microcode`, `iucode-tool`) |
| `POOL_PKGS` | Conservé |
| Section `HP=1` | `POST_DISTRO_PKGS` et `RM_PKGS` HP retirés (plus de `pop-hp-*`). Gardé `dbus-x11` |

### 2.3 `config/soryos/22.04.mk`

| Changement | Détail |
|---|---|
| `DISTRO_NAME` | `Pop_OS` → `SoryOS` |

*(Contenu non modifié par ailleurs — version legacy)*

### 2.4 `config/soryos/26.04.mk`

| Changement | Détail |
|---|---|
| `DISTRO_NAME` | `Pop_OS` → `SoryOS` |

*(Contenu non modifié par ailleurs — placeholder futur)*

---

## 3. Makefiles inclus — `mk/`

### 3.1 `mk/ubuntu.mk`

| Ligne | Avant | Après |
|-------|-------|-------|
| 16 | `http://apt.pop-os.org/ubuntu` | `http://archive.ubuntu.com/ubuntu` |

### 3.2 `mk/chroot.mk`

| Ligne | Avant | Après |
|-------|-------|-------|
| 55 | Export clé GPG `204DD8AEC33A7AFF` vers `pop-keyring-2017-archive.gpg` | Copie `soryos-archive-keyring.gpg` depuis `../soryos-apt/keyrings/` |
| 61 | `data/apt-preferences` → `pop-iso` | `data/apt-preferences` → `soryos-iso` |
| 95 | `KEY=/iso/pop-keyring-2017-archive.gpg` | `KEY=/iso/soryos-archive-keyring.gpg` |
| 95 | `STAGING_BRANCHES=$(STAGING_BRANCHES)` et `$(DISTRO_REPOS)` supprimés de la commande chroot | Variables retirées (plus de staging ni de repos supplémentaires) |
| 129 | `pop-iso` | `soryos-iso` |

### 3.3 `mk/germinate.mk`

| Ligne | Avant | Après |
|-------|-------|-------|
| 45 | `-m http://ppa.launchpad.net/system76/pop/ubuntu` | Ligne supprimée |

---

## 4. Données — `data/`

### 4.1 `data/apt-preferences`

Contenu remplacé :

```text
Package: *
Pin: release o=SoryOS
Pin-Priority: 1001

Package: *
Pin: release o=Ubuntu
Pin-Priority: 500
```

(Ancien : priorités pour `pop-os-release`, `pop-os-staging-master`, `LP-PPA-system76-pop`, `LP-PPA-system76-proposed`)

---

## 5. Scripts — `scripts/`

### 5.1 `scripts/chroot.sh`

Aucune modification nécessaire. Le script est générique :
- La clé APT est passée via `$KEY` (maintenant `soryos-archive-keyring.gpg`)
- Les PPAs/deb lines sont passés en arguments — plus utilisés (`DISTRO_REPOS` vidé)
- `STAGING_BRANCHES` n'est plus défini → le bloc `apt-manage` ne s'exécute pas

### 5.2 `deps.sh`

Aucun changement — la ligne `gpg --keyserver keyserver.ubuntu.com --recv-keys 204DD8AEC33A7AFF`
est conservée (infrastructure Ubuntu, pas Pop!_OS).

---

## 6. Ce qui n'a PAS été changé (inchangé, fonctionnel)

- `mk/automatic.mk` — variables génériques (sed, chemins)
- `mk/language.mk` — langues supportées
- `mk/iso.mk` — construction ISO (xorriso, squashfs, grub, pool)
- `mk/clean.mk` — cibles de nettoyage
- `mk/qemu.mk` — tests QEMU
- `mk/update.mk` — mise à jour des chroots
- `scripts/console-setup.sh` — configuration console
- `scripts/mount.sh` — montage chroot
- `scripts/unmount.sh` — démontage chroot
- `scripts/pool.sh` — correction noms URL-encodés
- `scripts/repos.sh` — écriture fichiers .sources DEB822
- `data/grub/grub.cfg` — utilise des variables Makefile (DISTRO_NAME, etc.)
- `data/isolinux/isolinux.cfg` — utilise des variables Makefile
- `data/disk/info` — utilise des variables Makefile
- `data/Release` — utilise des variables Makefile
- `data/efi/shimx64.efi.signed` — binaire signé (générique)
- `data/{amd64,arm64}/kernelstub` — configuration kernelstub (inchangée)
- `data/prime-discrete` — configuration live NVIDIA
- `data/system76-power.conf` — modprobe (ancore nécessaire pour NVIDIA)
- `data/grub-theme/` — sous-module git (encore system76/pop-grub-theme)

## 7. Références pop-os restantes (non modifiées, non bloquantes)

| Fichier | Raison |
|---|---|
| `README.md` | Documentation — à réécrire pour SoryOS |
| `CONTRIBUTING.md` | Documentation — à réécrire |
| `CODE_OF_CONDUCT.md` | Pointe vers Pop! Code of Conduct |
| `.github/ISSUE_TEMPLATE.md` | Template GitHub Issues |
| `.gitmodules` | Sous-module `system76/pop-grub-theme` — à forker |
| `buildchain.json` | CI — utilise PPA system76/pop |
| `config/soryos/22.04.mk` | Legacy — `DISTRO_NAME` changé, contenu pop-os conservé |
| `config/soryos/26.04.mk` | Placeholder — `DISTRO_NAME` changé, contenu pop-os conservé |
| `config/soryos/24.04.mk` ligne 91 | `system76-driver-nvidia` — paquet existant, conservé pour NVIDIA |

---

*Document généré le 26 juillet 2026*
