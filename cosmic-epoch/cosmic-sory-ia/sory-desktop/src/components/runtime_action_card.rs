use cosmic::{
    Element,
    iced::{Alignment, Length},
    widget::{self, Space, button, column, container, row},
};

use crate::{
    events::AppEvent,
    models::{RuntimeAction, RuntimeActionKind, RuntimeActionStatus},
    theme::tokens,
};

pub fn view(action: &RuntimeAction) -> Element<AppEvent> {
    let (icon, label, helper, color) = match action.kind {
        RuntimeActionKind::Permission => (
            "\u{1f512}",
            "Permission",
            "Sory IA demande ton accord avant de continuer.",
            cosmic::palette::SORY.ACCENT_ORANGE,
        ),
        RuntimeActionKind::Question => (
            "\u{2753}",
            "Question",
            "Une r\u{00e9}ponse est n\u{00e9}cessaire pour poursuivre la t\u{00e2}che.",
            cosmic::palette::SORY.ACCENT,
        ),
        RuntimeActionKind::ToolApproval => (
            "\u{2699}",
            "Autorisation outil",
            "Un outil va agir sur le workspace ou l'environnement.",
            cosmic::palette::SORY.ACCENT_PURPLE,
        ),
    };

    let (status_text, status_color) = match action.status {
        RuntimeActionStatus::Pending => ("En attente", cosmic::palette::SORY.ACCENT_BRIGHT),
        RuntimeActionStatus::Resolved => ("R\u{00e9}solu", cosmic::palette::SORY.ACCENT_GREEN),
        RuntimeActionStatus::Cancelled => ("Annul\u{00e9}", cosmic::palette::SORY.TEXT_MUTED),
    };

    let tile = match action.kind {
        RuntimeActionKind::Permission => cosmic::theme::sory::tile_icon_orange(),
        RuntimeActionKind::Question => cosmic::theme::sory::tile_icon_blue(),
        RuntimeActionKind::ToolApproval => cosmic::theme::sory::tile_icon_purple(),
    };

    let card_class = match action.kind {
        RuntimeActionKind::Permission => cosmic::theme::sory::dialog_panel(),
        RuntimeActionKind::Question => cosmic::theme::sory::dialog_panel(),
        RuntimeActionKind::ToolApproval => cosmic::theme::sory::dialog_panel(),
    };

    let buttons: Element<AppEvent> = match action.status {
        RuntimeActionStatus::Pending => row(Vec::new())
            .spacing(tokens::SPACE_SM)
            .push(
                button::text("Autoriser")
                    .on_press(AppEvent::RuntimeActionResolve {
                        action_id: action.id,
                        decision: "accept".into(),
                    })
                    .class(cosmic::theme::Button::Suggested),
            )
            .push(
                button::text("Refuser")
                    .on_press(AppEvent::RuntimeActionReject {
                        action_id: action.id,
                    })
                    .class(cosmic::theme::Button::Destructive),
            )
            .into(),
        RuntimeActionStatus::Resolved => row(Vec::new())
            .push(
                widget::text::colored("\u{2713} R\u{00e9}solu", cosmic::palette::SORY.ACCENT_GREEN)
                    .size(f32::from(tokens::FONT_SM)),
            )
            .into(),
        RuntimeActionStatus::Cancelled => row(Vec::new())
            .push(
                widget::text::colored("Refus\u{00e9}", cosmic::palette::SORY.TEXT_MUTED)
                    .size(f32::from(tokens::FONT_SM)),
            )
            .into(),
    };

    container(
        column(Vec::new())
            .spacing(tokens::SPACE_SM)
            .push(
                row(Vec::new())
                    .spacing(tokens::SPACE_SM)
                    .align_y(Alignment::Center)
                    .push(
                        container(widget::text(icon).size(f32::from(tokens::FONT_MD)))
                            .padding(tokens::SPACE_XXS)
                            .class(tile),
                    )
                    .push(
                        widget::text::colored(label, color)
                            .size(f32::from(tokens::FONT_MD))
                            .font(cosmic::font::semibold()),
                    )
                    .push(Space::new().width(Length::Fill))
                    .push(
                        widget::text::colored(status_text, status_color)
                            .size(f32::from(tokens::FONT_XS))
                            .font(cosmic::font::semibold()),
                    ),
            )
            .push(
                widget::text::colored(helper, cosmic::palette::SORY.TEXT_SECONDARY)
                    .size(f32::from(tokens::FONT_SM))
                    .font(cosmic::font::default()),
            )
            .push(
                container(
                    column(Vec::new())
                        .spacing(tokens::SPACE_XS)
                        .push(
                            widget::text(&action.title)
                                .size(f32::from(tokens::FONT_MD))
                                .font(cosmic::font::semibold()),
                        )
                        .push(
                            widget::text(&action.details)
                                .size(f32::from(tokens::FONT_SM))
                                .font(cosmic::font::default()),
                        ),
                )
                .padding(tokens::SPACE_SM)
                .class(cosmic::theme::sory::context_content()),
            )
            .push(buttons),
    )
    .width(Length::Fill)
    .padding(tokens::CARD_PADDING)
    .class(card_class)
    .into()
}
