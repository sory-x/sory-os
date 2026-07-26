// SPDX-License-Identifier: GPL-3.0-only

//! Barre de statut en bas du chat — style "Deep Navy Glass".
//!
//! Affiche l'état du runtime avec icône colorée et indicateur de streaming.

use cosmic::{
    Element,
    iced::{Alignment, Length},
    widget::{self, Space, container, row},
};

use crate::{events::AppEvent, theme::tokens};

pub fn view(status: &str) -> Element<AppEvent> {
    // Détection basique de l'état pour colorer l'icône
    let (icon, color) = if status.contains("rr\u{00e9}") || status.contains("Erreur") {
        ("\u{26a0}", cosmic::palette::SORY.ACCENT_RED)
    } else if status.contains("n cours") || status.contains("f\u{00e9}l\u{00e9}chit") {
        ("\u{21bb}", cosmic::palette::SORY.ACCENT_BRIGHT)
    } else if status.contains("onnect") || status.contains("Pr\u{00ea}t") {
        ("\u{25cf}", cosmic::palette::SORY.ACCENT_GREEN)
    } else {
        ("\u{25cb}", cosmic::palette::SORY.TEXT_MUTED)
    };

    container(
        row(Vec::new())
            .spacing(tokens::SPACE_SM)
            .align_y(Alignment::Center)
            .push(widget::text::colored(icon, color).size(f32::from(tokens::FONT_SM)))
            .push(
                widget::text(status)
                    .size(f32::from(tokens::FONT_XS))
                    .font(cosmic::font::default()),
            )
            .push(Space::new().width(Length::Fill))
            .push(
                widget::text("Sory IA")
                    .size(f32::from(tokens::FONT_XS))
                    .font(cosmic::font::default()),
            ),
    )
    .width(Length::Fill)
    .padding([tokens::SPACE_XS, tokens::SPACE_SM])
    .class(cosmic::theme::sory::status_bar())
    .into()
}
