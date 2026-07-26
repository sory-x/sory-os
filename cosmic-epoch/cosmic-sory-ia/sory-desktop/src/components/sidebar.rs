// SPDX-License-Identifier: GPL-3.0-only

//! Sidebar gauche de Sory IA — fidèle à la maquette officielle.
//!
//! Layout vertical :
//! 1. Header (logo + bouton toggle ☰/✕)
//! 2. Bouton « Nouvelle conversation » (+ Ctrl K)
//! 3. Navigation principale
//! 4. Conversations récentes
//! 5. Profil utilisateur (bas)

use cosmic::{
    Element,
    iced::{Alignment, Length},
    widget::{self, Space, button, column, container, row, scrollable},
};

use crate::{
    events::AppEvent,
    state::{ActivePage, AppState, RuntimeStatus},
    theme::tokens,
};

fn sidebar_expanded(state: &AppState) -> bool {
    state.sidebar.expand_progress > 0.35
}

fn sidebar_width(state: &AppState) -> Length {
    Length::Fixed(state.sidebar.effective_width())
}

// ─── Header ─────────────────────────────────────────────────────────────────

fn sidebar_header(state: &AppState) -> Element<AppEvent> {
    let expanded = sidebar_expanded(state);
    let toggle_icon = if expanded {
        tokens::ICON_CLOSE
    } else {
        tokens::ICON_MENU_TOGGLE
    };

    let toggle_btn = button::text(toggle_icon)
        .on_press(AppEvent::ToggleSidebar)
        .padding(tokens::SPACE_XS)
        .class(cosmic::theme::Button::Text);

    if expanded {
        container(
            row(Vec::new())
                .spacing(tokens::SPACE_SM)
                .align_y(Alignment::Center)
                .push(
                    widget::text::colored(tokens::ICON_APP, cosmic::palette::SORY.ACCENT_BRIGHT)
                        .size(tokens::FONT_XL)
                        .font(cosmic::font::bold()),
                )
                .push(
                    widget::text(tokens::APP_TITLE)
                        .size(tokens::FONT_XL)
                        .font(cosmic::font::bold()),
                )
                .push(Space::new().width(Length::Fill))
                .push(toggle_btn),
        )
        .width(Length::Fill)
        .padding([tokens::SPACE_SM, tokens::SPACE_MD])
        .into()
    } else {
        container(
            column(Vec::new())
                .spacing(tokens::SPACE_SM)
                .align_x(Alignment::Center)
                .push(toggle_btn)
                .push(
                    widget::text::colored(tokens::ICON_APP, cosmic::palette::SORY.ACCENT_BRIGHT)
                        .size(tokens::FONT_LG),
                ),
        )
        .width(Length::Fill)
        .padding(tokens::SPACE_SM)
        .into()
    }
}

// ─── Nouvelle conversation ──────────────────────────────────────────────────

fn new_conversation_button(state: &AppState) -> Element<AppEvent> {
    let expanded = sidebar_expanded(state);

    if expanded {
        let label = row(Vec::new())
            .spacing(tokens::SPACE_SM)
            .align_y(Alignment::Center)
            .push(
                widget::text::colored(tokens::ICON_NEW, cosmic::palette::SORY.TEXT_ON_ACCENT)
                    .size(tokens::FONT_LG)
                    .font(cosmic::font::bold()),
            )
            .push(
                widget::text::colored(
                    tokens::NEW_CONVERSATION_LABEL,
                    cosmic::palette::SORY.TEXT_ON_ACCENT,
                )
                .size(tokens::FONT_MD)
                .font(cosmic::font::semibold()),
            )
            .push(Space::new().width(Length::Fill))
            .push(
                widget::text::colored(
                    tokens::NEW_CONVERSATION_SHORTCUT,
                    cosmic::palette::SORY.TEXT_ON_ACCENT,
                )
                .size(tokens::FONT_XS)
                .font(cosmic::font::default()),
            );

        button::custom(label)
            .on_press(AppEvent::NewConversation)
            .width(Length::Fill)
            .padding([tokens::SPACE_SM, tokens::SPACE_MD])
            .class(cosmic::theme::Button::Suggested)
            .into()
    } else {
        button::text(tokens::ICON_NEW)
            .on_press(AppEvent::NewConversation)
            .padding(tokens::SPACE_SM)
            .class(cosmic::theme::Button::Suggested)
            .into()
    }
}

// ─── Navigation ─────────────────────────────────────────────────────────────

