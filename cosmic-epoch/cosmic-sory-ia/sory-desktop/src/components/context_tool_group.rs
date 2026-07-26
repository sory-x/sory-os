use cosmic::{
    Element,
    iced::{Alignment, Length},
    widget::{self, column, container, row},
};

use crate::{
    events::AppEvent,
    models::ToolCall,
    theme::tokens,
};

pub fn view<'a>(tools: &'a [ToolCall]) -> Element<'a, AppEvent> {
    if tools.is_empty() {
        return column(Vec::new()).into();
    }

    let count = tools.len();

    let mut items = column(Vec::new()).spacing(tokens::SPACE_XXS);

    for tool in tools {
        let icon = match tool.name.as_str() {
            n if n.contains("read") => "\u{1f4d6}",
            n if n.contains("glob") || n.contains("search") => "\u{1f50d}",
            n if n.contains("grep") => "\u{1f50e}",
            n if n.contains("list") => "\u{1f4c2}",
            n if n.contains("write") || n.contains("create") => "\u{270f}",
            n if n.contains("edit") || n.contains("replace") => "\u{1f589}",
            n if n.contains("bash") || n.contains("run") || n.contains("execute") => "\u{1f5a5}",
            n if n.contains("web") || n.contains("fetch") || n.contains("curl") => "\u{1f310}",
            _ => "\u{2699}",
        };

        items = items.push(
            row(Vec::new())
                .spacing(tokens::SPACE_XS)
                .align_y(Alignment::Center)
                .push(
                    widget::text::colored(icon, cosmic::palette::SORY.TEXT_SECONDARY)
                        .size(f32::from(tokens::FONT_SM)),
                )
                .push(
                    widget::text(&tool.name)
                        .size(f32::from(tokens::FONT_SM))
                        .font(cosmic::font::default()),
                ),
        );
    }

    container(
        column(Vec::new())
            .spacing(tokens::SPACE_XS)
            .push(
                row(Vec::new())
                    .spacing(tokens::SPACE_SM)
                    .align_y(Alignment::Center)
                    .push(
                        widget::text::colored(
                            "\u{1f4e1} Contexte",
                            cosmic::palette::SORY.TEXT_SECONDARY,
                        )
                        .size(f32::from(tokens::FONT_SM))
                        .font(cosmic::font::semibold()),
                    )
                    .push(cosmic::widget::Space::new().width(Length::Fill))
                    .push(
                        widget::text::colored(
                            format!("{count} outils"),
                            cosmic::palette::SORY.TEXT_MUTED,
                        )
                        .size(f32::from(tokens::FONT_XS)),
                    ),
            )
            .push(items),
    )
    .width(Length::Fill)
    .padding(tokens::SPACE_SM)
    .class(cosmic::theme::sory::context_content())
    .into()
}
