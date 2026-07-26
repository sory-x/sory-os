use cosmic::{
    Element,
    iced::{Alignment, Length},
    widget::{self, column, container, row},
};

use crate::{
    events::AppEvent,
    models::{ToolCall, ToolStatus},
    theme::tokens,
};

fn tile_style(status: ToolStatus) -> cosmic::theme::Container<'static> {
    match status {
        ToolStatus::Started => cosmic::theme::sory::tile_icon_blue(),
        ToolStatus::Finished => cosmic::theme::sory::tile_icon_green(),
        ToolStatus::Failed => cosmic::theme::sory::tile_icon_orange(),
    }
}

fn default_summary(status: ToolStatus) -> &'static str {
    match status {
        ToolStatus::Started => "Sory IA travaille sur cette \u{00e9}tape\u{2026}",
        ToolStatus::Finished => "\u{00c9}tape termin\u{00e9}e.",
        ToolStatus::Failed => "Cette \u{00e9}tape doit \u{00ea}tre v\u{00e9}rifi\u{00e9}e.",
    }
}

pub fn header<'a>(tool: &'a ToolCall) -> Element<'a, AppEvent> {
    let (icon, label, color) = match tool.status {
        ToolStatus::Started => ("\u{21bb}", "En cours", cosmic::palette::SORY.ACCENT_BRIGHT),
        ToolStatus::Finished => ("\u{2713}", "Termin\u{00e9}", cosmic::palette::SORY.ACCENT_GREEN),
        ToolStatus::Failed => ("\u{2717}", "\u{00c9}chec", cosmic::palette::SORY.ACCENT_RED),
    };
    let summary = tool.summary.as_deref().unwrap_or(default_summary(tool.status));

    row(Vec::new())
        .spacing(tokens::SPACE_SM)
        .align_y(Alignment::Center)
        .push(
            container(widget::text::colored(icon, color).size(f32::from(tokens::FONT_MD)))
                .padding(tokens::SPACE_XXS)
                .class(tile_style(tool.status)),
        )
        .push(
            column(Vec::new())
                .spacing(tokens::SPACE_XXS)
                .push(
                    widget::text(&tool.name)
                        .size(f32::from(tokens::FONT_MD))
                        .font(cosmic::font::semibold()),
                )
                .push(
                    widget::text::colored(summary, cosmic::palette::SORY.TEXT_SECONDARY)
                        .size(f32::from(tokens::FONT_SM)),
                ),
        )
        .push(cosmic::widget::Space::new().width(Length::Fill))
        .push(
            widget::text::colored(label, color)
                .size(f32::from(tokens::FONT_XS))
                .font(cosmic::font::semibold()),
        )
        .into()
}

pub fn compact<'a>(tool: &'a ToolCall) -> Element<'a, AppEvent> {
    let (icon, label, color) = match tool.status {
        ToolStatus::Started => ("\u{21bb}", "En cours", cosmic::palette::SORY.ACCENT_BRIGHT),
        ToolStatus::Finished => ("\u{2713}", "Termin\u{00e9}", cosmic::palette::SORY.ACCENT_GREEN),
        ToolStatus::Failed => ("\u{2717}", "\u{00c9}chec", cosmic::palette::SORY.ACCENT_RED),
    };
    let summary = tool.summary.as_deref().unwrap_or(default_summary(tool.status));

    row(Vec::new())
        .spacing(tokens::SPACE_SM)
        .align_y(Alignment::Center)
        .push(
            container(widget::text::colored(icon, color).size(f32::from(tokens::FONT_MD)))
                .padding(tokens::SPACE_XXS)
                .class(tile_style(tool.status)),
        )
        .push(
            widget::text(&tool.name)
                .size(f32::from(tokens::FONT_MD))
                .font(cosmic::font::semibold()),
        )
        .push(
            widget::text::colored(summary, cosmic::palette::SORY.TEXT_SECONDARY)
                .size(f32::from(tokens::FONT_SM)),
        )
        .push(cosmic::widget::Space::new().width(Length::Fill))
        .push(
            widget::text::colored(label, color)
                .size(f32::from(tokens::FONT_XS))
                .font(cosmic::font::semibold()),
        )
        .into()
}
