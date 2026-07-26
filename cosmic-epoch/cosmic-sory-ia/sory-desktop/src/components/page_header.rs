// SPDX-License-Identifier: GPL-3.0-only

//! En-tête de page secondaire avec bouton retour vers l'accueil.

use cosmic::{
    Element,
    iced::{Alignment, Length},
    widget::{self, Space, button, container, row},
};

use crate::{events::AppEvent, theme::tokens};

pub fn view(title: &str, subtitle: Option<&str>) -> Element<'static, AppEvent> {
    let title_owned = title.to_owned();
    container(column_header(title_owned, subtitle.map(str::to_owned)))
        .width(Length::Fill)
        .padding([tokens::SPACE_MD, 0, tokens::SPACE_LG, 0])
        .into()
}

fn column_header(title: String, subtitle: Option<String>) -> Element<'static, AppEvent> {
    let back_btn = button::text(format!("\u{2190} Retour"))
        .on_press(AppEvent::OpenChat)
        .padding([tokens::SPACE_XS, tokens::SPACE_SM])
        .class(cosmic::theme::Button::Text);

    let mut title_col = cosmic::widget::column(Vec::new())
        .spacing(tokens::SPACE_XXS)
        .push(
            widget::text(title)
                .size(tokens::FONT_XXL)
                .font(cosmic::font::bold()),
        );

    if let Some(sub) = subtitle {
        title_col = title_col.push(
            widget::text::colored(sub, cosmic::palette::SORY.TEXT_MUTED).size(tokens::FONT_SM),
        );
    }

    row(Vec::new())
        .spacing(tokens::SPACE_MD)
        .align_y(Alignment::Center)
        .push(back_btn)
        .push(title_col)
        .push(Space::new().width(Length::Fill))
        .into()
}
