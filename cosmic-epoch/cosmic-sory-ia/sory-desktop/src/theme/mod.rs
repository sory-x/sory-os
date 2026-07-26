// SPDX-License-Identifier: GPL-3.0-only

//! Thème Sory IA — Intégration du design system SoryOS Deep Navy Glass.
//!
//! Le thème encapsule le thème COSMIC sous-jacent et applique les styles
//! SoryOS via `cosmic::theme::sory::*` et `cosmic::palette::SORY`.
//!
//! Supporte maintenant deux modes :
//! - Dark : Design system SoryOS Deep Navy Glass (par défaut)
//! - Light : Interface style ChatGPT (fond clair épuré)

pub mod tokens;

use cosmic::theme;

/// Modes de thème supportés par Sory IA.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SoryThemeMode {
    #[default]
    Dark,
    #[allow(dead_code)]
    Light,
}

/// Thème Sory IA encapsulant le thème COSMIC sous-jacent.
///
/// Applique le design system SoryOS Deep Navy Glass :
/// - Fonds sombres profonds (#030712 / #05091b)
/// - Accents bleus lumineux (#2b6de8 / #4a8aff)
/// - Effets de verre et glow
/// - Bordures lumineuses avec alpha
#[derive(Debug, Clone)]
pub struct SoryTheme {
    pub mode: SoryThemeMode,
}

impl Default for SoryTheme {
    fn default() -> Self {
        Self {
            mode: SoryThemeMode::Dark,
        }
    }
}

impl SoryTheme {
    /// Retourne le thème COSMIC correspondant au mode choisi.
    ///
    /// En mode sombre, utilise `theme::Theme::dark()` qui sera ensuite
    /// surchargé par les styles `cosmic::theme::sory::*` dans chaque composant.
    pub fn cosmic_theme(&self) -> theme::Theme {
        match self.mode {
            SoryThemeMode::Dark => theme::Theme::dark(),
            SoryThemeMode::Light => theme::Theme::light(),
        }
    }

    /// Retourne true si le mode sombre est actif.
    #[allow(dead_code)]
    pub fn is_dark(&self) -> bool {
        matches!(self.mode, SoryThemeMode::Dark)
    }
}
