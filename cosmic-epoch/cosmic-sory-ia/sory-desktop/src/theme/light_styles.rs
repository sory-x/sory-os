// SPDX-License-Identifier: GPL-3.0-only

//! Light Styles pour Sory Desktop — styles composants style ChatGPT.
//!
//! Styles pour les containers, boutons, et autres composants en light theme.

use crate::iced::{Background, Border, Color, Shadow, Vector};
use crate::theme::light_theme::LIGHT;
use crate::widget::container;

// ═════════════════════════════════════════════════════════════════════════════
// HELPERS INTERNES
// ═════════════════════════════════════════════════════════════════════════════

fn drop_shadow(color: Color, offset_y: f32, blur: f32) -> Shadow {
    Shadow {
        color,
        offset: Vector::new(0.0, offset_y),
        blur_radius: blur,
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// SIDEBAR
// ═════════════════════════════════════════════════════════════════════════════

/// Fond de sidebar light.
#[must_use]
pub fn sidebar() -> crate::theme::Container<'static> {
    crate::theme::Container::custom(|_| container::Style {
        icon_color: Some(LIGHT.TEXT_PRIMARY),
        text_color: Some(LIGHT.TEXT_PRIMARY),
        background: Some(Background::Color(LIGHT.BG_SECONDARY)),
        border: Border {
            color: LIGHT.BORDER_DEFAULT,
            width: 1.0,
            radius: 0.0.into(),
        },
        shadow: Default::default(),
        snap: true,
    })
}

/// Item de navigation sidebar — état inactif.
#[must_use]
pub fn sidebar_item() -> crate::theme::Container<'static> {
    crate::theme::Container::custom(|_| container::Style {
        icon_color: Some(LIGHT.TEXT_SECONDARY),
        text_color: Some(LIGHT.TEXT_SECONDARY),
        background: None,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: LIGHT.RADIUS_SMALL.into(),
            ..Default::default()
        },
        shadow: Default::default(),
        snap: true,
    })
}

/// Item de navigation sidebar — état actif.
#[must_use]
pub fn sidebar_item_active() -> crate::theme::Container<'static> {
    crate::theme::Container::custom(|_| container::Style {
        icon_color: Some(LIGHT.ACCENT_BLUE),
        text_color: Some(LIGHT.TEXT_PRIMARY),
        background: Some(Background::Color(LIGHT.BG_SELECTED)),
        border: Border {
            color: LIGHT.BORDER_SELECTED,
            width: 1.0,
            radius: LIGHT.RADIUS_SMALL.into(),
        },
        shadow: Default::default(),
        snap: true,
    })
}

/// Bande accent verticale dans la sidebar (indicateur actif).
#[must_use]
pub fn sidebar_accent_bar() -> crate::theme::Container<'static> {
    crate::theme::Container::custom(|_| container::Style {
        background: Some(Background::Color(LIGHT.ACCENT_BLUE)),
        border: Border {
            radius: [0.0, 4.0, 4.0, 0.0].into(),
            ..Default::default()
        },
        shadow: Default::default(),
        ..Default::default()
    })
}

/// Footer de sidebar.
#[must_use]
pub fn sidebar_footer() -> crate::theme::Container<'static> {
    crate::theme::Container::custom(|_| container::Style {
        icon_color: Some(LIGHT.TEXT_PRIMARY),
        text_color: Some(LIGHT.TEXT_PRIMARY),
        background: Some(Background::Color(LIGHT.BG_HOVER)),
        border: Border {
            color: LIGHT.BORDER_SUBTLE,
            width: 1.0,
            radius: LIGHT.RADIUS_MEDIUM.into(),
        },
        shadow: Default::default(),
        snap: true,
    })
}

// ═════════════════════════════════════════════════════════════════════════════
// HEADER
// ═════════════════════════════════════════════════════════════════════════════

