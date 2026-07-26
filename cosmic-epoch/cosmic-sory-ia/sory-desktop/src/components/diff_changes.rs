use cosmic::{
    Element,
    iced::{Alignment, Length},
    widget::{self, column, container, row},
};

use crate::{
    events::AppEvent,
    models::{DiffFile, DiffStatus},
    theme::tokens,
};

pub fn view<'a>(files: &'a [DiffFile]) -> Element<'a, AppEvent> {
    if files.is_empty() {
        return column(Vec::new()).into();
    }

    let total_added: usize = files.iter().map(|f| f.added).sum();
    let total_removed: usize = files.iter().map(|f| f.removed).sum();
    let total_modified: usize = files.iter().filter(|f| f.status == DiffStatus::Modified).count();
    let total_created: usize = files.iter().filter(|f| f.status == DiffStatus::Created).count();
    let total_deleted: usize = files.iter().filter(|f| f.status == DiffStatus::Deleted).count();

    let mut items = column(Vec::new()).spacing(tokens::SPACE_XXS);

    for file in files {
        let (status_icon, color) = match file.status {
            DiffStatus::Modified => ("\u{270f}", cosmic::palette::SORY.ACCENT),
            DiffStatus::Created => ("\u{2795}", cosmic::palette::SORY.ACCENT_GREEN),
            DiffStatus::Deleted => ("\u{2716}", cosmic::palette::SORY.ACCENT_RED),
        };

        items = items.push(
            row(Vec::new())
                .spacing(tokens::SPACE_XS)
                .align_y(Alignment::Center)
                .push(
                    widget::text::colored(status_icon, color)
                        .size(f32::from(tokens::FONT_XS)),
                )
                .push(
                    widget::text(&file.file_path)
                        .size(f32::from(tokens::FONT_SM))
                        .font(cosmic::font::default()),
                )
                .push(cosmic::widget::Space::new().width(Length::Fill))
                .push(
                    widget::text::colored(
                        format!("+{} -{}", file.added, file.removed),
                        cosmic::palette::SORY.TEXT_SECONDARY,
                    )
                    .size(f32::from(tokens::FONT_XS)),
                ),
        );
    }

    container(
        column(Vec::new())
            .spacing(tokens::SPACE_SM)
            .push(
                row(Vec::new())
                    .spacing(tokens::SPACE_SM)
                    .align_y(Alignment::Center)
                    .push(
                        widget::text::colored(
                            "\u{1f4c4} Fichiers modifi\u{00e9}s",
                            cosmic::palette::SORY.TEXT_SECONDARY,
                        )
                        .size(f32::from(tokens::FONT_SM))
                        .font(cosmic::font::semibold()),
                    )
                    .push(cosmic::widget::Space::new().width(Length::Fill))
                    .push(
                        widget::text::colored(
                            format!("+{total_added} -{total_removed}"),
                            cosmic::palette::SORY.TEXT_MUTED,
                        )
                        .size(f32::from(tokens::FONT_XS)),
                    ),
            )
            .push(items)
            .push(
                widget::text::colored(
                    format!(
                        "{} modifi\u{00e9}{}, {} cr\u{00e9}{}, {} supprim\u{00e9}{}",
                        total_modified,
                        if total_modified > 1 { "s" } else { "" },
                        total_created,
                        if total_created > 1 { "s" } else { "" },
                        total_deleted,
                        if total_deleted > 1 { "s" } else { "" },
                    ),
                    cosmic::palette::SORY.TEXT_MUTED,
                )
                .size(f32::from(tokens::FONT_XS)),
            ),
    )
    .width(Length::Fill)
    .padding(tokens::SPACE_SM)
    .class(cosmic::theme::sory::context_content())
    .into()
}
