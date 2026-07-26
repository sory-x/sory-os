use cosmic::{
    Element,
    iced::{Alignment, Length},
    widget::{self, container, row},
};

use crate::{events::AppEvent, theme::tokens};

pub fn view<'a>() -> Element<'a, AppEvent> {
    container(
        row(Vec::new())
            .spacing(tokens::SPACE_SM)
            .align_y(Alignment::Center)
            .push(
                widget::text::colored(
                    "\u{2728}",
                    cosmic::palette::SORY.ACCENT_BRIGHT,
                )
                .size(f32::from(tokens::FONT_LG)),
            )
            .push(
                widget::text::colored(
                    "Sory IA r\u{00e9}fl\u{00e9}chit\u{2026}",
                    cosmic::palette::SORY.ACCENT,
                )
                .size(f32::from(tokens::FONT_MD))
                .font(cosmic::font::semibold()),
            )
            .push(
                widget::text::colored(
                    "\u{2026}",
                    cosmic::palette::SORY.TEXT_MUTED,
                )
                .size(f32::from(tokens::FONT_LG)),
            ),
    )
    .width(Length::Fill)
    .padding(tokens::SPACE_SM)
    .class(cosmic::theme::sory::context_content())
    .into()
}
