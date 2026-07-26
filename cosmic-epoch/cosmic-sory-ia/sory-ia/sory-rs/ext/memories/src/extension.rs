use std::sync::Arc;

use sory_core::config::Config;
use sory_extension_api::ConfigContributor;
use sory_extension_api::ContextContributor;
use sory_extension_api::ExtensionData;
use sory_extension_api::ExtensionRegistryBuilder;
use sory_extension_api::PromptFragment;
use sory_extension_api::ThreadLifecycleContributor;
use sory_extension_api::ThreadStartInput;
use sory_extension_api::ToolContributor;
use sory_features::Feature;
use sory_memories_read::build_memory_tool_developer_instructions;
use sory_utils_absolute_path::AbsolutePathBuf;

use crate::local::LocalMemoriesBackend;
use crate::tools;

/// Contributes sory memory read-path prompt context and memory read tools.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct MemoriesExtension;

#[derive(Clone, Debug)]
pub(crate) struct MemoriesExtensionConfig {
    pub(crate) enabled: bool,
    pub(crate) sory_home: AbsolutePathBuf,
}

impl MemoriesExtensionConfig {
    fn from_config(config: &Config) -> Self {
        Self {
            enabled: config.features.enabled(Feature::MemoryTool) && config.memories.use_memories,
            sory_home: config.sory_home.clone(),
        }
    }
}

impl ContextContributor for MemoriesExtension {
    fn contribute<'a>(
        &'a self,
        _session_store: &'a ExtensionData,
        thread_store: &'a ExtensionData,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Vec<PromptFragment>> + Send + 'a>> {
        Box::pin(async move {
            let Some(config) = thread_store.get::<MemoriesExtensionConfig>() else {
                return Vec::new();
            };
            if !config.enabled {
                return Vec::new();
            }

            build_memory_tool_developer_instructions(&config.sory_home)
                .await
                .map(PromptFragment::developer_policy)
                .into_iter()
                .collect()
        })
    }
}

impl ThreadLifecycleContributor<Config> for MemoriesExtension {
    fn on_thread_start(&self, input: ThreadStartInput<'_, Config>) {
        input
            .thread_store
            .insert(MemoriesExtensionConfig::from_config(input.config));
    }
}

impl ConfigContributor<Config> for MemoriesExtension {
    fn on_config_changed(
        &self,
        _session_store: &ExtensionData,
        thread_store: &ExtensionData,
        _previous_config: &Config,
        new_config: &Config,
    ) {
        thread_store.insert(MemoriesExtensionConfig::from_config(new_config));
    }
}

impl ToolContributor for MemoriesExtension {
    fn tools(
        &self,
        _session_store: &ExtensionData,
        thread_store: &ExtensionData,
    ) -> Vec<Arc<dyn sory_extension_api::ToolExecutor<sory_extension_api::ToolCall>>> {
        let Some(config) = thread_store.get::<MemoriesExtensionConfig>() else {
            return Vec::new();
        };
        if !config.enabled {
            return Vec::new();
        }

        tools::memory_tools(LocalMemoriesBackend::from_sory_home(&config.sory_home))
    }
}

/// Installs the memories extension contributors into the extension registry.
pub fn install(registry: &mut ExtensionRegistryBuilder<Config>) {
    let extension = Arc::new(MemoriesExtension);
    registry.thread_lifecycle_contributor(extension.clone());
    registry.config_contributor(extension.clone());
    registry.prompt_contributor(extension);
    // Keep the read/retrieval tools out of app-server until that rollout is intentional.
    // registry.tool_contributor(extension);
}
