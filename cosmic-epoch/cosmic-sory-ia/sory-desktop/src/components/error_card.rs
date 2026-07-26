use cosmic::{
    Element,
    iced::{Alignment, Length},
    widget::{self, column, container, row},
};

use crate::{events::AppEvent, theme::tokens};

pub fn view<'a>(
    title: &'a str,
    details: &'a str,
) -> Element<'a, AppEvent> {
    container(
        column(Vec::new())
            .spacing(tokens::SPACE_SM)
            .push(
                row(Vec::new())
                    .spacing(tokens::SPACE_SM)
                    .align_y(Alignment::Center)
                    .push(
                        container(
                            widget::text::colored(
                                "\u{26a0}",
                                cosmic::palette::SORY.ACCENT_RED,
                            )
                            .size(f32::from(tokens::FONT_MD)),
                        )
                        .padding(tokens::SPACE_XXS)
                        .class(cosmic::theme::sory::tile_icon_orange()),
                    )
                    .push(
                        widget::text::colored(title, cosmic::palette::SORY.ACCENT_RED)
                            .size(f32::from(tokens::FONT_MD))
                            .font(cosmic::font::semibold()),
                    )
                    .push(cosmic::widget::Space::new().width(Length::Fill)),
            )
            .push(
                widget::text::colored(details, cosmic::palette::SORY.TEXT_SECONDARY)
                    .size(f32::from(tokens::FONT_SM)),
            ),
    )
    .width(Length::Fill)
    .padding(tokens::CARD_PADDING)
    .class(cosmic::theme::sory::info_card())
    .into()
}