fn nav_item<'a>(
    label: &'a str,
    icon: &'a str,
    page: ActivePage,
    active_page: ActivePage,
    event: AppEvent,
    expanded: bool,
) -> Element<'a, AppEvent> {
    let is_active = page == active_page;

    let indicator = if is_active {
        container(
            Space::new()
                .width(Length::Fixed(3.0))
                .height(Length::Fixed(20.0)),
        )
        .class(cosmic::theme::Container::Primary)
    } else {
        container(
            Space::new()
                .width(Length::Fixed(3.0))
                .height(Length::Fixed(20.0)),
        )
    };

    let mut content_row = row(Vec::new())
        .spacing(tokens::SPACE_SM)
        .align_y(Alignment::Center);

    if expanded {
        content_row = content_row
            .push(indicator)
            .push(
                widget::text(icon)
                    .size(tokens::NAV_ICON_SIZE)
                    .font(cosmic::font::default()),
            )
            .push(
                widget::text(label)
                    .size(tokens::FONT_MD)
                    .font(cosmic::font::default()),
            );
    } else {
        content_row = content_row.push(
            container(
                widget::text(icon)
                    .size(tokens::NAV_ICON_SIZE)
                    .font(cosmic::font::default()),
            )
            .center_x(Length::Fill)
            .width(Length::Fill),
        );
    }

    let content: Element<AppEvent> = container(content_row)
        .width(Length::Fill)
        .padding(if expanded {
            [tokens::SPACE_SM, tokens::SPACE_SM]
        } else {
            [tokens::SPACE_SM, tokens::SPACE_XS]
        })
        .class(if is_active {
            cosmic::theme::Container::Primary
        } else {
            cosmic::theme::Container::Transparent
        })
        .into();

    let mut btn = button::custom(content).width(Length::Fill);
    if !is_active {
        btn = btn.on_press(event);
    }
    btn.into()
}

fn nav_static<'a>(label: &'a str, icon: &'a str, expanded: bool) -> Element<'a, AppEvent> {
    if expanded {
        let content: Element<AppEvent> = container(
            row(Vec::new())
                .spacing(tokens::SPACE_SM)
                .align_y(Alignment::Center)
                .push(Space::new().width(Length::Fixed(3.0)))
                .push(
                    widget::text(icon)
                        .size(tokens::NAV_ICON_SIZE)
                        .font(cosmic::font::default()),
                )
                .push(
                    widget::text(label)
                        .size(tokens::FONT_MD)
                        .font(cosmic::font::default()),
                ),
        )
        .width(Length::Fill)
        .padding([tokens::SPACE_SM, tokens::SPACE_SM])
        .class(cosmic::theme::Container::Transparent)
        .into();

        button::custom(content)
            .on_press(AppEvent::None)
            .width(Length::Fill)
            .into()
    } else {
        button::text(icon)
            .on_press(AppEvent::None)
            .padding(tokens::SPACE_SM)
            .class(cosmic::theme::Button::Text)
            .into()
    }
}

fn nav_section(state: &AppState) -> Element<AppEvent> {
    let active_page = state.sidebar.active_page;
    let expanded = sidebar_expanded(state);

    column(Vec::new())
        .spacing(tokens::SPACE_XXS)
        .push(nav_item(
            "Accueil",
            tokens::ICON_HOME,
            ActivePage::Chat,
            active_page,
            AppEvent::OpenChat,
            expanded,
        ))
        .push(nav_item(
            "Historique",
            tokens::ICON_HISTORY,
            ActivePage::History,
            active_page,
            AppEvent::OpenHistory,
            expanded,
        ))
        .push(nav_item(
            "Favoris",
            tokens::ICON_FAVORITES,
            ActivePage::Favorites,
            active_page,
            AppEvent::OpenFavorites,
            expanded,
        ))
        .push(nav_item(
            "Workspace",
            tokens::ICON_WORKSPACE,
            ActivePage::Workspace,
            active_page,
            AppEvent::OpenWorkspace(None),
            expanded,
        ))
        .push(nav_static("Agents", tokens::ICON_AGENTS, expanded))
        .push(nav_static("Outils", tokens::ICON_TOOLS, expanded))
        .push(nav_item(
            "Param\u{00e8}tres",
            tokens::ICON_SETTINGS,
            ActivePage::Settings,
            active_page,
            AppEvent::OpenSettings,
            expanded,
        ))
        .push(nav_item(
            "\u{00c0} propos",
            tokens::ICON_ABOUT,
            ActivePage::About,
            active_page,
            AppEvent::OpenAbout,
            expanded,
        ))
        .into()
}

// ─── Conversations récentes ─────────────────────────────────────────────────

fn conversation_item(
    title: &str,
    is_active: bool,
    expanded: bool,
) -> Element<AppEvent> {
    if !expanded {
        return button::text(if is_active { "\u{25cf}" } else { "\u{25cb}" })
            .on_press(AppEvent::None)
            .padding(tokens::SPACE_XS)
            .class(cosmic::theme::Button::Text)
            .into();
    }

    let mut row_content = row(Vec::new())
        .spacing(tokens::SPACE_SM)
        .align_y(Alignment::Center)
        .push(
            widget::text(title)
                .size(tokens::FONT_SM)
                .font(if is_active {
                    cosmic::font::semibold()
                } else {
                    cosmic::font::default()
                }),
        )
        .push(Space::new().width(Length::Fill));

    if is_active {
        row_content = row_content.push(
            widget::text(tokens::ICON_MENU)
                .size(tokens::FONT_SM)
                .font(cosmic::font::default()),
        );
    }

    let content: Element<AppEvent> = container(row_content)
        .width(Length::Fill)
        .padding([tokens::SPACE_SM, tokens::SPACE_MD])
        .class(if is_active {
            cosmic::theme::Container::Primary
        } else {
            cosmic::theme::Container::Transparent
        })
        .into();

    button::custom(content)
        .on_press(AppEvent::None)
        .width(Length::Fill)
        .into()
}