/// Barre d'en-tête light.
#[must_use]
pub fn header_bar() -> crate::theme::Container<'static> {
    crate::theme::Container::custom(|_| container::Style {
        icon_color: Some(LIGHT.TEXT_PRIMARY),
        text_color: Some(LIGHT.TEXT_PRIMARY),
        background: Some(Background::Color(LIGHT.BG_PRIMARY)),
        border: Border {
            color: LIGHT.BORDER_SUBTLE,
            width: 0.0,
            radius: 0.0.into(),
        },
        shadow: drop_shadow(LIGHT.SHADOW_SUBTLE, 1.0, 3.0),
        snap: true,
    })
}

/// Status badge light.
#[must_use]
pub fn status_badge() -> crate::theme::Container<'static> {
    crate::theme::Container::custom(|_| container::Style {
        icon_color: Some(LIGHT.TEXT_PRIMARY),
        text_color: Some(LIGHT.TEXT_SECONDARY),
        background: Some(Background::Color(LIGHT.BG_HOVER)),
        border: Border {
            color: LIGHT.BORDER_DEFAULT,
            width: 1.0,
            radius: LIGHT.RADIUS_SMALL.into(),
        },
        shadow: Default::default(),
        snap: true,
    })
}

// ═════════════════════════════════════════════════════════════════════════════
// CARDS
// ═════════════════════════════════════════════════════════════════════════════

/// Carte light — état normal.
#[must_use]
pub fn card() -> crate::theme::Container<'static> {
    crate::theme::Container::custom(|_| container::Style {
        icon_color: Some(LIGHT.TEXT_PRIMARY),
        text_color: Some(LIGHT.TEXT_PRIMARY),
        background: Some(Background::Color(LIGHT.BG_CARD)),
        border: Border {
            color: LIGHT.BORDER_DEFAULT,
            width: 1.0,
            radius: LIGHT.RADIUS_MEDIUM.into(),
        },
        shadow: drop_shadow(LIGHT.SHADOW_SUBTLE, 1.0, 3.0),
        snap: true,
    })
}

/// Carte light — état hover.
#[must_use]
pub fn card_hover() -> crate::theme::Container<'static> {
    crate::theme::Container::custom(|_| container::Style {
        icon_color: Some(LIGHT.TEXT_PRIMARY),
        text_color: Some(LIGHT.TEXT_PRIMARY),
        background: Some(Background::Color(LIGHT.BG_HOVER)),
        border: Border {
            color: LIGHT.BORDER_HOVER,
            width: 1.0,
            radius: LIGHT.RADIUS_MEDIUM.into(),
        },
        shadow: drop_shadow(LIGHT.SHADOW_MEDIUM, 2.0, 4.0),
        snap: true,
    })
}

/// Carte light — état sélectionné.
#[must_use]
pub fn card_selected() -> crate::theme::Container<'static> {
    crate::theme::Container::custom(|_| container::Style {
        icon_color: Some(LIGHT.ACCENT_BLUE),
        text_color: Some(LIGHT.TEXT_PRIMARY),
        background: Some(Background::Color(LIGHT.BG_SELECTED)),
        border: Border {
            color: LIGHT.BORDER_SELECTED,
            width: 1.0,
            radius: LIGHT.RADIUS_MEDIUM.into(),
        },
        shadow: drop_shadow(LIGHT.SHADOW_MEDIUM, 2.0, 4.0),
        snap: true,
    })
}

// ═════════════════════════════════════════════════════════════════════════════
// INPUT AREA
// ═════════════════════════════════════════════════════════════════════════════

/// Zone de saisie light.
#[must_use]
pub fn input_area() -> crate::theme::Container<'static> {
    crate::theme::Container::custom(|_| container::Style {
        icon_color: Some(LIGHT.TEXT_PRIMARY),
        text_color: Some(LIGHT.TEXT_PRIMARY),
        background: Some(Background::Color(LIGHT.BG_INPUT)),
        border: Border {
            color: LIGHT.BORDER_DEFAULT,
            width: 1.0,
            radius: LIGHT.RADIUS_INPUT.into(),
        },
        shadow: drop_shadow(LIGHT.SHADOW_SUBTLE, 1.0, 2.0),
        snap: true,
    })
}

