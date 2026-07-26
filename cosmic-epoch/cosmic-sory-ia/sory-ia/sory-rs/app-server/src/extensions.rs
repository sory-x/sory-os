use std::sync::Arc;
use std::sync::Weak;

use sory_core::NewThread;
use sory_core::StartThreadOptions;
use sory_core::ThreadManager;
use sory_core::config::Config;
use sory_extension_api::AgentSpawnFuture;
use sory_extension_api::AgentSpawner;
use sory_extension_api::ExtensionRegistry;
use sory_extension_api::ExtensionRegistryBuilder;
use sory_protocol::ThreadId;
use sory_protocol::error::SoryErr;

pub(crate) fn thread_extensions<S>(guardian_agent_spawner: S) -> Arc<ExtensionRegistry<Config>>
where
    S: AgentSpawner<StartThreadOptions, Spawned = NewThread, Error = SoryErr> + 'static,
{
    let mut builder = ExtensionRegistryBuilder::<Config>::new();
    sory_guardian::install(&mut builder, guardian_agent_spawner);
    sory_memories_extension::install(&mut builder);
    Arc::new(builder.build())
}

pub(crate) fn guardian_agent_spawner(
    thread_manager: Weak<ThreadManager>,
) -> impl AgentSpawner<StartThreadOptions, Spawned = NewThread, Error = SoryErr> {
    move |forked_from_thread_id: ThreadId,
          options: StartThreadOptions|
          -> AgentSpawnFuture<'static, NewThread, SoryErr> {
        let thread_manager = thread_manager.clone();
        Box::pin(async move {
            let thread_manager = thread_manager.upgrade().ok_or_else(|| {
                SoryErr::UnsupportedOperation("thread manager dropped".to_string())
            })?;
            thread_manager
                .spawn_subagent(forked_from_thread_id, options)
                .await
        })
    }
}
