//! Exercises `ChatWidget` event handling and rendering invariants.
//!
//! These tests cover both app-server-native inputs and focused widget helpers. Many assertions are
//! snapshot-based so that layout regressions and status/header changes show up as stable,
//! reviewable diffs.

pub(super) use super::*;
pub(super) use crate::app_command::AppCommand as Op;
pub(super) use crate::app_event::AppEvent;
pub(super) use crate::app_event::ExitMode;
#[cfg(not(target_os = "linux"))]
pub(super) use crate::app_event::RealtimeAudioDeviceKind;
pub(super) use crate::app_event_sender::AppEventSender;
pub(super) use crate::approval_events::ApplyPatchApprovalRequestEvent;
pub(super) use crate::approval_events::ExecApprovalRequestEvent;
pub(super) use crate::bottom_pane::LocalImageAttachment;
pub(super) use crate::bottom_pane::MentionBinding;
pub(super) use crate::bottom_pane::QueuedInputAction;
pub(super) use crate::chatwidget::realtime::RealtimeConversationPhase;
pub(super) use crate::diff_model::FileChange;
pub(super) use crate::history_cell::UserHistoryCell;
pub(super) use crate::legacy_core::config::Config;
pub(super) use crate::legacy_core::config::ConfigBuilder;
pub(super) use crate::legacy_core::config::Constrained;
pub(super) use crate::legacy_core::config::ConstraintError;
pub(super) use crate::model_catalog::ModelCatalog;
pub(super) use crate::test_backend::VT100Backend;
pub(super) use crate::test_support::PathBufExt;
pub(super) use crate::test_support::test_path_buf;
pub(super) use crate::test_support::test_path_display;
pub(super) use crate::token_usage::TokenUsage;
pub(super) use crate::token_usage::TokenUsageInfo;
pub(super) use crate::tui::FrameRequester;
pub(super) use assert_matches::assert_matches;
pub(super) use sory_app_server_protocol::AddCreditsNudgeCreditType;
pub(super) use sory_app_server_protocol::AddCreditsNudgeEmailStatus;
pub(super) use sory_app_server_protocol::AdditionalFileSystemPermissions as AppServerAdditionalFileSystemPermissions;
pub(super) use sory_app_server_protocol::AdditionalNetworkPermissions as AppServerAdditionalNetworkPermissions;
pub(super) use sory_app_server_protocol::AdditionalPermissionProfile as AppServerAdditionalPermissionProfile;
pub(super) use sory_app_server_protocol::AppSummary;
pub(super) use sory_app_server_protocol::AutoReviewDecisionSource as AppServerGuardianApprovalReviewDecisionSource;
pub(super) use sory_app_server_protocol::SoryErrorInfo;
pub(super) use sory_app_server_protocol::CollabAgentState as AppServerCollabAgentState;
pub(super) use sory_app_server_protocol::CollabAgentStatus as AppServerCollabAgentStatus;
pub(super) use sory_app_server_protocol::CollabAgentTool as AppServerCollabAgentTool;
pub(super) use sory_app_server_protocol::CollabAgentToolCallStatus as AppServerCollabAgentToolCallStatus;
pub(super) use sory_app_server_protocol::CommandAction as AppServerCommandAction;
pub(super) use sory_app_server_protocol::CommandExecutionRequestApprovalParams as AppServerCommandExecutionRequestApprovalParams;
pub(super) use sory_app_server_protocol::CommandExecutionSource as ExecCommandSource;
pub(super) use sory_app_server_protocol::CommandExecutionSource as AppServerCommandExecutionSource;
pub(super) use sory_app_server_protocol::CommandExecutionStatus as AppServerCommandExecutionStatus;
pub(super) use sory_app_server_protocol::ConfigWarningNotification;
pub(super) use sory_app_server_protocol::CreditsSnapshot;
pub(super) use sory_app_server_protocol::ErrorNotification;
pub(super) use sory_app_server_protocol::ExecPolicyAmendment;
pub(super) use sory_app_server_protocol::FileUpdateChange;
pub(super) use sory_app_server_protocol::GuardianApprovalReview;
pub(super) use sory_app_server_protocol::GuardianApprovalReviewAction as AppServerGuardianApprovalReviewAction;
pub(super) use sory_app_server_protocol::GuardianApprovalReviewStatus;
pub(super) use sory_app_server_protocol::GuardianCommandSource as AppServerGuardianCommandSource;
pub(super) use sory_app_server_protocol::GuardianRiskLevel as AppServerGuardianRiskLevel;
pub(super) use sory_app_server_protocol::GuardianUserAuthorization as AppServerGuardianUserAuthorization;
pub(super) use sory_app_server_protocol::GuardianWarningNotification;
pub(super) use sory_app_server_protocol::HookCompletedNotification as AppServerHookCompletedNotification;
pub(super) use sory_app_server_protocol::HookEventName as AppServerHookEventName;
pub(super) use sory_app_server_protocol::HookExecutionMode as AppServerHookExecutionMode;
pub(super) use sory_app_server_protocol::HookHandlerType as AppServerHookHandlerType;
pub(super) use sory_app_server_protocol::HookOutputEntry as AppServerHookOutputEntry;
pub(super) use sory_app_server_protocol::HookOutputEntryKind as AppServerHookOutputEntryKind;
pub(super) use sory_app_server_protocol::HookRunStatus as AppServerHookRunStatus;
pub(super) use sory_app_server_protocol::HookRunSummary as AppServerHookRunSummary;
pub(super) use sory_app_server_protocol::HookScope as AppServerHookScope;
pub(super) use sory_app_server_protocol::HookStartedNotification as AppServerHookStartedNotification;
pub(super) use sory_app_server_protocol::ItemCompletedNotification;
pub(super) use sory_app_server_protocol::ItemGuardianApprovalReviewCompletedNotification;
pub(super) use sory_app_server_protocol::ItemGuardianApprovalReviewStartedNotification;
pub(super) use sory_app_server_protocol::ItemStartedNotification;
pub(super) use sory_app_server_protocol::MarketplaceAddResponse;
pub(super) use sory_app_server_protocol::MarketplaceInterface;
pub(super) use sory_app_server_protocol::MarketplaceUpgradeErrorInfo;
pub(super) use sory_app_server_protocol::MarketplaceUpgradeResponse;
pub(super) use sory_app_server_protocol::McpServerStartupState;
pub(super) use sory_app_server_protocol::McpServerStatusDetail;
pub(super) use sory_app_server_protocol::McpServerStatusUpdatedNotification;
pub(super) use sory_app_server_protocol::ModelVerification as AppServerModelVerification;
pub(super) use sory_app_server_protocol::ModelVerificationNotification;
pub(super) use sory_app_server_protocol::NonSteerableTurnKind;
pub(super) use sory_app_server_protocol::PatchApplyStatus as AppServerPatchApplyStatus;
pub(super) use sory_app_server_protocol::PatchChangeKind;
pub(super) use sory_app_server_protocol::PermissionsRequestApprovalParams as AppServerPermissionsRequestApprovalParams;
pub(super) use sory_app_server_protocol::PluginAuthPolicy;
pub(super) use sory_app_server_protocol::PluginDetail;
pub(super) use sory_app_server_protocol::PluginInstallPolicy;
pub(super) use sory_app_server_protocol::PluginInterface;
pub(super) use sory_app_server_protocol::PluginListResponse;
pub(super) use sory_app_server_protocol::PluginMarketplaceEntry;
pub(super) use sory_app_server_protocol::PluginReadResponse;
pub(super) use sory_app_server_protocol::PluginSource;
pub(super) use sory_app_server_protocol::PluginSummary;
pub(super) use sory_app_server_protocol::RateLimitReachedType;
pub(super) use sory_app_server_protocol::RateLimitSnapshot;
pub(super) use sory_app_server_protocol::RateLimitWindow;
pub(super) use sory_app_server_protocol::ReasoningSummaryTextDeltaNotification;
pub(super) use sory_app_server_protocol::ReviewTarget;
pub(super) use sory_app_server_protocol::ServerNotification;
pub(super) use sory_app_server_protocol::SkillSummary;
pub(super) use sory_app_server_protocol::ThreadClosedNotification;
pub(super) use sory_app_server_protocol::ThreadItem as AppServerThreadItem;
pub(super) use sory_app_server_protocol::ThreadRealtimeClosedNotification;
pub(super) use sory_app_server_protocol::ThreadRealtimeErrorNotification;
pub(super) use sory_app_server_protocol::ToolRequestUserInputOption;
pub(super) use sory_app_server_protocol::ToolRequestUserInputParams;
pub(super) use sory_app_server_protocol::ToolRequestUserInputQuestion;
pub(super) use sory_app_server_protocol::Turn as AppServerTurn;
pub(super) use sory_app_server_protocol::TurnCompletedNotification;
pub(super) use sory_app_server_protocol::TurnError as AppServerTurnError;
pub(super) use sory_app_server_protocol::TurnStartedNotification;
pub(super) use sory_app_server_protocol::TurnStatus as AppServerTurnStatus;
pub(super) use sory_app_server_protocol::UserInput;
pub(super) use sory_app_server_protocol::UserInput as AppServerUserInput;
pub(super) use sory_app_server_protocol::WarningNotification;
pub(super) use sory_config::AppRequirementToml;
pub(super) use sory_config::AppsRequirementsToml;
pub(super) use sory_config::ConfigLayerStack;
pub(super) use sory_config::ConfigRequirements;
pub(super) use sory_config::ConfigRequirementsToml;
pub(super) use sory_config::RequirementSource;
pub(super) use sory_config::types::ApprovalsReviewer;
pub(super) use sory_config::types::Notifications;
#[cfg(target_os = "windows")]
pub(super) use sory_config::types::WindowsSandboxModeToml;
pub(super) use sory_core_plugins::OPENAI_CURATED_MARKETPLACE_NAME;
pub(super) use sory_core_skills::model::SkillMetadata;
pub(super) use sory_features::FEATURES;
pub(super) use sory_features::Feature;
pub(super) use sory_git_utils::CommitLogEntry;
pub(super) use sory_otel::RuntimeMetricsSummary;
pub(super) use sory_otel::SessionTelemetry;
pub(super) use sory_protocol::ThreadId;
pub(super) use sory_protocol::account::PlanType;
pub(super) use sory_protocol::approvals::GuardianAssessmentAction;
pub(super) use sory_protocol::approvals::GuardianAssessmentDecisionSource;
pub(super) use sory_protocol::approvals::GuardianAssessmentEvent;
pub(super) use sory_protocol::approvals::GuardianAssessmentStatus;
pub(super) use sory_protocol::approvals::GuardianCommandSource;
pub(super) use sory_protocol::approvals::GuardianRiskLevel;
pub(super) use sory_protocol::approvals::GuardianUserAuthorization;
pub(super) use sory_protocol::config_types::CollaborationMode;
pub(super) use sory_protocol::config_types::ModeKind;
pub(super) use sory_protocol::config_types::Personality;
pub(super) use sory_protocol::config_types::ServiceTier;
pub(super) use sory_protocol::models::ActivePermissionProfile;
pub(super) use sory_protocol::models::BUILT_IN_PERMISSION_PROFILE_WORKSPACE;
pub(super) use sory_protocol::models::FileSystemPermissions;
pub(super) use sory_protocol::models::MessagePhase;
pub(super) use sory_protocol::models::NetworkPermissions;
pub(super) use sory_protocol::models::PermissionProfile;
pub(super) use sory_protocol::openai_models::ModelInfo;
pub(super) use sory_protocol::openai_models::ModelPreset;
pub(super) use sory_protocol::openai_models::ModelsResponse;
pub(super) use sory_protocol::openai_models::ReasoningEffortPreset;
pub(super) use sory_protocol::openai_models::default_input_modalities;
pub(super) use sory_protocol::parse_command::ParsedCommand;
pub(super) use sory_protocol::plan_tool::PlanItemArg;
pub(super) use sory_protocol::plan_tool::StepStatus;
pub(super) use sory_protocol::plan_tool::UpdatePlanArgs;
pub(super) use sory_protocol::request_permissions::RequestPermissionProfile;
pub(super) use sory_protocol::user_input::TextElement;
pub(super) use sory_terminal_detection::Multiplexer;
pub(super) use sory_terminal_detection::TerminalInfo;
pub(super) use sory_terminal_detection::TerminalName;
pub(super) use sory_utils_absolute_path::AbsolutePathBuf;
pub(super) use sory_utils_approval_presets::builtin_approval_presets;
pub(super) use crossterm::event::KeyCode;
pub(super) use crossterm::event::KeyEvent;
pub(super) use crossterm::event::KeyModifiers;
pub(super) use insta::assert_snapshot;
pub(super) use serde_json::json;
#[cfg(target_os = "windows")]
pub(super) use serial_test::serial;
pub(super) use std::collections::BTreeMap;
pub(super) use std::collections::HashMap;
pub(super) use std::path::PathBuf;
pub(super) use tempfile::NamedTempFile;
pub(super) use tempfile::tempdir;
pub(super) use tokio::sync::mpsc::error::TryRecvError;
pub(super) use tokio::sync::mpsc::unbounded_channel;
pub(super) use toml::Value as TomlValue;