/// Zone de saisie light — état focus.
#[must_use]
pub fn input_area_focus() -> crate::theme::Container<'static> {
    crate::theme::Container::custom(|_| container::Style {
        icon_color: Some(LIGHT.TEXT_PRIMARY),
        text_color: Some(LIGHT.TEXT_PRIMARY),
        background: Some(Background::Color(LIGHT.BG_INPUT)),
        border: Border {
            color: LIGHT.ACCENT_BLUE,
            width: 2.0,
            radius: LIGHT.RADIUS_INPUT.into(),
        },
        shadow: drop_shadow(LIGHT.SHADOW_MEDIUM, 1.0, 3.0),
        snap: true,
    })
}

// ═════════════════════════════════════════════════════════════════════════════
// BUTTONS
// ═════════════════════════════════════════════════════════════════════════════

/// Bouton principal light.
#[must_use]
pub fn button_primary() -> crate::theme::Container<'static> {
    crate::theme::Container::custom(|_| container::Style {
        icon_color: Some(LIGHT.TEXT_ON_ACCENT),
        text_color: Some(LIGHT.TEXT_ON_ACCENT),
        background: Some(Background::Color(LIGHT.ACCENT_BLUE)),
        border: Border {
            radius: LIGHT.RADIUS_SMALL.into(),
            ..Default::default()
        },
        shadow: drop_shadow(LIGHT.SHADOW_SUBTLE, 1.0, 2.0),
        snap: true,
    })
}

/// Bouton principal light — état hover.
#[must_use]
pub fn button_primary_hover() -> crate::theme::Container<'static> {
    crate::theme::Container::custom(|_| container::Style {
        icon_color: Some(LIGHT.TEXT_ON_ACCENT),
        text_color: Some(LIGHT.TEXT_ON_ACCENT),
        background: Some(Background::Color(LIGHT.ACCENT_HOVER)),
        border: Border {
            radius: LIGHT.RADIUS_SMALL.into(),
            ..Default::default()
        },
        shadow: drop_shadow(LIGHT.SHADOW_MEDIUM, 1.0, 3.0),
        snap: true,
    })
}

/// Bouton secondaire light.
#[must_use]
pub fn button_secondary() -> crate::theme::Container<'static> {
    crate::theme::Container::custom(|_| container::Style {
        icon_color: Some(LIGHT.TEXT_PRIMARY),
        text_color: Some(LIGHT.TEXT_PRIMARY),
        background: Some(Background::Color(LIGHT.BG_HOVER)),
        border: Border {
            color: LIGHT.BORDER_DEFAULT,
            width: 1.0,
            radius: LIGHT.RADIUS_SMALL.into(),
        },
        shadow: Default::default(),
        snap: true,
    })
}

// ═════════════════════════════════════════════════════════════════════════════
// BACKGROUND
// ═════════════════════════════════════════════════════════════════════════════

/// Fond principal light.
#[must_use]
pub fn background() -> crate::theme::Container<'static> {
    crate::theme::Container::custom(|_| container::Style {
        icon_color: Some(LIGHT.TEXT_PRIMARY),
        text_color: Some(LIGHT.TEXT_PRIMARY),
        background: Some(Background::Color(LIGHT.BG_PRIMARY)),
        border: Border::default(),
        shadow: Default::default(),
        snap: true,
    })
}

/// Fond de contenu light.
#[must_use]
pub fn content_background() -> crate::theme::Container<'static> {
    crate::theme::Container::custom(|_| container::Style {
        icon_color: Some(LIGHT.TEXT_PRIMARY),
        text_color: Some(LIGHT.TEXT_PRIMARY),
        background: Some(Background::Color(LIGHT.BG_PRIMARY)),
        border: Border::default(),
        shadow: Default::default(),
        snap: true,
    })
}
