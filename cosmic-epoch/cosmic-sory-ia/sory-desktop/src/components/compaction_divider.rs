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
            .push(cosmic::widget::Space::new().width(Length::Fill))
            .push(
                container(
                    widget::text::colored(
                        "Messages pr\u{00e9}c\u{00e9}dents masqu\u{00e9}s",
                        cosmic::palette::SORY.TEXT_MUTED,
                    )
                    .size(f32::from(tokens::FONT_XS)),
                )
                .padding([tokens::SPACE_XS, tokens::SPACE_SM])
                .class(cosmic::theme::sory::chip()),
            )
            .push(cosmic::widget::Space::new().width(Length::Fill)),
    )
    .width(Length::Fill)
    .padding([tokens::SPACE_SM, 0])
    .into()
}
