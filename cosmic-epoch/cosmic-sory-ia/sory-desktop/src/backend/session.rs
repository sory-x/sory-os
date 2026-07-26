// SPDX-License-Identifier: GPL-3.0-only

use std::collections::HashMap;

use uuid::Uuid;

/// Table de correspondance entre les conversations affichées par Sory IA et
/// les threads/sessions possédés par le runtime.
///
/// Le Desktop conserve ses identifiants stables pour l'UI, mais le runtime reste
/// propriétaire de la session réelle.
#[derive(Debug, Default)]
pub struct RuntimeSessionMap {
    by_conversation: HashMap<Uuid, RuntimeSessionLink>,
}

#[derive(Debug, Clone)]
pub struct RuntimeSessionLink {
    pub thread_id: String,
    pub active_turn_id: Option<String>,
}

impl RuntimeSessionMap {
    pub fn get_thread(&self, conversation_id: Uuid) -> Option<&str> {
        self.by_conversation
            .get(&conversation_id)
            .map(|link| link.thread_id.as_str())
    }

    pub fn link_thread(&mut self, conversation_id: Uuid, thread_id: impl Into<String>) {
        self.by_conversation.insert(
            conversation_id,
            RuntimeSessionLink {
                thread_id: thread_id.into(),
                active_turn_id: None,
            },
        );
    }

    pub fn set_active_turn(&mut self, conversation_id: Uuid, turn_id: impl Into<String>) {
        if let Some(link) = self.by_conversation.get_mut(&conversation_id) {
            link.active_turn_id = Some(turn_id.into());
        }
    }

    pub fn active_turn(&self, conversation_id: Uuid) -> Option<&str> {
        self.by_conversation
            .get(&conversation_id)
            .and_then(|link| link.active_turn_id.as_deref())
    }
}
