use cosmic::{
    Element,
    iced::{Alignment, Length},
    widget::{self, button, column, container, row},
};

use crate::{events::AppEvent, theme::tokens};

pub fn view<'a>(
    header: impl Into<Element<'a, AppEvent>>,
    is_expanded: bool,
    on_toggle: AppEvent,
    content: impl Into<Element<'a, AppEvent>>,
) -> Element<'a, AppEvent> {
    let chevron = if is_expanded { "\u{25bc}" } else { "\u{25b6}" };

    let header_row: Element<'a, AppEvent> = row(Vec::new())
        .spacing(tokens::SPACE_XS)
        .align_y(Alignment::Center)
        .push(
            widget::text::colored(chevron, cosmic::palette::SORY.TEXT_SECONDARY)
                .size(f32::from(tokens::FONT_SM)),
        )
        .push(header.into())
        .push(cosmic::widget::Space::new().width(Length::Fill))
        .into();

    let trigger = button::custom(header_row)
        .on_press(on_toggle)
        .padding(tokens::SPACE_XS);

    let mut col = column(Vec::new()).spacing(tokens::SPACE_XS);

    if is_expanded {
        col = col.push(
            container(trigger)
                .width(Length::Fill)
                .class(cosmic::theme::sory::context_content()),
        );
        col = col.push(content.into());
    } else {
        col = col.push(
            container(trigger)
                .width(Length::Fill)
                .class(cosmic::theme::sory::context_content()),
        );
    }

    col.into()
}
