use anyhow::Result;
use predicates::str::contains;
use std::path::Path;
use tempfile::TempDir;

fn sory_command(sory_home: &Path) -> Result<assert_cmd::Command> {
    let mut cmd = assert_cmd::Command::new(sory_utils_cargo_bin::cargo_bin("sory")?);
    cmd.env("sory_HOME", sory_home);
    Ok(cmd)
}

#[cfg(debug_assertions)]
#[tokio::test]
async fn update_does_not_start_interactive_prompt() -> Result<()> {
    let sory_home = TempDir::new()?;

    sory_command(sory_home.path())?
        .arg("update")
        .assert()
        .failure()
        .stderr(contains("`sory update` is not available in debug builds"));

    Ok(())
}
