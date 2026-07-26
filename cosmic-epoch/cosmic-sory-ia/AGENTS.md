# cosmic-sory-ia — Notes de mémoire du projet

## Structure workspace unifié

`cargo build` depuis la racine `/home/sory/Bureau/soryos/cosmic-epoch/cosmic-sory-ia/`
avec un seul `target/` partagé.

- `sory-rs/` → moteur IA (fork OpenAI Codex, ~115+ crates Rust)
- `sory-desktop/` → interface graphique COSMIC (GTK/libcosmic)
- `patch-rama-http/` → patch pour rama-http

## Contraintes machine

- **2 cœurs CPU**, **3.7 Go RAM**, SSD
- Ne JAMAIS lancer `cargo build --release` (sans `-p`) — sature la RAM (>8h)
- Toujours utiliser `cargo build -p NOM_DU_PACKAGE` ou `cargo check -p NOM_DU_PACKAGE`
- mold + clang sont installés, config dans `~/.cargo/config.toml`
- `sccache` en cours d'installation (background)

## Crates principales

| Package | Description |
|---|---|
| `sory-cli` | Binaire CLI principal |
| `sory-desktop` | Binaire desktop COSMIC |
| `sory-tui` | Interface TUI (ratatui) |
| `sory-core` | Crate centrale du moteur |
| `sory-mcp` | Protocole MCP |
| `sory-login` | Auth et login |
| `sory-config` | Configuration Toml |

## Conventions Rust déjà appliquées

- Constantes en SCREAMING_SNAKE_CASE renommées (`sory_CLI_VERSION` → `SORY_CLI_VERSION`, etc.)
- Pas de `--jobs 1` — utilise les 2 cœurs par défaut
