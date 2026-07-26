// SPDX-License-Identifier: GPL-3.0-only

//! Light Theme pour Sory Desktop — interface style ChatGPT.
//!
//! Palette de couleurs fond clair épuré comme l'interface originale.

use crate::iced::Color;

/// Convertit un code hexadécimal en `Color` RGB (alpha = 1.0).
#[must_use]
#[inline]
const fn rgb(hex: u32) -> Color {
    Color::from_rgb(
        ((hex >> 16) & 0xff) as f32 / 255.0,
        ((hex >> 8) & 0xff) as f32 / 255.0,
        (hex & 0xff) as f32 / 255.0,
    )
}

/// Crée une `Color` avec un alpha spécifique.
#[must_use]
#[inline]
const fn with_alpha(color: Color, a: f32) -> Color {
    Color { a, ..color }
}

/// Tous les tokens de design Light Theme — style ChatGPT.
pub static LIGHT: LightTokens = LightTokens {
    // Backgrounds
    BG_PRIMARY: rgb(0xffffff),      // Fond principal (blanc)
    BG_SECONDARY: rgb(0xf9fafb),    // Fond sidebar (gris très clair)
    BG_CARD: rgb(0xffffff),         // Fond cartes (blanc)
    BG_INPUT: rgb(0xffffff),         // Fond input (blanc)
    BG_HOVER: rgb(0xf3f4f6),        // Hover (gris clair)
    BG_SELECTED: rgb(0xeef2ff),     // Sélection (bleu très clair)
    
    // Text
    TEXT_PRIMARY: rgb(0x111827),    // Texte principal (noir presque)
    TEXT_SECONDARY: rgb(0x6b7280),   // Texte secondaire (gris)
    TEXT_MUTED: rgb(0x9ca3af),      // Texte atténué (gris clair)
    TEXT_ON_ACCENT: Color::WHITE,    // Texte sur accent
    
    // Accent colors
    ACCENT_BLUE: rgb(0x3b82f6),     // Bleu principal
    ACCENT_HOVER: rgb(0x2563eb),    // Hover bleu
    ACCENT_DARK: rgb(0x1e40af),     // Bleu sombre
    ACCENT_GREEN: rgb(0x10b981),    // Vert succès
    ACCENT_RED: rgb(0xef4444),      // Rouge erreur
    ACCENT_ORANGE: rgb(0xf59e0b),   // Orange warning
    
    // Borders
    BORDER_DEFAULT: rgb(0xe5e7eb),  // Bordure principale
    BORDER_HOVER: rgb(0xd1d5db),    // Bordure hover
    BORDER_SELECTED: rgb(0x3b82f6), // Bordure sélection
    BORDER_SUBTLE: rgb(0xf3f4f6),  // Bordure subtile
    
    // Shadows
    SHADOW_SUBTLE: with_alpha(rgb(0x000000), 0.05),
    SHADOW_MEDIUM: with_alpha(rgb(0x000000), 0.1),
    SHADOW_STRONG: with_alpha(rgb(0x000000), 0.15),
    
    // Spacing & Radius
    RADIUS_SMALL: 6.0,   // Boutons petits
    RADIUS_MEDIUM: 8.0,  // Cartes
    RADIUS_LARGE: 12.0,  // Dialogues
    RADIUS_INPUT: 8.0,    // Input area
};

#[allow(non_snake_case)]
pub struct LightTokens {
    // ── Backgrounds ──────────────────────────────────────────────────────
    /// Fond principal blanc.
    pub BG_PRIMARY: Color,
    /// Fond sidebar gris très clair.
    pub BG_SECONDARY: Color,
    /// Fond cartes blanc.
    pub BG_CARD: Color,
    /// Fond input blanc.
    pub BG_INPUT: Color,
    /// Fond hover gris clair.
    pub BG_HOVER: Color,
    /// Fond sélection bleu très clair.
    pub BG_SELECTED: Color,

    // ── Text ─────────────────────────────────────────────────────────────
    /// Texte principal noir.
    pub TEXT_PRIMARY: Color,
    /// Texte secondaire gris.
    pub TEXT_SECONDARY: Color,
    /// Texte atténué gris clair.
    pub TEXT_MUTED: Color,
    /// Texte sur accent blanc.
    pub TEXT_ON_ACCENT: Color,

    // ── Accent colors ────────────────────────────────────────────────────
    /// Bleu principal.
    pub ACCENT_BLUE: Color,
    /// Hover bleu.
    pub ACCENT_HOVER: Color,
    /// Bleu sombre.
    pub ACCENT_DARK: Color,
    /// Vert succès.
    pub ACCENT_GREEN: Color,
    /// Rouge erreur.
    pub ACCENT_RED: Color,
    /// Orange warning.
    pub ACCENT_ORANGE: Color,

    // ── Borders ──────────────────────────────────────────────────────────
    /// Bordure principale grise.
    pub BORDER_DEFAULT: Color,
    /// Bordure hover.
    pub BORDER_HOVER: Color,
    /// Bordure sélection bleue.
    pub BORDER_SELECTED: Color,
    /// Bordure subtile.
    pub BORDER_SUBTLE: Color,

    // ── Shadows ──────────────────────────────────────────────────────────
    /// Ombre subtile.
    pub SHADOW_SUBTLE: Color,
    /// Ombre moyenne.
    pub SHADOW_MEDIUM: Color,
    /// Ombre forte.
    pub SHADOW_STRONG: Color,

    // ── Spacing & Radius ─────────────────────────────────────────────────
    /// Rayon petit.
    pub RADIUS_SMALL: f32,
    /// Rayon moyen.
    pub RADIUS_MEDIUM: f32,
    /// Rayon large.
    pub RADIUS_LARGE: f32,
    /// Rayon input.
    pub RADIUS_INPUT: f32,
}
