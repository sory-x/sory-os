// SPDX-License-Identifier: GPL-3.0-only

//! Rendu markdown natif — fidèle à la maquette officielle.

use cosmic::{
    Element,
    widget::{self, column, container, row},
};

use crate::{events::AppEvent, theme::tokens};

pub fn view(markdown: &str) -> Element<AppEvent> {
    let mut col = column(Vec::new()).spacing(tokens::SPACE_XS);
    let mut code_lines: Vec<String> = Vec::new();
    let mut in_code_block = false;

    for line in markdown.lines() {
        let trimmed = line.trim();

        if in_code_block {
            if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
                col = col.push(render_code_block(code_lines.join("\n")));
                code_lines.clear();
                in_code_block = false;
            } else {
                code_lines.push(line.to_string());
            }
            continue;
        }

        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            in_code_block = true;
            continue;
        }

        if trimmed.is_empty() {
            col = col.push(cosmic::widget::Space::new().height(tokens::SPACE_SM));
            continue;
        }

        if trimmed.starts_with("### ") {
            col = col.push(
                widget::text(&trimmed[4..])
                    .size(tokens::FONT_MD_H3)
                    .font(cosmic::font::semibold()),
            );
        } else if trimmed.starts_with("## ") {
            col = col.push(
                widget::text(&trimmed[3..])
                    .size(tokens::FONT_MD_H2)
                    .font(cosmic::font::semibold()),
            );
        } else if trimmed.starts_with("# ") {
            col = col.push(
                widget::text(&trimmed[2..])
                    .size(tokens::FONT_MD_H1)
                    .font(cosmic::font::bold()),
            );
        } else if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
            let item: Element<AppEvent> = row(Vec::new())
                .spacing(tokens::SPACE_XS)
                .push(
                    widget::text::colored(
                        "\u{2022}",
                        cosmic::palette::SORY.TEXT_SECONDARY,
                    )
                    .size(tokens::FONT_LG),
                )
                .push(
                    widget::text(&trimmed[2..])
                        .size(tokens::FONT_LG)
                        .font(cosmic::font::default()),
                )
                .into();
            col = col.push(item);
        } else if trimmed.starts_with("1. ")
            || trimmed.starts_with("2. ")
            || trimmed.starts_with("3. ")
            || trimmed.starts_with("4. ")
        {
            let num = &trimmed[..1];
            let text = &trimmed[3..];
            let item: Element<AppEvent> = row(Vec::new())
                .spacing(tokens::SPACE_XS)
                .push(
                    widget::text::colored(
                        num,
                        cosmic::palette::SORY.ACCENT_BRIGHT,
                    )
                    .size(tokens::FONT_LG)
                    .font(cosmic::font::semibold()),
                )
                .push(
                    widget::text(text)
                        .size(tokens::FONT_LG)
                        .font(cosmic::font::default()),
                )
                .into();
            col = col.push(item);
        } else if trimmed.chars().all(|c| c == '-' || c == '=' || c == '*') && trimmed.len() >= 3 {
            let rule: Element<AppEvent> = container(cosmic::widget::Space::new().height(1))
                .width(cosmic::iced::Length::Fill)
                .class(cosmic::theme::sory::context_content())
                .into();
            col = col.push(rule);
        } else {
            let text = render_inline_markdown(line);
            col = col.push(text);
        }
    }

    if in_code_block && !code_lines.is_empty() {
        col = col.push(render_code_block(code_lines.join("\n")));
    }

    col.into()
}

fn render_inline_markdown(text: &str) -> Element<AppEvent> {
    let mut segments: Vec<Element<AppEvent>> = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        if i + 1 < len && chars[i] == '*' && chars[i + 1] == '*' {
            let rest = &text[i + 2..];
            let end = rest.find("**");
            if let Some(pos) = end {
                let bold_text = &text[i + 2..i + 2 + pos];
                segments.push(
                    widget::text(bold_text)
                        .size(tokens::FONT_LG)
                        .font(cosmic::font::bold())
                        .into(),
                );
                i += 4 + pos;
                continue;
            }
        }

        if chars[i] == '*' || chars[i] == '_' {
            let search_char = if chars[i] == '*' { '*' } else { '_' };
            let rest = &text[i + 1..];
            let end = rest.find(search_char);
            if let Some(pos) = end {
                let italic_text = &text[i + 1..i + 1 + pos];
                segments.push(
                    widget::text(italic_text)
                        .size(tokens::FONT_LG)
                        .font(cosmic::font::default())
                        .into(),
                );
                i += 2 + pos;
                continue;
            }
        }

        if chars[i] == '`' {
            let rest = &text[i + 1..];
            let end = rest.find('`');
            if let Some(pos) = end {
                let code_text = &text[i + 1..i + 1 + pos];
                segments.push(
                    container(
                        widget::text(code_text)
                            .size(tokens::FONT_CODE)
                            .font(cosmic::font::default()),
                    )
                    .padding([1, tokens::SPACE_XS])
                    .class(cosmic::theme::sory::context_content())
                    .into(),
                );
                i += 2 + pos;
                continue;
            }
        }

        let start = i;
        while i < len && !(chars[i] == '*' || chars[i] == '_' || chars[i] == '`') {
            i += 1;
        }
        let plain = &text[start..i];
        if !plain.is_empty() {
            segments.push(
                widget::text(plain)
                    .size(tokens::FONT_LG)
                    .font(cosmic::font::default())
                    .into(),
            );
        }
    }

    if segments.is_empty() {
        return widget::text(text)
            .size(tokens::FONT_LG)
            .font(cosmic::font::default())
            .into();
    }

    let mut row_widget = row(Vec::new()).spacing(2);
    for seg in segments {
        row_widget = row_widget.push(seg);
    }
    row_widget.into()
}

fn render_code_block(code: String) -> Element<'static, AppEvent> {
    let code_element: Element<AppEvent> = container(
        widget::text(code)
            .size(tokens::FONT_CODE)
            .font(cosmic::font::default()),
    )
    .width(cosmic::iced::Length::Fill)
    .padding(tokens::SPACE_SM)
    .class(cosmic::theme::sory::context_content())
    .into();

    container(
        column(Vec::new())
            .spacing(tokens::SPACE_XS)
            .push(
                row(Vec::new())
                    .push(
                        widget::text::colored(
                            "\u{2398} Code",
                            cosmic::palette::SORY.TEXT_SECONDARY,
                        )
                        .size(tokens::FONT_XS),
                    )
                    .push(cosmic::widget::Space::new().width(cosmic::iced::Length::Fill))
                    .push(
                        widget::button::text(tokens::ICON_COPY)
                            .on_press(AppEvent::None)
                            .padding(tokens::SPACE_XS)
                            .class(cosmic::theme::Button::Text),
                    ),
            )
            .push(code_element),
    )
    .width(cosmic::iced::Length::Fill)
    .padding([tokens::SPACE_XS, 0])
    .into()
}
