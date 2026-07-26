// SPDX-License-Identifier: GPL-3.0-only

//! Panneau droit (workspace) — fidèle à la maquette officielle.

use cosmic::{
    Element,
    iced::{Alignment, Length},
    widget::{self, Space, button, column, container, row, scrollable},
};

use crate::{events::AppEvent, state::AppState, theme::tokens};

fn workspace_expanded(state: &AppState) -> bool {
    state.workspace_sidebar.expand_progress > 0.35
}

fn workspace_width(state: &AppState) -> Length {
    Length::Fixed(state.workspace_sidebar.effective_width())
}

fn panel_header(state: &AppState) -> Element<AppEvent> {
    let expanded = workspace_expanded(state);
    let toggle_icon = if expanded {
        tokens::ICON_CLOSE
    } else {
        tokens::ICON_MENU_TOGGLE
    };

    row(Vec::new())
        .spacing(tokens::SPACE_SM)
        .align_y(Alignment::Center)
        .push(
            button::text(toggle_icon)
                .on_press(AppEvent::ToggleWorkspaceSidebar)
                .padding(tokens::SPACE_XS)
                .class(cosmic::theme::Button::Text),
        )
        .push(
            widget::text::colored("WORKSPACE", cosmic::palette::SORY.TEXT_MUTED)
                .size(tokens::FONT_XS)
                .font(cosmic::font::semibold()),
        )
        .push(Space::new().width(Length::Fill))
        .into()
}

fn workspace_info(state: &AppState) -> Element<AppEvent> {
    let workspace_name = state
        .workspace
        .active
        .as_ref()
        .map(|w| w.name.as_str())
        .unwrap_or("SoryOS");

    let workspace_path = state
        .workspace
        .active
        .as_ref()
        .and_then(|w| w.path.as_deref())
        .unwrap_or("/home/soryos/SoryOS");

    container(
        column(Vec::new())
            .spacing(tokens::SPACE_XS)
            .push(
                row(Vec::new())
                    .spacing(tokens::SPACE_SM)
                    .align_y(Alignment::Center)
                    .push(
                        widget::text::colored(tokens::ICON_FOLDER, cosmic::palette::SORY.ACCENT)
                            .size(tokens::FONT_LG),
                    )
                    .push(
                        widget::text(workspace_name)
                            .size(tokens::FONT_MD)
                            .font(cosmic::font::semibold()),
                    ),
            )
            .push(
                widget::text::colored(workspace_path, cosmic::palette::SORY.TEXT_MUTED)
                    .size(tokens::FONT_XS),
            ),
    )
    .width(Length::Fill)
    .padding(tokens::CARD_PADDING)
    .class(cosmic::theme::sory::card_selectable())
    .into()
}

fn compact_metric(label: &str, value: &str) -> Element<'static, AppEvent> {
    let label = label.to_owned();
    let value = value.to_owned();
    container(
        row(Vec::new())
            .spacing(tokens::SPACE_SM)
            .align_y(Alignment::Center)
            .push(
                widget::text(label)
                    .size(tokens::FONT_SM)
                    .font(cosmic::font::default()),
            )
            .push(Space::new().width(Length::Fill))
            .push(
                container(widget::text(value).size(tokens::FONT_XS))
                    .padding([tokens::SPACE_XXS, tokens::SPACE_SM])
                    .class(cosmic::theme::sory::chip()),
            ),
    )
    .width(Length::Fill)
    .padding([tokens::SPACE_SM, tokens::SPACE_MD])
    .class(cosmic::theme::sory::card_selectable())
    .into()
}

fn search_bar() -> Element<'static, AppEvent> {
    container(
        row(Vec::new())
            .spacing(tokens::SPACE_SM)
            .align_y(Alignment::Center)
            .push(widget::text(tokens::ICON_SEARCH).size(tokens::FONT_SM))
            .push(
                widget::text::colored("Recherche", cosmic::palette::SORY.TEXT_MUTED)
                    .size(tokens::FONT_SM),
            ),
    )
    .width(Length::Fill)
    .padding([tokens::SPACE_SM, tokens::SPACE_MD])
    .class(cosmic::theme::sory::card_selectable())
    .into()
}

fn context_card() -> Element<'static, AppEvent> {
    container(
        column(Vec::new())
            .spacing(tokens::SPACE_SM)
            .push(
                widget::text::colored("CONTEXTE ACTIF", cosmic::palette::SORY.TEXT_MUTED)
                    .size(tokens::FONT_XS)
                    .font(cosmic::font::semibold()),
            )
            .push(compact_metric("Fichiers inclus", "12"))
            .push(compact_metric("Symboles", "24"))
            .push(compact_metric("D\u{00e9}pendances", "7")),
    )
    .width(Length::Fill)
    .padding(tokens::CARD_PADDING)
    .class(cosmic::theme::sory::context_content())
    .into()
}

fn progress_bar(progress: f32) -> Element<'static, AppEvent> {
    let filled = (progress * 100.0).clamp(0.0, 100.0) as u16;
    container(
        row(Vec::new())
            .push(
                container(Space::new().width(Length::Fill).height(Length::Fixed(4.0)))
                    .width(Length::Fixed(filled as f32))
                    .class(cosmic::theme::sory::sidebar_accent_bar()),
            )
            .push(
                container(Space::new().width(Length::Fill).height(Length::Fixed(4.0)))
                    .width(Length::Fill)
                    .class(cosmic::theme::sory::chip()),
            ),
    )
    .width(Length::Fill)
    .into()
}

