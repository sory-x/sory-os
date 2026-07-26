pub(crate) use sory_skills::install_system_skills;
pub(crate) use sory_skills::system_cache_root_dir;

use sory_utils_absolute_path::AbsolutePathBuf;

pub(crate) fn uninstall_system_skills(sory_home: &AbsolutePathBuf) {
    let _ = std::fs::remove_dir_all(system_cache_root_dir(sory_home));
}
