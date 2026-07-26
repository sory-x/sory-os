// SPDX-License-Identifier: GPL-3.0-only

use crate::{backend::BackendEvent, events::AppEvent};

use super::ApplicationState;

/// Applique les événements normalisés du runtime Sory IA à l'état applicatif.
///
/// Cette couche joue le rôle du reducer central observé dans OpenCode : l'UI ne
/// connaît pas le protocole runtime et l'application ne disperse pas le mapping
/// des événements dans les composants.
impl ApplicationState {
    pub fn apply_backend_event(&mut self, event: &BackendEvent) {
        match event {
            BackendEvent::Connected => self.reduce(&AppEvent::BackendConnected),
            BackendEvent::Disconnected => self.reduce(&AppEvent::BackendDisconnected),
            BackendEvent::Reconnecting => self.reduce(&AppEvent::BackendReconnecting),
            BackendEvent::ConversationLinked {
                conversation_id,
                thread_id,
            } => {
                if let Some(conversation) = self
                    .conversations
                    .conversations
                    .iter_mut()
                    .find(|conversation| conversation.id == *conversation_id)
                {
                    conversation.runtime_thread_id = Some(thread_id.clone());
                }
            }
            BackendEvent::Token {
                conversation_id,
                token,
            } => {
                self.reduce(&AppEvent::ReceiveToken {
                    conversation_id: conversation_id.unwrap_or(self.conversations.active_id),
                    token: token.clone(),
                });
            }
            BackendEvent::ToolStarted {
                conversation_id,
                name,
            } => self.reduce(&AppEvent::ToolStarted {
                conversation_id: conversation_id.unwrap_or(self.conversations.active_id),
                name: name.clone(),
            }),
            BackendEvent::ToolFinished {
                conversation_id,
                name,
            } => self.reduce(&AppEvent::ToolFinished {
                conversation_id: conversation_id.unwrap_or(self.conversations.active_id),
                name: name.clone(),
            }),
            BackendEvent::PermissionRequested {
                conversation_id,
                title,
                details,
                request_id,
                thread_id,
                turn_id,
                item_id,
            } => self.reduce(&AppEvent::PermissionRequested {
                conversation_id: conversation_id.unwrap_or(self.conversations.active_id),
                title: title.clone(),
                details: details.clone(),
                request_id: request_id.clone(),
                thread_id: thread_id.clone(),
                turn_id: turn_id.clone(),
                item_id: item_id.clone(),
            }),
            BackendEvent::QuestionAsked {
                conversation_id,
                prompt,
                details,
                request_id,
                thread_id,
                turn_id,
                item_id,
            } => self.reduce(&AppEvent::QuestionAsked {
                conversation_id: conversation_id.unwrap_or(self.conversations.active_id),
                prompt: prompt.clone(),
                details: details.clone(),
                request_id: request_id.clone(),
                thread_id: thread_id.clone(),
                turn_id: turn_id.clone(),
                item_id: item_id.clone(),
            }),
            BackendEvent::ToolApprovalRequested {
                conversation_id,
                tool,
                details,
                request_id,
                thread_id,
                turn_id,
                item_id,
            } => self.reduce(&AppEvent::ToolApprovalRequested {
                conversation_id: conversation_id.unwrap_or(self.conversations.active_id),
                tool: tool.clone(),
                details: details.clone(),
                request_id: request_id.clone(),
                thread_id: thread_id.clone(),
                turn_id: turn_id.clone(),
                item_id: item_id.clone(),
            }),
            BackendEvent::AgentStep {
                conversation_id,
                label,
            } => self.reduce(&AppEvent::AgentStep {
                conversation_id: conversation_id.unwrap_or(self.conversations.active_id),
                label: label.clone(),
            }),
            BackendEvent::AgentFinished {
                conversation_id, ..
            } => self.reduce(&AppEvent::AgentFinished {
                conversation_id: conversation_id.unwrap_or(self.conversations.active_id),
            }),
            BackendEvent::HealthCheckPassed => {
                self.status = "Health check réussi — runtime opérationnel".into();
            }
            BackendEvent::HealthCheckFailed { message } => {
                self.status = format!("Health check échoué : {message}");
                // Suite à un échec health check, on considère la connexion perdue
                self.reduce(&AppEvent::BackendReconnecting);
            }
            BackendEvent::Progress { message } => self.status = message.clone(),
            BackendEvent::Warning { message } => {
                self.reduce(&AppEvent::BackendWarning(message.clone()));
            }
            BackendEvent::Error { message } => {
                let human = crate::platform::provider_auth::humanize_runtime_error(message);
                self.reduce(&AppEvent::BackendError(human));
            }
        }
    }
}
