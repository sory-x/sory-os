// SPDX-License-Identifier: GPL-3.0-only

//! Sélecteur de modèle IA avec retour vers l'accueil.

use cosmic::{
    Element,
    iced::Length,
    widget::{self, button, column, container, row, scrollable},
};

use crate::{
    components::page_header,
    events::AppEvent,
    models::{catalog, known_providers},
    state::AppState,
    theme::tokens,
};

pub fn view(state: &AppState) -> Element<AppEvent> {
    let provider_id = &state.settings.settings.provider_id;
    let provider_name = known_providers()
        .into_iter()
        .find(|p| p.id == *provider_id)
        .map(|p| p.name)
        .unwrap_or_else(|| provider_id.clone());

    let active_model = state
        .settings
        .settings
        .provider_configs
        .get(provider_id)
        .map(|c| c.model.as_str())
        .filter(|m| !m.is_empty())
        .unwrap_or("auto");

    let models = catalog::resolved_models_for(provider_id, active_model);
    let model_count = models.len();
    let subtitle = format!(
        "{provider_name} \u{2022} {model_count} mod\u{00e8}le(s) disponible(s)"
    );

    let mut list = column(Vec::new()).spacing(tokens::SPACE_XS);

    if models.is_empty() {
        list = list.push(
            widget::text::colored(
                "Aucun modèle disponible pour ce provider.",
                cosmic::palette::SORY.TEXT_MUTED,
            )
            .size(tokens::FONT_SM),
        );
    } else {
        for model in models {
            let is_active = model == active_model;
            let pid = provider_id.clone();
            let model_clone = model.clone();

            let item = container(
                row(Vec::new())
                    .spacing(tokens::SPACE_SM)
                    .align_y(cosmic::iced::Alignment::Center)
                    .push(
                        widget::text::colored(
                            if is_active { "\u{25cf}" } else { "\u{25cb}" },
                            if is_active {
                                cosmic::palette::SORY.ACCENT
                            } else {
                                cosmic::palette::SORY.TEXT_MUTED
                            },
                        )
                        .size(tokens::FONT_SM),
                    )
                    .push(widget::text(model).size(tokens::FONT_MD)),
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
                    .on_press(AppEvent::SelectModelAndReturn(pid, model_clone)),
            );
        }
    }

    column(Vec::new())
        .spacing(tokens::SPACE_MD)
        .push(page_header::view(
            "Mod\u{00e8}le",
            Some(&subtitle),
        ))
        .push(widget::divider::horizontal::default())
        .push(
            scrollable(list)
                .height(Length::Fixed(480.0))
                .width(Length::Fill),
        )
        .push(
            button::text("Retour au chat")
                .on_press(AppEvent::OpenChat)
                .class(cosmic::theme::Button::Suggested),
        )
        .into()
}
