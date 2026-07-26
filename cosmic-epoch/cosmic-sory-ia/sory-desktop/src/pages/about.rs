// SPDX-License-Identifier: GPL-3.0-only

use crate::{components::page_header, events::AppEvent, theme::tokens};
use cosmic::{
    Element,
    iced::{Alignment, Length},
    widget::{self, column, container},
};

pub fn view() -> Element<'static, AppEvent> {
    column(Vec::new())
        .spacing(tokens::SPACE_MD)
        .width(Length::Fill)
        .push(page_header::view(
            "\u{00c0} propos",
            Some("Sory IA pour SoryOS"),
        ))
        .push(widget::divider::horizontal::default())
        .push(
            container(
                column(Vec::new())
                    .spacing(tokens::SPACE_XL)
                    .align_x(Alignment::Center)
                    .push(
                        container(
                            widget::text::colored(
                                tokens::ICON_APP,
                                cosmic::palette::SORY.ACCENT_BRIGHT,
                            )
                            .size(64.0),
                        )
                        .center_x(Length::Fill)
                        .padding(tokens::SPACE_LG),
                    )
                    .push(
                        widget::text::colored("Sory IA", cosmic::palette::SORY.ACCENT)
                            .size(tokens::FONT_XXXL)
                            .font(cosmic::font::bold()),
                    )
                    .push(
                        widget::text::colored(
                            format!("Version {}", env!("CARGO_PKG_VERSION")),
                            cosmic::palette::SORY.TEXT_SECONDARY,
                        )
                        .size(tokens::FONT_MD),
                    )
                    .push(
                        widget::text::colored(
                            "Assistant IA natif int\u{00e9}gr\u{00e9} \u{00e0} SoryOS.\n\
                             Propuls\u{00e9} par le runtime Sory IA et les meilleurs mod\u{00e8}les\n\
                             de fournisseurs comme OpenAI, Anthropic, Google, et plus.",
                            cosmic::palette::SORY.TEXT_SECONDARY,
                        )
                        .size(tokens::FONT_MD)
                        .font(cosmic::font::default())
                        .align_x(Alignment::Center),
                    ),
            )
            .width(Length::Fill)
            .padding(tokens::SPACE_XXL),
        )
        .into()
}
