pub const TOOL_CALL_COUNT_METRIC: &str = "sory.tool.call";
pub const TOOL_CALL_DURATION_METRIC: &str = "sory.tool.call.duration_ms";
pub const TOOL_CALL_UNIFIED_EXEC_METRIC: &str = "sory.tool.unified_exec";
pub const PROCESS_START_METRIC: &str = "sory.process.start";
pub const API_CALL_COUNT_METRIC: &str = "sory.api_request";
pub const API_CALL_DURATION_METRIC: &str = "sory.api_request.duration_ms";
pub const SSE_EVENT_COUNT_METRIC: &str = "sory.sse_event";
pub const SSE_EVENT_DURATION_METRIC: &str = "sory.sse_event.duration_ms";
pub const WEBSOCKET_REQUEST_COUNT_METRIC: &str = "sory.websocket.request";
pub const WEBSOCKET_REQUEST_DURATION_METRIC: &str = "sory.websocket.request.duration_ms";
pub const WEBSOCKET_EVENT_COUNT_METRIC: &str = "sory.websocket.event";
pub const WEBSOCKET_EVENT_DURATION_METRIC: &str = "sory.websocket.event.duration_ms";
pub const RESPONSES_API_OVERHEAD_DURATION_METRIC: &str = "sory.responses_api_overhead.duration_ms";
pub const RESPONSES_API_INFERENCE_TIME_DURATION_METRIC: &str =
    "sory.responses_api_inference_time.duration_ms";
pub const RESPONSES_API_ENGINE_IAPI_TTFT_DURATION_METRIC: &str =
    "sory.responses_api_engine_iapi_ttft.duration_ms";
pub const RESPONSES_API_ENGINE_SERVICE_TTFT_DURATION_METRIC: &str =
    "sory.responses_api_engine_service_ttft.duration_ms";
pub const RESPONSES_API_ENGINE_IAPI_TBT_DURATION_METRIC: &str =
    "sory.responses_api_engine_iapi_tbt.duration_ms";
pub const RESPONSES_API_ENGINE_SERVICE_TBT_DURATION_METRIC: &str =
    "sory.responses_api_engine_service_tbt.duration_ms";
pub const TURN_E2E_DURATION_METRIC: &str = "sory.turn.e2e_duration_ms";
pub const TURN_TTFT_DURATION_METRIC: &str = "sory.turn.ttft.duration_ms";
pub const TURN_TTFM_DURATION_METRIC: &str = "sory.turn.ttfm.duration_ms";
pub const TURN_NETWORK_PROXY_METRIC: &str = "sory.turn.network_proxy";
pub const TURN_MEMORY_METRIC: &str = "sory.turn.memory";
pub const TURN_TOOL_CALL_METRIC: &str = "sory.turn.tool.call";
pub const TURN_TOKEN_USAGE_METRIC: &str = "sory.turn.token_usage";
pub const GOAL_CREATED_METRIC: &str = "sory.goal.created";
pub const GOAL_COMPLETED_METRIC: &str = "sory.goal.completed";
pub const GOAL_BUDGET_LIMITED_METRIC: &str = "sory.goal.budget_limited";
pub const GOAL_TOKEN_COUNT_METRIC: &str = "sory.goal.token_count";
pub const GOAL_DURATION_SECONDS_METRIC: &str = "sory.goal.duration_s";
pub const PROFILE_USAGE_METRIC: &str = "sory.profile.usage";
pub const CURATED_PLUGINS_STARTUP_SYNC_METRIC: &str = "sory.plugins.startup_sync";
pub const CURATED_PLUGINS_STARTUP_SYNC_FINAL_METRIC: &str = "sory.plugins.startup_sync.final";
pub const HOOK_RUN_METRIC: &str = "sory.hooks.run";
pub const HOOK_RUN_DURATION_METRIC: &str = "sory.hooks.run.duration_ms";
/// Duration for coarse startup phases, tagged by low-cardinality phase and status.
pub const STARTUP_PHASE_DURATION_METRIC: &str = "sory.startup.phase.duration_ms";
/// Total runtime of a startup prewarm attempt until it completes, tagged by final status.
pub const STARTUP_PREWARM_DURATION_METRIC: &str = "sory.startup_prewarm.duration_ms";
/// Age of the startup prewarm attempt when the first real turn resolves it, tagged by outcome.
pub const STARTUP_PREWARM_AGE_AT_FIRST_TURN_METRIC: &str =
    "sory.startup_prewarm.age_at_first_turn_ms";
pub const THREAD_STARTED_METRIC: &str = "sory.thread.started";
pub const THREAD_SKILLS_ENABLED_TOTAL_METRIC: &str = "sory.thread.skills.enabled_total";
pub const THREAD_SKILLS_KEPT_TOTAL_METRIC: &str = "sory.thread.skills.kept_total";
pub const THREAD_SKILLS_DESCRIPTION_TRUNCATED_CHARS_METRIC: &str =
    "sory.thread.skills.description_truncated_chars";
pub const THREAD_SKILLS_TRUNCATED_METRIC: &str = "sory.thread.skills.truncated";
