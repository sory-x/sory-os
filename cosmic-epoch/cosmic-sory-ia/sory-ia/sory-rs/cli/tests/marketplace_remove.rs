use anyhow::Result;
use predicates::str::contains;
use sory_config::MarketplaceConfigUpdate;
use sory_config::record_user_marketplace;
use sory_core_plugins::installed_marketplaces::marketplace_install_root;
use std::path::Path;
use tempfile::TempDir;

fn sory_command(sory_home: &Path) -> Result<assert_cmd::Command> {
    let mut cmd = assert_cmd::Command::new(sory_utils_cargo_bin::cargo_bin("sory")?);
    cmd.env("sory_HOME", sory_home);
    Ok(cmd)
}

fn configured_marketplace_update() -> MarketplaceConfigUpdate<'static> {
    MarketplaceConfigUpdate {
        last_updated: "2026-04-13T00:00:00Z",
        last_revision: None,
        source_type: "git",
        source: "https://github.com/owner/repo.git",
        ref_name: Some("main"),
        sparse_paths: &[],
    }
}

fn write_installed_marketplace(sory_home: &Path, marketplace_name: &str) -> Result<()> {
    let root = marketplace_install_root(sory_home).join(marketplace_name);
    std::fs::create_dir_all(root.join(".agents/plugins"))?;
    std::fs::write(root.join(".agents/plugins/marketplace.json"), "{}")?;
    std::fs::write(root.join("marker.txt"), "installed")?;
    Ok(())
}

#[tokio::test]
async fn marketplace_remove_deletes_config_and_installed_root() -> Result<()> {
    let sory_home = TempDir::new()?;
    record_user_marketplace(sory_home.path(), "debug", &configured_marketplace_update())?;
    write_installed_marketplace(sory_home.path(), "debug")?;

    sory_command(sory_home.path())?
        .args(["plugin", "marketplace", "remove", "debug"])
        .assert()
        .success()
        .stdout(contains("Removed marketplace `debug`."));

    let config_path = sory_home.path().join("config.toml");
    let config = std::fs::read_to_string(config_path)?;
    assert!(!config.contains("[marketplaces.debug]"));
    assert!(
        !marketplace_install_root(sory_home.path())
            .join("debug")
            .exists()
    );
    Ok(())
}

#[tokio::test]
async fn marketplace_remove_rejects_unknown_marketplace() -> Result<()> {
    let sory_home = TempDir::new()?;

    sory_command(sory_home.path())?
        .args(["plugin", "marketplace", "remove", "debug"])
        .assert()
        .failure()
        .stderr(contains(
            "marketplace `debug` is not configured or installed",
        ));

    Ok(())
}
