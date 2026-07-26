#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
POOL_DIR="$ROOT_DIR/pool/stable"
COSMIC_DIR="$ROOT_DIR/../cosmic-epoch"
LOG_DIR="$ROOT_DIR/logs"
LOG_FILE="$LOG_DIR/build-cosmic-local.log"

mkdir -p "$LOG_DIR" "$POOL_DIR"
: > "$LOG_FILE"

require_tool() {
  if ! command -v "$1" >/dev/null 2>&1; then
    printf 'missing required tool: %s\n' "$1" | tee -a "$LOG_FILE" >&2
    exit 1
  fi
}

require_tool dpkg-buildpackage
require_tool dpkg-deb

build_component() {
  local dir="$1"
  local control_file="$dir/debian/control"

  if [ ! -f "$control_file" ]; then
    printf 'SKIP %s: no debian/control\n' "$dir" | tee -a "$LOG_FILE"
    return 0
  fi

  local source_name
  source_name=$(grep '^Source:' "$control_file" | sed 's/Source: //')

  printf 'Building %s (%s)...\n' "$dir" "$source_name" | tee -a "$LOG_FILE"

  pushd "$dir" >/dev/null
  if dpkg-buildpackage -us -uc -b -d >> "$LOG_FILE" 2>&1; then
    popd >/dev/null
    # Move .deb to pool
    for deb in "${source_name}"_*.deb; do
      if [ -f "$deb" ]; then
        mv "$deb" "$POOL_DIR/"
        printf '  -> %s\n' "$deb" | tee -a "$LOG_FILE"
      fi
    done
    return 0
  else
    popd >/dev/null
    printf 'FAIL %s\n' "$dir" | tee -a "$LOG_FILE" >&2
    return 1
  fi
}

# Components to build (all with debian/)
COMPONENTS=(
  cosmic-applets cosmic-applibrary cosmic-bg cosmic-comp
  cosmic-edit cosmic-files cosmic-greeter cosmic-icons
  cosmic-idle cosmic-initial-setup cosmic-launcher cosmic-monitor
  cosmic-notifications cosmic-osd cosmic-panel cosmic-player
  cosmic-randr cosmic-screenshot cosmic-session cosmic-settings
  cosmic-settings-daemon cosmic-sory-ia cosmic-store cosmic-term cosmic-wallpapers
  cosmic-workspaces-epoch pop-launcher simple-wrapper
  xdg-desktop-portal-cosmic
)

BUILD_ALL=false
TARGETS=()

while [ $# -gt 0 ]; do
  case "$1" in
    --all) BUILD_ALL=true ;;
    --list)
      for c in "${COMPONENTS[@]}"; do echo "$c"; done
      exit 0
      ;;
    *)
      TARGETS+=("$1")
      ;;
  esac
  shift
done

if [ "$BUILD_ALL" = true ] || [ ${#TARGETS[@]} -eq 0 ]; then
  TARGETS=("${COMPONENTS[@]}")
fi

if [ ! -d "$COSMIC_DIR" ]; then
  printf 'ERROR: cosmic-epoch not found at %s\n' "$COSMIC_DIR" >&2
  exit 1
fi

cd "$COSMIC_DIR"

FAILED=0
for target in "${TARGETS[@]}"; do
  if [ ! -d "$target" ]; then
    printf 'SKIP %s: not found\n' "$target" | tee -a "$LOG_FILE"
    continue
  fi
  build_component "$target" || FAILED=$((FAILED + 1))
done

if [ "$FAILED" -gt 0 ]; then
  printf '\n%d component(s) failed\n' "$FAILED" | tee -a "$LOG_FILE" >&2
else
  printf '\nAll components built successfully\n' | tee -a "$LOG_FILE"
fi

exit "$FAILED"
