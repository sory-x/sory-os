// SPDX-License-Identifier: GPL-3.0-only

//! Barre d'outils du workspace — style "Deep Navy Glass".

use cosmic::{
    Element,
    iced::{Alignment, Length},
    widget::{self, container, row},
};

use crate::{
    events::AppEvent,
    state::{AppState, RuntimeStatus},
    theme::tokens,
};

pub fn view(state: &AppState) -> Element<AppEvent> {
    let (icon, connection, conn_color) = match state.runtime_status {
        RuntimeStatus::Disconnected => (
            "\u{25cb}",
            "D\u{00e9}connect\u{00e9}",
            cosmic::palette::SORY.TEXT_MUTED,
        ),
        RuntimeStatus::Connecting => (
            "\u{25cc}",
            "Connexion\u{2026}",
            cosmic::palette::SORY.ACCENT_ORANGE,
        ),
        RuntimeStatus::HealthChecking => (
            "\u{25c9}",
            "V\u{00e9}rification\u{2026}",
            cosmic::palette::SORY.ACCENT_ORANGE,
        ),
        RuntimeStatus::Ready => (
            "\u{25cf}",
            "Pr\u{00ea}t",
            cosmic::palette::SORY.ACCENT_GREEN,
        ),
        RuntimeStatus::Reconnecting => (
            "\u{21bb}",
            "Reconnexion\u{2026}",
            cosmic::palette::SORY.ACCENT_ORANGE,
        ),
        RuntimeStatus::Failed => ("\u{26a0}", "Erreur", cosmic::palette::SORY.ACCENT_RED),
    };

    let provider = state
        .providers
        .definitions
        .iter()
        .find(|d| d.id == state.providers.active_provider_id)
        .map(|d| d.name.as_str())
        .unwrap_or(&state.providers.active_provider_id);
    let model: &str = state
        .settings
        .settings
        .provider_configs
        .get(&state.settings.settings.provider_id)
        .map(|c| c.model.as_str())
        .filter(|m| !m.is_empty())
        .unwrap_or("auto");

    container(
        row(Vec::new())
            .spacing(tokens::SPACE_MD)
            .align_y(Alignment::Center)
            // Titre
            .push(
                widget::text(&state.window.title)
                    .size(f32::from(tokens::FONT_XL))
                    .font(cosmic::font::semibold()),
            )
            // Statut connexion
            .push(
                row(Vec::new())
                    .spacing(tokens::SPACE_XS)
                    .align_y(Alignment::Center)
                    .push(widget::text::colored(icon, conn_color).size(f32::from(tokens::FONT_SM)))
                    .push(
                        widget::text::colored(connection, conn_color)
                            .size(f32::from(tokens::FONT_SM))
                            .font(cosmic::font::default()),
                    ),
            )
            .push(widget::Space::new().width(Length::Fill))
            // Provider + modèle
            .push(
                widget::text::colored(
                    format!("Provider : {provider}"),
                    cosmic::palette::SORY.TEXT_SECONDARY,
                )
                .size(f32::from(tokens::FONT_SM))
                .font(cosmic::font::default()),
            )
            .push(
                widget::text::colored(
                    format!("Mod\u{00e8}le : {model}"),
                    cosmic::palette::SORY.TEXT_SECONDARY,
                )
                .size(f32::from(tokens::FONT_SM))
                .font(cosmic::font::default()),
            ),
    )
    .width(Length::Fill)
    .padding([tokens::SPACE_XS, tokens::SPACE_MD])
    .class(cosmic::theme::sory::header_bar())
    .into()
}
