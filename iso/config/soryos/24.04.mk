DISTRO_NAME=SoryOS

ifeq ($(NVIDIA),1)
DISTRO_VOLUME_LABEL=$(DISTRO_NAME) $(DISTRO_VERSION) $(DISTRO_ARCH) NVIDIA
else
DISTRO_VOLUME_LABEL=$(DISTRO_NAME) $(DISTRO_VERSION) $(DISTRO_ARCH)
endif

# Show splash screen
DISTRO_PARAMS+=quiet splash

SORYOS_APT_URI:=https://sory-x.github.io/soryos-apt

# DEB822 format system repositories
DEB822:=1

# Repositories to be present in installed system
RELEASE_URI:=$(SORYOS_APT_URI)
RELEASE_KEY=/etc/apt/keyrings/soryos-archive-keyring.gpg

# Packages to install
DISTRO_PKGS=\
	systemd \
	soryos-desktop

# Packages to install after (to avoid dependency issues)
POST_DISTRO_PKGS=

POST_DISTRO_PKGS+=rsync

#TODO: systemd-boot is added because it is not depended on by anything
# This was broken out from the systemd package for 24.04 and should be
# added to soryos-desktop and/or kernelstub
POST_DISTRO_PKGS+=systemd-boot

ifeq ($(NVIDIA),1)
DISTRO_PARAMS+=modules_load=nvidia
DISTRO_PARAMS+=nvidia-drm.modeset=1
POST_DISTRO_PKGS+=nvidia-driver-595
ifeq ($(DISTRO_ARCH),amd64)
POST_DISTRO_PKGS+=amd-ppt-bin
endif
endif

# Staging branches to use when building ISO.
# No values is the same as building from release
# `branch-name` is equivalent to `apt-manage add popdev:branch-name -y`
STAGING_BRANCHES=

# Packages to have in live instance
LIVE_PKGS=\
	casper \
	distinst \
	expect \
	gparted

# Packages to remove from installed system (usually installed as Recommends)
RM_PKGS=\
	snapd \
	ubuntu-advantage-tools \
	ubuntu-minimal \
	ubuntu-session \
	ubuntu-wallpapers \
	unattended-upgrades

# Packages not installed, but that may need to be discovered by the installer
MAIN_POOL=\
	efibootmgr \
	ethtool \
	grub-efi-$(DISTRO_ARCH) \
	grub-efi-$(DISTRO_ARCH)-bin \
	grub-efi-$(DISTRO_ARCH)-signed \
	hdparm \
	kernelstub \
	lm-sensors \
	pm-utils \
	postfix \
	powermgmt-base \
	xbacklight
ifeq ($(DISTRO_ARCH),amd64)
MAIN_POOL+=\
	grub-gfxpayload-lists \
	grub-pc \
	grub-pc-bin \
	libx86-1 \
	vbetool
endif

ifeq ($(NVIDIA),1)
MAIN_POOL+=\
	system76-driver-nvidia
endif

# Additional pool packages from the restricted set of packages
ifeq ($(DISTRO_ARCH),amd64)
RESTRICTED_POOL=\
	amd64-microcode \
	intel-microcode \
	iucode-tool
else
RESTRICTED_POOL=
endif

# Extra packages to install in the pool for use by iso creation
POOL_PKGS=\
	grub-efi-$(DISTRO_ARCH)-bin \
	grub-efi-$(DISTRO_ARCH)-signed \
	shim-signed

ifeq ($(HP),1)
DISTRO_VOLUME_LABEL=$(DISTRO_NAME) $(DISTRO_VERSION) $(DISTRO_ARCH) HP
LIVE_PKGS+=\
	dbus-x11
endif
