// SPDX-License-Identifier: GPL-3.0-only

//! Notification banner — style "Deep Navy Glass".

use cosmic::{
    Element,
    iced::{Alignment, Length},
    widget::{self, container, row},
};

use crate::{events::AppEvent, theme::tokens};

pub fn view(message: &str) -> Element<AppEvent> {
    container(
        row(Vec::new())
            .spacing(tokens::SPACE_SM)
            .align_y(Alignment::Center)
            .push(
                widget::text::colored("\u{2139}", cosmic::palette::SORY.ACCENT)
                    .size(f32::from(tokens::FONT_MD)),
            )
            .push(
                widget::text(message)
                    .size(f32::from(tokens::FONT_SM))
                    .font(cosmic::font::default()),
            ),
    )
    .width(Length::Fill)
    .padding([tokens::SPACE_SM, tokens::SPACE_MD])
    .class(cosmic::theme::sory::notification())
    .into()
}
