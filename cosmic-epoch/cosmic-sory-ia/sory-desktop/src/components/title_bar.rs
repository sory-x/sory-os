// SPDX-License-Identifier: GPL-3.0-only

//! Barre supérieure de Sory IA — fidèle à la maquette officielle.

use cosmic::{
    Element,
    iced::{Alignment, Length},
    widget::{self, button, container, row},
};

use crate::{
    events::AppEvent,
    state::{AppState, RuntimeStatus},
    theme::tokens,
};

fn status_badge(state: &AppState) -> Element<AppEvent> {
    let (status_dot, status_label, dot_color) = match state.runtime_status {
        RuntimeStatus::Ready => (
            "\u{25cf}",
            tokens::STATUS_CONNECTED,
            cosmic::palette::SORY.ACCENT_GREEN,
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
        RuntimeStatus::Reconnecting => (
            "\u{21bb}",
            "Reconnexion\u{2026}",
            cosmic::palette::SORY.ACCENT_ORANGE,
        ),
        RuntimeStatus::Disconnected => (
            "\u{25cb}",
            "D\u{00e9}connect\u{00e9}",
            cosmic::palette::SORY.TEXT_MUTED,
        ),
        RuntimeStatus::Failed => ("\u{26a0}", "Erreur", cosmic::palette::SORY.ACCENT_RED),
    };

    container(
        row(Vec::new())
            .spacing(tokens::SPACE_XS)
            .align_y(Alignment::Center)
            .push(widget::text::colored(status_dot, dot_color).size(tokens::FONT_SM))
            .push(widget::text(status_label).size(tokens::FONT_SM)),
    )
    .padding([tokens::SPACE_XS, tokens::SPACE_SM])
    .class(cosmic::theme::Container::default())
    .into()
}

fn selector_button(label: String, event: AppEvent) -> Element<'static, AppEvent> {
    button::text(label)
        .on_press(event)
        .padding([tokens::SPACE_XS, tokens::SPACE_MD])
        .class(cosmic::theme::Button::Text)
        .into()
}

pub fn view(state: &AppState) -> Element<AppEvent> {
    let model_label: String = state
        .settings
        .settings
        .provider_configs
        .get(&state.settings.settings.provider_id)
        .map(|c| c.model.as_str())
        .filter(|m| !m.is_empty())
        .unwrap_or("Devstral Medium")
        .to_string();

    let provider_name = state
        .providers
        .definitions
        .iter()
        .find(|d| d.id == state.settings.settings.provider_id)
        .map(|d| d.name.as_str())
        .unwrap_or(&state.settings.settings.provider_id);

    let model_selector = selector_button(
        format!("\u{25a1} {model_label} \u{25be}"),
        AppEvent::OpenModelPicker,
    );
    let provider_selector = selector_button(
        format!("\u{21c4} {provider_name} \u{25be}"),
        AppEvent::OpenProviderPicker,
    );

    let btn_history = button::text(tokens::ICON_HISTORY)
        .on_press(AppEvent::OpenHistory)
        .padding(tokens::SPACE_SM)
        .class(cosmic::theme::Button::Text);

    let btn_sliders = button::text(tokens::ICON_SLIDERS)
        .on_press(AppEvent::OpenSettings)
        .padding(tokens::SPACE_SM)
        .class(cosmic::theme::Button::Text);

    let btn_expand = button::text(tokens::ICON_EXPAND)
        .on_press(AppEvent::ToggleWorkspaceSidebar)
        .padding(tokens::SPACE_SM)
        .class(cosmic::theme::Button::Text);

    container(
        row(Vec::new())
            .spacing(tokens::SPACE_MD)
            .align_y(Alignment::Center)
            .push(
                row(Vec::new())
                    .spacing(tokens::SPACE_SM)
                    .align_y(Alignment::Center)
                    .push(
                        widget::text(tokens::APP_TITLE)
                            .size(tokens::FONT_XL)
                            .font(cosmic::font::bold()),
                    )
                    .push(status_badge(state)),
            )
            .push(widget::Space::new().width(Length::Fill))
            .push(model_selector)
            .push(provider_selector)
            .push(btn_history)
            .push(btn_sliders)
            .push(btn_expand),
    )
    .width(Length::Fill)
    .height(tokens::HEADER_HEIGHT)
    .padding([0, tokens::PANEL_PADDING])
    .class(cosmic::theme::Container::default())
    .into()
}
