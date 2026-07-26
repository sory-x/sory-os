use anyhow::Result;
use predicates::prelude::PredicateBooleanExt;
use predicates::str::contains;
use sory_config::CONFIG_TOML_FILE;
use sory_config::MarketplaceConfigUpdate;
use sory_config::record_user_marketplace;
use std::path::Path;
use tempfile::TempDir;

fn sory_command(sory_home: &Path) -> Result<assert_cmd::Command> {
    let mut cmd = assert_cmd::Command::new(sory_utils_cargo_bin::cargo_bin("sory")?);
    cmd.env("sory_HOME", sory_home);
    Ok(cmd)
}

fn sory_command_in(sory_home: &Path, current_dir: &Path) -> Result<assert_cmd::Command> {
    let mut cmd = sory_command(sory_home)?;
    cmd.current_dir(current_dir);
    Ok(cmd)
}

fn configured_local_marketplace(source: &str) -> MarketplaceConfigUpdate<'_> {
    MarketplaceConfigUpdate {
        last_updated: "2026-05-06T00:00:00Z",
        last_revision: None,
        source_type: "local",
        source,
        ref_name: None,
        sparse_paths: &[],
    }
}

fn write_plugins_enabled_config(sory_home: &Path) -> Result<()> {
    std::fs::write(
        sory_home.join(CONFIG_TOML_FILE),
        r#"[features]
plugins = true
"#,
    )?;
    Ok(())
}

fn write_marketplace_source(source: &Path) -> Result<()> {
    std::fs::create_dir_all(source.join(".agents/plugins"))?;
    std::fs::create_dir_all(source.join("plugins/sample/.sory-plugin"))?;
    std::fs::write(
        source.join(".agents/plugins/marketplace.json"),
        r#"{
  "name": "debug",
  "plugins": [
    {
      "name": "sample",
      "source": {
        "source": "local",
        "path": "./plugins/sample"
      }
    }
  ]
}"#,
    )?;
    std::fs::write(
        source.join("plugins/sample/.sory-plugin/plugin.json"),
        r#"{"name":"sample","description":"Sample plugin"}"#,
    )?;
    Ok(())
}

fn setup_local_marketplace() -> Result<(TempDir, TempDir)> {
    let sory_home = TempDir::new()?;
    let source = TempDir::new()?;
    write_plugins_enabled_config(sory_home.path())?;
    write_marketplace_source(source.path())?;
    let source_path = source.path().to_string_lossy().into_owned();
    record_user_marketplace(
        sory_home.path(),
        "debug",
        &configured_local_marketplace(&source_path),
    )?;
    Ok((sory_home, source))
}

fn setup_unconfigured_local_marketplace() -> Result<(TempDir, TempDir)> {
    let sory_home = TempDir::new()?;
    let source = TempDir::new()?;
    write_plugins_enabled_config(sory_home.path())?;
    write_marketplace_source(source.path())?;
    Ok((sory_home, source))
}

fn setup_configured_marketplace_without_manifest() -> Result<(TempDir, TempDir)> {
    let sory_home = TempDir::new()?;
    let source = TempDir::new()?;
    write_plugins_enabled_config(sory_home.path())?;
    let source_path = source.path().to_string_lossy().into_owned();
    record_user_marketplace(
        sory_home.path(),
        "debug",
        &configured_local_marketplace(&source_path),
    )?;
    Ok((sory_home, source))
}

fn setup_configured_marketplace_with_malformed_manifest() -> Result<(TempDir, TempDir)> {
    let sory_home = TempDir::new()?;
    let source = TempDir::new()?;
    write_plugins_enabled_config(sory_home.path())?;
    std::fs::create_dir_all(source.path().join(".agents/plugins"))?;
    std::fs::write(
        source.path().join(".agents/plugins/marketplace.json"),
        "{not valid json",
    )?;
    let source_path = source.path().to_string_lossy().into_owned();
    record_user_marketplace(
        sory_home.path(),
        "debug",
        &configured_local_marketplace(&source_path),
    )?;
    Ok((sory_home, source))
}

#[tokio::test]
async fn marketplace_list_shows_configured_marketplace_names() -> Result<()> {
    let (sory_home, source) = setup_local_marketplace()?;

    sory_command(sory_home.path())?
        .args(["plugin", "marketplace", "list"])
        .assert()
        .success()
        .stdout(contains("debug"))
        .stdout(contains(source.path().display().to_string()));

    Ok(())
}

#[tokio::test]
async fn plugin_list_shows_plugins_grouped_by_marketplace() -> Result<()> {
    let (sory_home, _source) = setup_local_marketplace()?;

    sory_command(sory_home.path())?
        .args(["plugin", "list"])
        .assert()
        .success()
        .stdout(contains("Marketplace `debug`"))
        .stdout(contains("sample@debug (not installed)"));

    Ok(())
}

#[tokio::test]
async fn plugin_list_excludes_unconfigured_repo_local_marketplaces() -> Result<()> {
    let (sory_home, source) = setup_unconfigured_local_marketplace()?;

    sory_command_in(sory_home.path(), source.path())?
        .args(["plugin", "list"])
        .assert()
        .success()
        .stdout(contains("No marketplace plugins found."))
        .stdout(predicates::str::is_match("sample@debug").unwrap().not());

    Ok(())
}

