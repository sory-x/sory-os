// SPDX-License-Identifier: GPL-3.0-only

//! Zone de saisie de messages — fidèle à la maquette officielle.

use cosmic::{
    Element,
    iced::{Alignment, Length},
    widget::{self, Space, column, container, row},
};

use crate::{events::AppEvent, theme::tokens};

fn tool_button(icon: &str) -> Element<AppEvent> {
    widget::button::text(icon)
        .on_press(AppEvent::None)
        .padding(tokens::SPACE_SM)
        .class(cosmic::theme::Button::Text)
        .into()
}

pub fn view(value: &str, is_generating: bool) -> Element<AppEvent> {
    let toolbar = row(Vec::new())
        .spacing(tokens::SPACE_XS)
        .align_y(Alignment::Center)
        .push(tool_button(tokens::ICON_NEW))
        .push(tool_button(tokens::ICON_ATTACHMENT))
        .push(tool_button(tokens::ICON_MENTION))
        .push(tool_button(tokens::ICON_WEB))
        .push(tool_button(tokens::ICON_CODE))
        .push(Space::new().width(Length::Fill))
        .push(
            container(
                row(Vec::new())
                    .spacing(tokens::SPACE_XS)
                    .align_y(Alignment::Center)
                    .push(widget::text(tokens::ICON_REASONING).size(tokens::FONT_SM))
                    .push(
                        widget::text(tokens::REASONING_LABEL)
                            .size(tokens::FONT_XS)
                            .font(cosmic::font::default()),
                    ),
            )
            .padding([tokens::SPACE_XS, tokens::SPACE_SM])
            .class(cosmic::theme::sory::chip()),
        );

    let input = widget::text_input(tokens::INPUT_PLACEHOLDER, value)
        .on_input(AppEvent::InputChanged)
        .width(Length::Fill)
        .padding([
            tokens::SPACE_MD,
            tokens::SPACE_MD,
            tokens::SPACE_MD,
            tokens::SPACE_MD,
        ])
        .on_submit(|val| AppEvent::SendMessage(val));

    let send_stop_btn: Element<AppEvent> = if is_generating {
        widget::button::text(tokens::ICON_STOP)
            .on_press(AppEvent::StopGeneration)
            .padding(tokens::SPACE_MD)
            .class(cosmic::theme::Button::Destructive)
            .into()
    } else {
        widget::button::suggested(tokens::ICON_SEND)
            .on_press(AppEvent::SendMessage(value.to_owned()))
            .padding(tokens::SPACE_MD)
            .into()
    };

    let bottom_row = row(Vec::new())
        .spacing(tokens::SPACE_SM)
        .align_y(Alignment::Center)
        .push(
            container(
                row(Vec::new())
                    .spacing(tokens::SPACE_XS)
                    .align_y(Alignment::Center)
                    .push(widget::text(tokens::ICON_WORKSPACE).size(tokens::FONT_SM))
                    .push(
                        widget::text(format!("{} \u{25be}", tokens::WORKSPACE_CURRENT))
                            .size(tokens::FONT_XS)
                            .font(cosmic::font::default()),
                    ),
            )
            .padding([tokens::SPACE_XS, tokens::SPACE_SM])
            .class(cosmic::theme::sory::chip()),
        )
        .push(Space::new().width(Length::Fill))
        .push(send_stop_btn);

    container(
        column(Vec::new())
            .spacing(tokens::SPACE_MD)
            .push(toolbar)
            .push(input)
            .push(bottom_row)
            .push(
                container(
                    widget::text::colored(tokens::DISCLAIMER, cosmic::palette::SORY.TEXT_MUTED)
                        .size(tokens::FONT_XS)
                        .font(cosmic::font::default()),
                )
                .center_x(Length::Fill)
                .width(Length::Fill),
            ),
    )
    .width(Length::Fill)
    .padding(tokens::INPUT_PADDING)
    .class(cosmic::theme::sory::dialog_panel())
    .into()
}
