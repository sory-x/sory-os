// SPDX-License-Identifier: GPL-3.0-only

use crate::{components::page_header, events::AppEvent, theme::tokens};
use cosmic::{
    Element,
    iced::Length,
    widget::{self, column, container},
};

pub fn view() -> Element<'static, AppEvent> {
    column(Vec::new())
        .spacing(tokens::SPACE_MD)
        .width(Length::Fill)
        .push(page_header::view(
            "Historique",
            Some("Vos conversations pass\u{00e9}es"),
        ))
        .push(widget::divider::horizontal::default())
        .push(
            container(
                column(Vec::new())
                    .spacing(tokens::SPACE_XL)
                    .push(
                        container(
                            widget::text::colored(
                                "\u{1f4ad}",
                                cosmic::palette::SORY.TEXT_MUTED,
                            )
                            .size(48.0),
                        )
                        .center_x(Length::Fill)
                        .padding(tokens::SPACE_LG),
                    )
                    .push(
                        widget::text::colored(
                            "Aucune conversation",
                            cosmic::palette::SORY.TEXT_SECONDARY,
                        )
                        .size(tokens::FONT_LG),
                    )
                    .push(
                        widget::text::colored(
                            "Vos conversations appara\u{00ee}tront ici une fois que\n\
                             vous aurez commenc\u{00e9} \u{00e0} discuter avec Sory IA.",
                            cosmic::palette::SORY.TEXT_MUTED,
                        )
                        .size(tokens::FONT_SM),
                    ),
            )
            .width(Length::Fill)
            .padding(tokens::SPACE_XXL),
        )
        .into()
}
