use cosmic::{
    Element,
    iced::Length,
    widget::{self, row},
};

use crate::{events::AppEvent, models::ToolStatus, theme::tokens};

pub fn view<'a>(name: &'a str, status: ToolStatus) -> Element<'a, AppEvent> {
    let (label, color) = match status {
        ToolStatus::Started => (
            format!("{name} \u{2192} En cours\u{2026}"),
            cosmic::palette::SORY.ACCENT_BRIGHT,
        ),
        ToolStatus::Finished => (
            format!("{name} \u{2192} Termin\u{00e9}"),
            cosmic::palette::SORY.ACCENT_GREEN,
        ),
        ToolStatus::Failed => (
            format!("{name} \u{2192} \u{00c9}chec"),
            cosmic::palette::SORY.ACCENT_RED,
        ),
    };

    row(Vec::new())
        .push(
            widget::text::colored(label, color)
                .size(f32::from(tokens::FONT_SM))
                .font(cosmic::font::semibold()),
        )
        .push(cosmic::widget::Space::new().width(Length::Fill))
        .into()
}
