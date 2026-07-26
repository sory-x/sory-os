// SPDX-License-Identifier: GPL-3.0-only

//! Layout principal 3 colonnes partagé par toutes les pages.

use cosmic::{
    Element,
    iced::Length,
    widget::{self, column, container, row},
};

use crate::{
    components::{right_sidebar, sidebar},
    events::AppEvent,
    state::AppState,
    theme::tokens,
};

pub fn view<'a>(
    state: &'a AppState,
    center: Element<'a, AppEvent>,
    show_workspace: bool,
) -> Element<'a, AppEvent> {
    let center_panel = container(center)
        .width(Length::Fill)
        .height(Length::Fill)
        .class(cosmic::theme::Container::default());

    let mut layout = row(Vec::new())
        .push(sidebar::view(state))
        .push(center_panel);

    if show_workspace {
        layout = layout.push(right_sidebar::view(state));
    }

    layout
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

pub fn scrollable_page<'a>(content: Element<'a, AppEvent>) -> Element<'a, AppEvent> {
    widget::scrollable(
        container(content)
            .width(Length::Fill)
            .max_width(tokens::CHAT_MAX_WIDTH),
    )
    .height(Length::Fill)
    .into()
}

pub fn centered_page<'a>(content: Element<'a, AppEvent>) -> Element<'a, AppEvent> {
    container(
        row(Vec::new())
            .push(widget::Space::new().width(Length::Fill))
            .push(
                column(Vec::new())
                    .push(content)
                    .width(Length::Fixed(tokens::CHAT_MAX_WIDTH.into())),
            )
            .push(widget::Space::new().width(Length::Fill)),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
