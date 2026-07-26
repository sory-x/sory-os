use std::path::Path;

use anyhow::Result;
use predicates::str::contains;
use tempfile::TempDir;

fn sory_command(sory_home: &Path) -> Result<assert_cmd::Command> {
    let mut cmd = assert_cmd::Command::new(sory_utils_cargo_bin::cargo_bin("sory")?);
    cmd.env("sory_HOME", sory_home);
    Ok(cmd)
}

#[test]
fn strict_config_rejects_unknown_config_fields_for_app_server() -> Result<()> {
    let sory_home = TempDir::new()?;
    std::fs::write(
        sory_home.path().join("config.toml"),
        r#"
foo = "bar"
"#,
    )?;

    let mut cmd = sory_command(sory_home.path())?;
    cmd.args(["app-server", "--strict-config", "--listen", "off"])
        .assert()
        .failure()
        .stderr(contains("unknown configuration field"));

    Ok(())
}
