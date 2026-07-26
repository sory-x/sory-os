use std::path::PathBuf;

use sory_utils_absolute_path::AbsolutePathBuf;

/// Runtime paths needed by exec-server child processes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecServerRuntimePaths {
    /// Stable path to the sory executable used to launch hidden helper modes.
    pub sory_self_exe: AbsolutePathBuf,
    /// Path to the Linux sandbox helper alias used when the platform sandbox
    /// needs to re-enter sory by argv0.
    pub sory_linux_sandbox_exe: Option<AbsolutePathBuf>,
}

impl ExecServerRuntimePaths {
    pub fn from_optional_paths(
        sory_self_exe: Option<PathBuf>,
        sory_linux_sandbox_exe: Option<PathBuf>,
    ) -> std::io::Result<Self> {
        let sory_self_exe = sory_self_exe.ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "sory executable path is not configured",
            )
        })?;
        Self::new(sory_self_exe, sory_linux_sandbox_exe)
    }

    pub fn new(
        sory_self_exe: PathBuf,
        sory_linux_sandbox_exe: Option<PathBuf>,
    ) -> std::io::Result<Self> {
        Ok(Self {
            sory_self_exe: absolute_path(sory_self_exe)?,
            sory_linux_sandbox_exe: sory_linux_sandbox_exe.map(absolute_path).transpose()?,
        })
    }
}

fn absolute_path(path: PathBuf) -> std::io::Result<AbsolutePathBuf> {
    AbsolutePathBuf::from_absolute_path(path.as_path())
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidInput, err))
}