fn conversation_list(state: &AppState) -> Element<AppEvent> {
    let expanded = sidebar_expanded(state);
    let active_id = state.conversations.active_id;

    if !expanded {
        return Space::new().height(Length::Shrink).into();
    }

    let mut items = column(Vec::new()).spacing(tokens::SPACE_XXS);

    for conversation in state.conversations.conversations.iter().take(5) {
        let is_active = conversation.id == active_id;
        items = items.push(conversation_item(&conversation.title, is_active, expanded));
    }

    if state.conversations.conversations.is_empty() {
        items = items.push(
            container(
                widget::text::colored("Aucune conversation", cosmic::palette::SORY.TEXT_MUTED)
                    .size(tokens::FONT_SM),
            )
            .padding([tokens::SPACE_SM, tokens::SPACE_MD]),
        );
    }

    column(Vec::new())
        .spacing(tokens::SPACE_XS)
        .push(
            widget::text(tokens::RECENT_CONVERSATIONS_LABEL)
                .size(tokens::FONT_XS)
                .font(cosmic::font::semibold()),
        )
        .push(items)
        .push(
            button::text(tokens::SEE_ALL_LABEL)
                .on_press(AppEvent::OpenHistory)
                .padding([tokens::SPACE_XS, tokens::SPACE_MD])
                .class(cosmic::theme::Button::Text),
        )
        .into()
}

// ─── Profil utilisateur (bas) ───────────────────────────────────────────────

fn profile_footer(state: &AppState) -> Element<AppEvent> {
    let expanded = sidebar_expanded(state);

    let (status_dot, status_color) = match state.runtime_status {
        RuntimeStatus::Ready => ("\u{25cf}", cosmic::palette::SORY.ACCENT_GREEN),
        _ => ("\u{25cb}", cosmic::palette::SORY.TEXT_MUTED),
    };

    let avatar = container(
        widget::text::colored("S", cosmic::palette::SORY.TEXT_ON_ACCENT)
            .size(tokens::FONT_MD)
            .font(cosmic::font::semibold()),
    )
    .center_x(tokens::AVATAR_SIZE)
    .center_y(tokens::AVATAR_SIZE)
    .class(cosmic::theme::Container::default());

    if expanded {
        container(
            row(Vec::new())
                .spacing(tokens::SPACE_SM)
                .align_y(Alignment::Center)
                .push(avatar)
                .push(
                    column(Vec::new())
                        .spacing(tokens::SPACE_XXS)
                        .push(
                            widget::text("SoryOS")
                                .size(tokens::FONT_MD)
                                .font(cosmic::font::semibold()),
                        )
                        .push(
                            row(Vec::new())
                                .spacing(tokens::SPACE_XS)
                                .push(
                                    widget::text::colored(status_dot, status_color)
                                        .size(tokens::FONT_XS),
                                )
                                .push(
                                    widget::text(tokens::STATUS_ONLINE)
                                        .size(tokens::FONT_XS)
                                        .font(cosmic::font::default()),
                                ),
                        ),
                )
                .push(Space::new().width(Length::Fill))
                .push(
                    button::text(tokens::ICON_SETTINGS)
                        .on_press(AppEvent::OpenSettings)
                        .padding(tokens::SPACE_XS)
                        .class(cosmic::theme::Button::Text),
                ),
        )
        .width(Length::Fill)
        .padding([tokens::SPACE_SM, tokens::SPACE_MD])
        .class(cosmic::theme::Container::default())
        .into()
    } else {
        container(
            column(Vec::new())
                .spacing(tokens::SPACE_XS)
                .align_x(Alignment::Center)
                .push(avatar)
                .push(
                    button::text(tokens::ICON_SETTINGS)
                        .on_press(AppEvent::OpenSettings)
                        .padding(tokens::SPACE_XS)
                        .class(cosmic::theme::Button::Text),
                ),
        )
        .width(Length::Fill)
        .padding(tokens::SPACE_SM)
        .class(cosmic::theme::Container::default())
        .into()
    }
}

// ─── Vue principale ─────────────────────────────────────────────────────────

pub fn view(state: &AppState) -> Element<AppEvent> {
    let scrollable_content = column(Vec::new())
        .spacing(tokens::SPACE_SM)
        .push(sidebar_header(state))
        .push(
            container(new_conversation_button(state))
                .padding([0, tokens::SPACE_MD]),
        )
        .push(
            container(nav_section(state))
                .padding([0, tokens::SPACE_XS]),
        )
        .push(widget::divider::horizontal::default())
        .push(
            container(conversation_list(state))
                .padding([0, tokens::SPACE_MD]),
        );

    container(
        column(Vec::new())
            .push(scrollable(scrollable_content).height(Length::Fill))
            .push(profile_footer(state)),
    )
    .width(sidebar_width(state))
    .height(Length::Fill)
    .padding([tokens::SPACE_SM, 0])
    .class(cosmic::theme::Container::default())
    .into()
}
