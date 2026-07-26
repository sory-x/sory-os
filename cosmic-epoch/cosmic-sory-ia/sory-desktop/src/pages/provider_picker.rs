// SPDX-License-Identifier: GPL-3.0-only

//! Sélecteur de provider IA avec retour vers l'accueil.

use cosmic::{
    Element,
    iced::Length,
    widget::{self, button, column, container, row, scrollable},
};

use crate::{
    components::page_header,
    events::AppEvent,
    state::AppState,
    theme::tokens,
};

pub fn view(state: &AppState) -> Element<AppEvent> {
    let active_id = &state.settings.settings.provider_id;

    let mut list = column(Vec::new()).spacing(tokens::SPACE_XS);
    for provider in &state.providers.definitions {
        if !provider.enabled {
            continue;
        }
        let is_active = provider.id == *active_id;
        let pid = provider.id.clone();

        let item = container(
            row(Vec::new())
                .spacing(tokens::SPACE_SM)
                .push(widget::text(if is_active { "\u{25cf}" } else { "\u{25cb}" }))
                .push(
                    column(Vec::new())
                        .spacing(tokens::SPACE_XXS)
                        .push(widget::text(provider.name.clone()).size(tokens::FONT_MD))
                        .push(
                            widget::text::colored(
                                &provider.endpoint,
                                cosmic::palette::SORY.TEXT_MUTED,
                            )
                            .size(tokens::FONT_XS),
                        ),
                ),
        )
        .width(Length::Fill)
        .padding([tokens::SPACE_SM, tokens::SPACE_MD])
        .class(if is_active {
            cosmic::theme::sory::sidebar_item_active()
        } else {
            cosmic::theme::sory::sidebar_item()
        });

        list = list.push(
            button::custom(item)
                .width(Length::Fill)
                .on_press(AppEvent::SelectProviderAndReturn(pid)),
        );
    }

    column(Vec::new())
        .spacing(tokens::SPACE_MD)
        .push(page_header::view(
            "Provider",
            Some("S\u{00e9}lectionnez le fournisseur IA"),
        ))
        .push(widget::divider::horizontal::default())
        .push(
            scrollable(list)
                .height(Length::Fixed(520.0))
                .width(Length::Fill),
        )
        .push(
            button::text("Configurer les param\u{00e8}tres avanc\u{00e9}s")
                .on_press(AppEvent::OpenSettings)
                .class(cosmic::theme::Button::Text),
        )
        .push(
            button::text("Retour au chat")
                .on_press(AppEvent::OpenChat)
                .class(cosmic::theme::Button::Suggested),
        )
        .into()
}
