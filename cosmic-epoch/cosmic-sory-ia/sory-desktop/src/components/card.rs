use cosmic::{
    Element,
    widget::{column, container},
};

use crate::{events::AppEvent, theme::tokens};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    Normal,
    Error,
    Warning,
    Success,
    Info,
}

pub fn view<'a>(
    variant: Variant,
    body: impl Into<Element<'a, AppEvent>>,
) -> Element<'a, AppEvent> {
    let class = match variant {
        Variant::Normal => cosmic::theme::sory::dialog_panel(),
        Variant::Error => cosmic::theme::sory::info_card(),
        Variant::Warning => cosmic::theme::sory::info_card(),
        Variant::Success => cosmic::theme::sory::info_card(),
        Variant::Info => cosmic::theme::sory::info_card(),
    };

    let content = column(Vec::new()).spacing(tokens::SPACE_SM).push(body);

    container(content)
        .width(cosmic::iced::Length::Fill)
        .padding(tokens::CARD_PADDING)
        .class(class)
        .into()
}

pub fn view_with_header<'a>(
    variant: Variant,
    header: Element<'a, AppEvent>,
    body: impl Into<Element<'a, AppEvent>>,
) -> Element<'a, AppEvent> {
    let class = match variant {
        Variant::Normal => cosmic::theme::sory::dialog_panel(),
        Variant::Error => cosmic::theme::sory::info_card(),
        Variant::Warning => cosmic::theme::sory::info_card(),
        Variant::Success => cosmic::theme::sory::info_card(),
        Variant::Info => cosmic::theme::sory::info_card(),
    };

    let content = column(Vec::new())
        .spacing(tokens::SPACE_SM)
        .push(header)
        .push(body);

    container(content)
        .width(cosmic::iced::Length::Fill)
        .padding(tokens::CARD_PADDING)
        .class(class)
        .into()
}
