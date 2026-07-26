// SPDX-License-Identifier: GPL-3.0-only

//! Page principale de chat Sory IA.
//!
//! Retourne le panneau central : TitleBar + conversation scrollable + zone de saisie.
//! Le layout 3 colonnes (sidebar + centre + workspace) est géré par main_shell.

use cosmic::{
    Element,
    iced::Length,
    widget::{self, column, container, row},
};

use crate::{
    components::{
        chat_bubble, message_input, runtime_action_card, title_bar,
    },
    events::AppEvent,
    state::AppState,
    theme::tokens,
};

pub fn view(state: &AppState) -> Element<AppEvent> {
    let conversation = state.conversations.active();

    // ── Construction des messages ──
    let mut messages = column(Vec::new()).spacing(tokens::SPACE_LG);
    let mut active_actions = column(Vec::new()).spacing(tokens::SPACE_SM);
    let mut action_count = 0usize;

    if let Some(conversation) = conversation {
        if conversation.messages.is_empty() {
            messages = messages.push(chat_bubble::empty_state());
        } else {
            for message in &conversation.messages {
                messages = messages.push(chat_bubble::view(message, state));
            }

            for action in state
                .runtime_actions
                .actions
                .iter()
                .filter(|action| action.conversation_id == conversation.id)
            {
                active_actions = active_actions.push(runtime_action_card::view(action));
                action_count += 1;
            }
        }
    }

    // ── Actions runtime (si présentes) ──
    if action_count > 0 {
        messages = messages.push(
            container(
                column(Vec::new())
                    .spacing(tokens::SPACE_SM)
                    .push(
                        widget::text::colored(
                            "Demandes runtime",
                            cosmic::palette::SORY.TEXT_SECONDARY,
                        )
                        .size(tokens::FONT_SM),
                    )
                    .push(active_actions),
            )
            .width(Length::Fill)
            .padding(tokens::SPACE_SM)
            .class(cosmic::theme::Container::default()),
        );
    }

    // ── Zone de conversation centrée ──
    let transcript = container(
        row(Vec::new())
            .push(widget::Space::new().width(Length::Fill))
            .push(
                column(Vec::new())
                    .spacing(tokens::SPACE_LG)
                    .push(messages)
                    .width(Length::Fixed(tokens::CHAT_MAX_WIDTH.into())),
            )
            .push(widget::Space::new().width(Length::Fill)),
    )
    .width(Length::Fill);

    // ── Zone de saisie centrée ──
    let composer = row(Vec::new())
        .push(widget::Space::new().width(Length::Fill))
        .push(
            column(Vec::new())
                .spacing(tokens::SPACE_SM)
                .push(message_input::view(
                    &state.draft_message,
                    state.conversations.is_generating,
                ))
                .width(Length::Fixed(tokens::INPUT_MAX_WIDTH.into())),
        )
        .push(widget::Space::new().width(Length::Fill));

    // ── Colonne centrale : titlebar + conversation + saisie ──
    let center_panel = container(
        column(Vec::new())
            .push(title_bar::view(state))
            .push(widget::scrollable(transcript).height(Length::Fill))
            .push(composer),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .class(cosmic::theme::Container::default());

    // ── Layout 3 colonnes ──
    // main_shell::view() ajoute déjà la sidebar et le right_sidebar.
    // On retourne uniquement le panneau central.
    center_panel.into()
}
