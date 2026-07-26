// SPDX-License-Identifier: GPL-3.0-only

//! Bulles de message — fidèle à la maquette officielle.

use cosmic::{
    Element,
    iced::{Alignment, Length},
    widget::{self, Space, column, container, row},
};

use crate::{
    components::message_part,
    events::AppEvent,
    models::{Message, MessageRole, MessageStatus},
    state::AppState,
    theme::tokens,
};

fn role_icon(role: MessageRole) -> &'static str {
    match role {
        MessageRole::User => "\u{25cf}",
        MessageRole::Assistant => "\u{2726}",
        MessageRole::System => "\u{25c6}",
        MessageRole::Tool => "\u{25a3}",
    }
}

fn role_name(role: MessageRole) -> &'static str {
    match role {
        MessageRole::User => tokens::USER_NAME,
        MessageRole::Assistant => tokens::ASSISTANT_NAME,
        MessageRole::System => "Syst\u{00e8}me",
        MessageRole::Tool => "Outil",
    }
}

fn role_container_style(role: MessageRole) -> cosmic::theme::Container<'static> {
    match role {
        MessageRole::User => cosmic::theme::sory::tile_icon_green(),
        MessageRole::Assistant => cosmic::theme::sory::tile_icon_blue(),
        MessageRole::System => cosmic::theme::sory::tile_icon_orange(),
        MessageRole::Tool => cosmic::theme::sory::tile_icon_purple(),
    }
}

fn avatar(role: MessageRole) -> Element<'static, AppEvent> {
    let icon = role_icon(role);

    container(
        widget::text(icon)
            .size(f32::from(tokens::NAV_ICON_SIZE))
            .font(cosmic::font::bold()),
    )
    .center_x(f32::from(tokens::AVATAR_SIZE))
    .center_y(f32::from(tokens::AVATAR_SIZE))
    .class(role_container_style(role))
    .into()
}

fn message_header(message: &Message) -> Element<'static, AppEvent> {
    let role = message.role;

    let name = widget::text(role_name(role))
        .size(f32::from(tokens::FONT_MD))
        .font(cosmic::font::semibold());

    let timestamp = widget::text::colored("10:42", cosmic::palette::SORY.TEXT_MUTED)
        .size(f32::from(tokens::FONT_XS));

    let mut header = row(Vec::new())
        .spacing(tokens::SPACE_SM)
        .align_y(Alignment::Center)
        .push(name)
        .push(timestamp)
        .push(Space::new().width(Length::Fill));

    match message.status {
        MessageStatus::Streaming => {
            header = header.push(
                widget::text::colored("En cours\u{2026}", cosmic::palette::SORY.ACCENT_BRIGHT)
                    .size(f32::from(tokens::FONT_XS)),
            );
        }
        MessageStatus::Failed => {
            header = header.push(
                widget::text::colored("Erreur", cosmic::palette::SORY.ACCENT_RED)
                    .size(f32::from(tokens::FONT_XS)),
            );
        }
        _ => {}
    }

    header.into()
}

fn action_button(icon: &str, event: AppEvent) -> Element<AppEvent> {
    cosmic::widget::button::text(icon)
        .on_press(event)
        .padding(tokens::SPACE_XS)
        .class(cosmic::theme::Button::Text)
        .into()
}

fn message_actions(message: &Message) -> Element<'static, AppEvent> {
    if message.role != MessageRole::Assistant || message.status == MessageStatus::Streaming {
        return Space::new().height(Length::Shrink).into();
    }

    row(Vec::new())
        .spacing(tokens::SPACE_XS)
        .push(action_button(
            tokens::ICON_COPY,
            AppEvent::CopyMessage(message.id),
        ))
        .push(action_button(tokens::ICON_LIKE, AppEvent::None))
        .push(action_button(tokens::ICON_DISLIKE, AppEvent::None))
        .push(action_button(
            tokens::ICON_REGENERATE,
            AppEvent::RegenerateMessage(message.id),
        ))
        .push(action_button(tokens::ICON_SHARE, AppEvent::None))
        .push(action_button(tokens::ICON_AUDIO, AppEvent::None))
        .into()
}

pub fn view<'a>(message: &'a Message, state: &'a AppState) -> Element<'a, AppEvent> {
    let header = message_header(message);
    let parts = message_part::render_parts(message, state);
    let mut body = column(Vec::new())
        .spacing(tokens::SPACE_SM)
        .push(header);

    for part in parts {
        body = body.push(part);
    }
    body = body.push(message_actions(message));

    let bubble_container: Element<AppEvent> = match message.role {
        MessageRole::User => container(body)
            .width(Length::Fill)
            .padding(tokens::BUBBLE_PADDING)
            .class(cosmic::theme::sory::dialog_panel())
            .into(),
        MessageRole::Assistant => container(body)
            .width(Length::Fill)
            .padding(tokens::BUBBLE_PADDING)
            .class(cosmic::theme::sory::dialog_panel())
            .into(),
        MessageRole::System => container(body)
            .width(Length::Fill)
            .padding(tokens::BUBBLE_PADDING)
            .class(cosmic::theme::sory::info_card())
            .into(),
        MessageRole::Tool => container(body)
            .width(Length::Fill)
            .padding(tokens::BUBBLE_PADDING)
            .class(cosmic::theme::sory::context_content())
            .into(),
    };

    container(
        row(Vec::new())
            .spacing(tokens::SPACE_SM)
            .align_y(Alignment::Start)
            .push(avatar(message.role))
            .push(bubble_container),
    )
    .width(Length::Fill)
    .padding([tokens::SPACE_SM, tokens::SPACE_MD])
    .into()
}

pub fn empty_state() -> Element<'static, AppEvent> {
    container(
        column(Vec::new())
            .spacing(tokens::SPACE_XL)
            .align_x(Alignment::Center)
            .push(
                container(widget::text(tokens::ICON_APP).size(48.0))
                    .center_x(Length::Fill)
                    .padding(tokens::SPACE_LG),
            )
            .push(
                widget::text::colored(tokens::APP_TITLE, cosmic::palette::SORY.ACCENT)
                    .size(f32::from(tokens::FONT_XXXL))
                    .font(cosmic::font::bold()),
            )
            .push(
                widget::text::colored(
                    "Session de travail pr\u{00e9}e. Les messages, outils, \n\
                     permissions et t\u{00e2}ches seront organis\u{00e9}s dans ce fil.",
                    cosmic::palette::SORY.TEXT_SECONDARY,
                )
                .size(f32::from(tokens::FONT_MD))
                .align_x(Alignment::Center),
            )
            .push(
                row(Vec::new())
                    .spacing(tokens::SPACE_MD)
                    .align_y(Alignment::Center)
                    .push(status_badge("Messages", "\u{2709}"))
                    .push(status_badge("Outils", "\u{2699}"))
                    .push(status_badge("Permissions", "\u{1f512}")),
            ),
    )
    .width(Length::Fill)
    .center_y(Length::Fill)
    .padding(f32::from(tokens::SPACE_XXL))
    .class(cosmic::theme::sory::empty_state())
    .into()
}

fn status_badge(label: &'static str, icon: &'static str) -> Element<'static, AppEvent> {
    container(
        row(Vec::new())
            .spacing(tokens::SPACE_XS)
            .align_y(Alignment::Center)
            .push(widget::text(icon).size(f32::from(tokens::FONT_MD)))
            .push(widget::text(label).size(f32::from(tokens::FONT_SM))),
    )
    .padding([tokens::SPACE_XS, tokens::SPACE_SM])
    .class(cosmic::theme::sory::chip())
    .into()
}
