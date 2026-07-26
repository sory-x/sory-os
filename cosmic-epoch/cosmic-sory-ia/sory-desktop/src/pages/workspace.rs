// SPDX-License-Identifier: GPL-3.0-only

use cosmic::{
    Element,
    iced::{Alignment, Length},
    widget::{self, button, column, container, row},
};

use crate::{
    components::page_header,
    events::AppEvent,
    state::AppState,
    theme::tokens,
};

pub fn view(state: &AppState) -> Element<AppEvent> {
    let active = state.workspace.active.as_ref();

    column(Vec::new())
        .spacing(tokens::SPACE_MD)
        .width(Length::Fill)
        .push(page_header::view(
            "Workspace",
            Some("G\u{00e9}rez votre espace de travail actif"),
        ))
        .push(widget::divider::horizontal::default())
        .push(
            container(
                column(Vec::new())
                    .spacing(tokens::SPACE_LG)
                    .push(
                        container(
                            row(Vec::new())
                                .spacing(tokens::SPACE_SM)
                                .align_y(Alignment::Center)
                                .push(
                                    widget::text::colored(
                                        "\u{1f4c1}",
                                        cosmic::palette::SORY.ACCENT,
                                    )
                                    .size(tokens::FONT_XXL),
                                )
                                .push(
                                    column(Vec::new())
                                        .spacing(tokens::SPACE_XXS)
                                        .push(
                                            widget::text(
                                                active
                                                    .map(|w| w.name.as_str())
                                                    .unwrap_or("Aucun workspace"),
                                            )
                                            .size(tokens::FONT_XL)
                                            .font(cosmic::font::bold()),
                                        )
                                        .push(
                                            widget::text::colored(
                                                active
                                                    .and_then(|w| w.path.as_deref())
                                                    .unwrap_or("Aucun dossier ouvert"),
                                                cosmic::palette::SORY.TEXT_MUTED,
                                            )
                                            .size(tokens::FONT_SM),
                                        ),
                                ),
                        )
                        .padding(tokens::CARD_PADDING)
                        .class(cosmic::theme::sory::dialog_panel()),
                    )
                    .push(
                        row(Vec::new())
                            .spacing(tokens::SPACE_SM)
                            .push(
                                button::custom(
                                    row(Vec::new())
                                        .spacing(tokens::SPACE_SM)
                                        .push(widget::text("\u{1f4c2}"))
                                        .push(widget::text("Ouvrir un dossier\u{2026}")),
                                )
                                .on_press(AppEvent::WorkspaceOpened(None))
                                .class(cosmic::theme::Button::Suggested),
                            )
                            .push(
                                button::custom(
                                    row(Vec::new())
                                        .spacing(tokens::SPACE_SM)
                                        .push(widget::text("\u{274c}"))
                                        .push(widget::text("Fermer le workspace")),
                                )
                                .on_press(AppEvent::WorkspaceOpened(None))
                                .class(cosmic::theme::Button::Destructive),
                            ),
                    ),
            )
            .width(Length::Fill)
            .padding(tokens::SPACE_LG),
        )
        .into()
}