#[tokio::test]
async fn plugin_list_fails_when_configured_marketplace_snapshot_is_missing() -> Result<()> {
    let (sory_home, source) = setup_configured_marketplace_without_manifest()?;

    sory_command(sory_home.path())?
        .args(["plugin", "list"])
        .assert()
        .failure()
        .stderr(contains(
            "failed to load configured marketplace snapshot(s):",
        ))
        .stderr(contains("`debug`"))
        .stderr(contains(source.path().display().to_string()))
        .stderr(contains(
            "marketplace root does not contain a supported manifest",
        ));

    Ok(())
}

#[tokio::test]
async fn plugin_add_and_remove_updates_installed_plugin_config() -> Result<()> {
    let (sory_home, _source) = setup_local_marketplace()?;

    sory_command(sory_home.path())?
        .args(["plugin", "add", "sample@debug"])
        .assert()
        .success()
        .stdout(contains("Added plugin `sample` from marketplace `debug`."));

    let config = std::fs::read_to_string(sory_home.path().join(CONFIG_TOML_FILE))?;
    assert!(config.contains("[plugins.\"sample@debug\"]"));

    sory_command(sory_home.path())?
        .args(["plugin", "remove", "sample", "--marketplace", "debug"])
        .assert()
        .success()
        .stdout(contains(
            "Removed plugin `sample` from marketplace `debug`.",
        ));

    let config = std::fs::read_to_string(sory_home.path().join(CONFIG_TOML_FILE))?;
    assert!(!config.contains("[plugins.\"sample@debug\"]"));

    Ok(())
}

#[tokio::test]
async fn plugin_add_rejects_unconfigured_repo_local_marketplaces() -> Result<()> {
    let (sory_home, source) = setup_unconfigured_local_marketplace()?;

    sory_command_in(sory_home.path(), source.path())?
        .args(["plugin", "add", "sample@debug"])
        .assert()
        .failure()
        .stderr(contains(
            "plugin `sample` was not found in marketplace `debug`",
        ));

    Ok(())
}

#[tokio::test]
async fn plugin_add_fails_when_configured_marketplace_snapshot_is_malformed() -> Result<()> {
    let (sory_home, _source) = setup_configured_marketplace_with_malformed_manifest()?;

    sory_command(sory_home.path())?
        .args(["plugin", "add", "sample@debug"])
        .assert()
        .failure()
        .stderr(contains(
            "failed to load configured marketplace snapshot(s):",
        ))
        .stderr(contains("`debug`"))
        .stderr(contains("invalid marketplace file"))
        .stderr(contains("key must be a string"));

    Ok(())
}

#[tokio::test]
async fn plugin_add_reinstalls_from_configured_marketplace_snapshot() -> Result<()> {
    let (sory_home, _source) = setup_local_marketplace()?;

    sory_command(sory_home.path())?
        .args(["plugin", "add", "sample@debug"])
        .assert()
        .success();

    sory_command(sory_home.path())?
        .args(["plugin", "add", "sample@debug"])
        .assert()
        .success()
        .stdout(contains("Added plugin `sample` from marketplace `debug`."));

    assert!(
        sory_home
            .path()
            .join("plugins/cache/debug/sample/local/.sory-plugin/plugin.json")
            .is_file()
    );

    Ok(())
}

#[tokio::test]
async fn plugin_remove_works_after_marketplace_is_removed() -> Result<()> {
    let (sory_home, _source) = setup_local_marketplace()?;

    sory_command(sory_home.path())?
        .args(["plugin", "add", "sample", "--marketplace", "debug"])
        .assert()
        .success();

    sory_command(sory_home.path())?
        .args(["plugin", "marketplace", "remove", "debug"])
        .assert()
        .success();

    sory_command(sory_home.path())?
        .args(["plugin", "remove", "sample@debug"])
        .assert()
        .success()
        .stdout(contains(
            "Removed plugin `sample` from marketplace `debug`.",
        ));

    let config = std::fs::read_to_string(sory_home.path().join(CONFIG_TOML_FILE))?;
    assert!(!config.contains("[plugins.\"sample@debug\"]"));

    Ok(())
}

#[tokio::test]
async fn plugin_add_rejects_cached_plugins_without_authorizing_marketplace_snapshot() -> Result<()>
{
    let (sory_home, _source) = setup_local_marketplace()?;

    sory_command(sory_home.path())?
        .args(["plugin", "add", "sample@debug"])
        .assert()
        .success();

    sory_command(sory_home.path())?
        .args(["plugin", "marketplace", "remove", "debug"])
        .assert()
        .success();

    assert!(
        sory_home
            .path()
            .join("plugins/cache/debug/sample/local/.sory-plugin/plugin.json")
            .is_file()
    );

    sory_command(sory_home.path())?
        .args(["plugin", "add", "sample@debug"])
        .assert()
        .failure()
        .stderr(contains(
            "plugin `sample` was not found in marketplace `debug`",
        ));

    Ok(())
}