pub(super) fn chatwidget_snapshot_dir() -> PathBuf {
    let snapshot_file = sory_utils_cargo_bin::find_resource!(
        "src/chatwidget/snapshots/sory_tui__chatwidget__tests__chatwidget_tall.snap"
    )
    .expect("snapshot file");
    snapshot_file
        .parent()
        .unwrap_or_else(|| panic!("snapshot file has no parent: {}", snapshot_file.display()))
        .to_path_buf()
}

macro_rules! assert_chatwidget_snapshot {
    ($name:expr, $value:expr $(,)?) => {{
        let mut settings = insta::Settings::clone_current();
        settings.set_prepend_module_to_snapshot(false);
        settings.set_snapshot_path(crate::chatwidget::tests::chatwidget_snapshot_dir());
        settings.bind(|| {
            insta::assert_snapshot!(format!("sory_tui__chatwidget__tests__{}", $name), $value);
        });
    }};
    ($name:expr, $value:expr, @$snapshot:literal $(,)?) => {{
        let mut settings = insta::Settings::clone_current();
        settings.set_prepend_module_to_snapshot(false);
        settings.set_snapshot_path(crate::chatwidget::tests::chatwidget_snapshot_dir());
        settings.bind(|| {
            insta::assert_snapshot!(
                format!("sory_tui__chatwidget__tests__{}", $name),
                &($value),
                @$snapshot
            );
        });
    }};
}

mod app_server;
mod approval_requests;
mod composer_submission;
mod exec_flow;
mod goal_menu;
mod goal_validation;
mod guardian;
mod helpers;
mod history_replay;
mod mcp_startup;
mod permissions;
mod plan_mode;
mod popups_and_settings;
mod review_mode;
mod side;
mod slash_commands;
mod status_and_layout;
mod status_command_tests;
mod status_surface_previews;
mod terminal_title;

pub(crate) use helpers::make_chatwidget_manual_with_sender;
pub(crate) use helpers::set_chatgpt_auth;
pub(crate) use helpers::set_fast_mode_test_catalog;
pub(super) use helpers::*;
