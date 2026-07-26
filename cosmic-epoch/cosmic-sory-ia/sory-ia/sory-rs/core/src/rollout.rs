use crate::config::Config;
pub use sory_rollout::ARCHIVED_SESSIONS_SUBDIR;
pub use sory_rollout::Cursor;
pub use sory_rollout::EventPersistenceMode;
pub use sory_rollout::INTERACTIVE_SESSION_SOURCES;
pub use sory_rollout::RolloutRecorder;
pub use sory_rollout::RolloutRecorderParams;
pub use sory_rollout::SESSIONS_SUBDIR;
pub use sory_rollout::SessionMeta;
pub use sory_rollout::SortDirection;
pub use sory_rollout::ThreadItem;
pub use sory_rollout::ThreadSortKey;
pub use sory_rollout::ThreadsPage;
pub use sory_rollout::append_thread_name;
pub use sory_rollout::find_archived_thread_path_by_id_str;
#[deprecated(note = "use find_thread_path_by_id_str")]
pub use sory_rollout::find_conversation_path_by_id_str;
pub use sory_rollout::find_thread_meta_by_name_str;
pub use sory_rollout::find_thread_name_by_id;
pub use sory_rollout::find_thread_names_by_ids;
pub use sory_rollout::find_thread_path_by_id_str;
pub use sory_rollout::parse_cursor;
pub use sory_rollout::read_head_for_summary;
pub use sory_rollout::read_session_meta_line;
pub use sory_rollout::rollout_date_parts;

impl sory_rollout::RolloutConfigView for Config {
    fn sory_home(&self) -> &std::path::Path {
        self.sory_home.as_path()
    }

    fn sqlite_home(&self) -> &std::path::Path {
        self.sqlite_home.as_path()
    }

    fn cwd(&self) -> &std::path::Path {
        self.cwd.as_path()
    }

    fn model_provider_id(&self) -> &str {
        self.model_provider_id.as_str()
    }

    fn generate_memories(&self) -> bool {
        self.memories.generate_memories
    }
}

pub(crate) mod list {
    pub use sory_rollout::find_thread_path_by_id_str;
}

#[cfg(test)]
pub(crate) mod recorder {
    pub use sory_rollout::RolloutRecorder;
}

pub(crate) use crate::session_rollout_init_error::map_session_init_error;

pub(crate) mod truncation {
    pub(crate) use crate::thread_rollout_truncation::*;
}