fn tasks_card(state: &AppState) -> Element<'static, AppEvent> {
    let is_generating = state.conversations.is_generating;
    let progress = if is_generating { 0.45 } else { 1.0 };
    let task_state = if is_generating { "En cours" } else { "Pr\u{00ea}t" };

    let step_done = |label: String| -> Element<'static, AppEvent> {
        row(Vec::new())
            .spacing(tokens::SPACE_XS)
            .push(
                widget::text::colored(tokens::ICON_CHECK, cosmic::palette::SORY.ACCENT_GREEN)
                    .size(tokens::FONT_XS),
            )
            .push(widget::text(label).size(tokens::FONT_XS))
            .into()
    };

    let step_pending = |label: String| -> Element<'static, AppEvent> {
        row(Vec::new())
            .spacing(tokens::SPACE_XS)
            .push(
                widget::text::colored(tokens::ICON_SPINNER, cosmic::palette::SORY.ACCENT)
                    .size(tokens::FONT_XS),
            )
            .push(widget::text(label).size(tokens::FONT_XS))
            .into()
    };

    container(
        column(Vec::new())
            .spacing(tokens::SPACE_SM)
            .push(
                widget::text::colored("T\u{00c2}CHES EN COURS", cosmic::palette::SORY.TEXT_MUTED)
                    .size(tokens::FONT_XS)
                    .font(cosmic::font::semibold()),
            )
            .push(
                row(Vec::new())
                    .spacing(tokens::SPACE_SM)
                    .align_y(Alignment::Center)
                    .push(
                        widget::text::colored("+", cosmic::palette::SORY.ACCENT_GREEN)
                            .size(tokens::FONT_MD),
                    )
                    .push(widget::text("Analyse du code").size(tokens::FONT_SM))
                    .push(Space::new().width(Length::Fill))
                    .push(
                        widget::text::colored(task_state, cosmic::palette::SORY.ACCENT_GREEN)
                            .size(tokens::FONT_XS),
                    ),
            )
            .push(progress_bar(progress))
            .push(widget::text(format!("{:.0}%", progress * 100.0)).size(tokens::FONT_XS))
            .push(step_done("Lecture des fichiers".into()))
            .push(step_done("Analyse des modules".into()))
            .push(if is_generating {
                step_pending("G\u{00e9}n\u{00e9}ration de r\u{00e9}ponse".into())
            } else {
                step_done("G\u{00e9}n\u{00e9}ration de r\u{00e9}ponse".into())
            }),
    )
    .width(Length::Fill)
    .padding(tokens::CARD_PADDING)
    .class(cosmic::theme::sory::context_content())
    .into()
}

fn tools_card() -> Element<'static, AppEvent> {
    container(
        column(Vec::new())
            .spacing(tokens::SPACE_SM)
            .push(
                widget::text::colored("OUTILS UTILIS\u{00c9}S", cosmic::palette::SORY.TEXT_MUTED)
                    .size(tokens::FONT_XS)
                    .font(cosmic::font::semibold()),
            )
            .push(compact_metric("File Reader", "23 fois"))
            .push(compact_metric("Code Analyzer", "15 fois"))
            .push(compact_metric("Rust Compiler", "8 fois"))
            .push(compact_metric("Terminal", "5 fois"))
            .push(
                button::text("Voir tous les outils")
                    .on_press(AppEvent::None)
                    .padding([tokens::SPACE_XS, 0])
                    .class(cosmic::theme::Button::Text),
            ),
    )
    .width(Length::Fill)
    .padding(tokens::CARD_PADDING)
    .class(cosmic::theme::sory::context_content())
    .into()
}

fn edge_rail() -> Element<'static, AppEvent> {
    let icons = [
        tokens::ICON_HOME,
        tokens::ICON_WORKSPACE,
        tokens::ICON_CODE,
        tokens::ICON_TOOLS,
        tokens::ICON_SETTINGS,
    ];

    let mut col = column(Vec::new()).spacing(tokens::SPACE_SM);
    for icon in icons {
        col = col.push(
            button::text(icon)
                .on_press(AppEvent::None)
                .padding(tokens::SPACE_XS)
                .class(cosmic::theme::Button::Text),
        );
    }

    container(col)
        .padding([tokens::SPACE_MD, tokens::SPACE_XS])
        .class(cosmic::theme::sory::sidebar())
        .into()
}

pub fn view(state: &AppState) -> Element<AppEvent> {
    let width = workspace_width(state);

    if state.workspace_sidebar.expand_progress < 0.05 {
        return container(edge_rail())
            .width(Length::Fixed(40.0))
            .height(Length::Fill)
            .into();
    }

    let open_files = state
        .conversations
        .active()
        .map(|c| c.messages.len())
        .unwrap_or(0)
        .min(99);

    let content = scrollable(
        column(Vec::new())
            .spacing(tokens::SPACE_MD)
            .push(panel_header(state))
            .push(workspace_info(state))
            .push(compact_metric("Fichiers ouverts", &open_files.to_string()))
            .push(compact_metric("Modifications", "3"))
            .push(search_bar())
            .push(context_card())
            .push(tasks_card(state))
            .push(tools_card()),
    )
    .height(Length::Fill);

    row(Vec::new())
        .push(
            container(content)
                .width(width)
                .height(Length::Fill)
                .padding(tokens::PANEL_PADDING)
                .class(cosmic::theme::sory::context_drawer()),
        )
        .push(edge_rail())
        .height(Length::Fill)
        .into()
}
