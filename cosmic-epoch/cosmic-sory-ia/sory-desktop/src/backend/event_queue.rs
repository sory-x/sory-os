// SPDX-License-Identifier: GPL-3.0-only

use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

use super::BackendEvent;

const HEARTBEAT_TIMEOUT: Duration = Duration::from_secs(15);

/// Petite file d'événements inspirée du modèle OpenCode.
///
/// Elle garde les événements critiques et coalesce les deltas de texte
/// consécutifs d'une même conversation pour réduire la pression sur l'UI.
#[derive(Debug)]
pub struct BackendEventQueue {
    pending: VecDeque<BackendEvent>,
    last_event_at: Instant,
}

impl Default for BackendEventQueue {
    fn default() -> Self {
        Self {
            pending: VecDeque::new(),
            last_event_at: Instant::now(),
        }
    }
}

impl BackendEventQueue {
    pub fn push(&mut self, event: BackendEvent) {
        self.last_event_at = Instant::now();

        if let BackendEvent::Token {
            conversation_id,
            token,
        } = &event
        {
            if let Some(BackendEvent::Token {
                conversation_id: previous_id,
                token: previous_token,
            }) = self.pending.back_mut()
            {
                if previous_id == conversation_id {
                    previous_token.push_str(token);
                    return;
                }
            }
        }

        self.pending.push_back(event);
    }

    pub fn drain(&mut self) -> impl Iterator<Item = BackendEvent> + '_ {
        self.pending.drain(..)
    }

    pub fn heartbeat_expired(&self) -> bool {
        self.last_event_at.elapsed() > HEARTBEAT_TIMEOUT
    }
}
