// SPDX-License-Identifier: GPL-3.0-only

//! Tokens visuels natifs pour Sory IA Desktop.
//!
//! L'application est Rust/libcosmic : on ne charge pas de CSS web. Ces valeurs
//! jouent le rôle d'un design system léger côté composants natifs.
//!
//! Pour les couleurs, gradients et styles de containers, utiliser directement
//! `cosmic::palette::SORY` et `cosmic::theme::sory::*`.

// ═════════════════════════════════════════════════════════════════════════════
// SPACING (espacement)
// ═════════════════════════════════════════════════════════════════════════════

/// Espacement extra-extra-small (2px).
pub const SPACE_XXS: u16 = 2;
/// Espacement extra-small (4px).
pub const SPACE_XS: u16 = 4;
/// Espacement small (8px).
pub const SPACE_SM: u16 = 8;
/// Espacement medium (16px).
pub const SPACE_MD: u16 = 16;
/// Espacement large (20px).
pub const SPACE_LG: u16 = 20;
/// Espacement extra-large (28px).
pub const SPACE_XL: u16 = 28;
/// Espacement double extra-large (36px).
pub const SPACE_XXL: u16 = 36;
/// Espacement triple extra-large (52px).
pub const SPACE_XXXL: u16 = 52;

// ═════════════════════════════════════════════════════════════════════════════
// DIMENSIONS (largeurs / hauteurs)
// ═════════════════════════════════════════════════════════════════════════════

/// Largeur de la sidebar gauche (navigation + conversations).
pub const SIDEBAR_WIDTH: u16 = 280;
/// Largeur de la sidebar gauche réduite (icônes seules).
pub const SIDEBAR_COLLAPSED_WIDTH: u16 = 64;
/// Largeur du panneau droit (workspace).
pub const RIGHT_SIDEBAR_WIDTH: u16 = 300;
/// Largeur du panneau workspace réduit.
pub const RIGHT_SIDEBAR_COLLAPSED_WIDTH: u16 = 0;
/// Durée d'une frame d'animation de layout (ms).
pub const LAYOUT_ANIMATION_STEP_MS: u64 = 16;
/// Vitesse d'interpolation sidebar (0.0–1.0 par frame).
pub const LAYOUT_ANIMATION_SPEED: f32 = 0.12;
/// Largeur maximale de la zone de conversation.
pub const CHAT_MAX_WIDTH: u16 = 860;
/// Alias de compatibilité (ancien nom).
pub const CHAT_WIDTH: u16 = CHAT_MAX_WIDTH;
/// Largeur de la zone de saisie.
pub const INPUT_MAX_WIDTH: u16 = 860;
/// Hauteur du header de conversation.
pub const HEADER_HEIGHT: u16 = 60;
/// Hauteur de la barre de titre personnalisée.
pub const TITLE_BAR_HEIGHT: u16 = 56;
/// Hauteur minimale de la zone de saisie.
pub const INPUT_MIN_HEIGHT: u16 = 52;
/// Hauteur maximale de la zone de saisie (multiline).
pub const INPUT_MAX_HEIGHT: u16 = 200;
/// Taille des avatars dans les messages.
pub const AVATAR_SIZE: u16 = 32;
/// Taille des icônes de navigation.
pub const NAV_ICON_SIZE: u16 = 18;
/// Taille des icônes de bouton.
pub const BUTTON_ICON_SIZE: u16 = 16;

// ═════════════════════════════════════════════════════════════════════════════
// PADDING (rembourrage interne)
// ═════════════════════════════════════════════════════════════════════════════

/// Padding des bulles de message.
pub const BUBBLE_PADDING: u16 = 20;
/// Padding des cartes.
pub const CARD_PADDING: u16 = 16;
/// Padding des panneaux (sidebar, workspace).
pub const PANEL_PADDING: u16 = 20;
/// Padding de la zone de saisie.
pub const INPUT_PADDING: u16 = 16;
/// Padding des items de navigation.
pub const NAV_ITEM_PADDING: u16 = 12;
/// Padding des boutons d'action.
pub const ACTION_BUTTON_PADDING: u16 = 10;

// ═════════════════════════════════════════════════════════════════════════════
// FONT SIZES (tailles de texte)
// ═════════════════════════════════════════════════════════════════════════════

/// Texte extra-small (11px) — badges, compteurs.
pub const FONT_XS: f32 = 11.0;
/// Texte small (12px) — hints, métadonnées, timestamps.
pub const FONT_SM: f32 = 12.0;
/// Texte normal (14px) — contenu secondaire, labels.
pub const FONT_MD: f32 = 14.0;
/// Texte large (15px) — contenu principal des messages.
pub const FONT_LG: f32 = 15.0;
/// Texte extra-large (16px) — titres de section.
pub const FONT_XL: f32 = 16.0;
/// Texte double extra-large (20px) — titres de page.
pub const FONT_XXL: f32 = 20.0;
/// Texte triple extra-large (24px) — titre de l'application.
pub const FONT_XXXL: f32 = 24.0;
/// Texte titres markdown H1 (22px).
pub const FONT_MD_H1: f32 = 22.0;
/// Texte titres markdown H2 (18px).
pub const FONT_MD_H2: f32 = 18.0;
/// Texte titres markdown H3 (15px, gras).
pub const FONT_MD_H3: f32 = 15.0;
/// Texte de code (13px, monospace).
pub const FONT_CODE: f32 = 13.0;

// ═════════════════════════════════════════════════════════════════════════════
// BORDER RADIUS (rayon de bordure)
// ═════════════════════════════════════════════════════════════════════════════

/// Rayon pour les éléments pills (boutons, badges).
pub const RADIUS_PILL: f32 = 999.0;
/// Rayon pour les cartes.
pub const RADIUS_CARD: f32 = 16.0;
/// Rayon pour les dialogues/modales.
pub const RADIUS_DIALOG: f32 = 16.0;
/// Rayon pour les boutons.
pub const RADIUS_BUTTON: f32 = 10.0;
/// Rayon pour les sections.
pub const RADIUS_SECTION: f32 = 8.0;
/// Rayon pour les blocs de code.
pub const RADIUS_CODE_BLOCK: f32 = 8.0;
/// Rayon pour les items de liste.
pub const RADIUS_LIST_ITEM: f32 = 6.0;
/// Rayon pour les avatars (cercle).
pub const RADIUS_CIRCLE: f32 = 999.0;
/// Rayon pour les inputs.
pub const RADIUS_INPUT: f32 = 10.0;

// ═════════════════════════════════════════════════════════════════════════════
// SHADOWS / BLUR (ombres et effets)
// ═════════════════════════════════════════════════════════════════════════════

/// Blur pour les effets glow subtils (cartes au survol).
pub const GLOW_BLUR_SUBTLE: f32 = 8.0;
/// Blur pour les effets glow moyens (cartes sélectionnées).
pub const GLOW_BLUR_MEDIUM: f32 = 16.0;
/// Blur pour les effets glow forts (éléments actifs).
pub const GLOW_BLUR_STRONG: f32 = 24.0;
/// Blur pour les ombres portées.
pub const SHADOW_BLUR: f32 = 12.0;
/// Offset Y des ombres portées.
pub const SHADOW_OFFSET_Y: f32 = 2.0;

// ═════════════════════════════════════════════════════════════════════════════
// OPACITY (transparence)
// ═════════════════════════════════════════════════════════════════════════════

/// Opacity pour les éléments désactivés.
pub const OPACITY_DISABLED: f32 = 0.4;
/// Opacity pour les éléments secondaires.
pub const OPACITY_SECONDARY: f32 = 0.7;
/// Opacity pour les overlays.
pub const OPACITY_OVERLAY: f32 = 0.6;

// ═════════════════════════════════════════════════════════════════════════════
// STRING CONSTANTS (chaînes de l'interface)
// ═════════════════════════════════════════════════════════════════════════════

/// Nom de l'application.
pub const APP_TITLE: &str = "Sory IA";
/// Nom de l'assistant dans les bulles.
pub const ASSISTANT_NAME: &str = "Sory IA";
/// Nom de l'utilisateur dans les bulles.
pub const USER_NAME: &str = "Vous";
/// Placeholder de la zone de saisie.
pub const INPUT_PLACEHOLDER: &str = "Demandez n'importe quoi \u{00e0} Sory IA\u{2026}";
/// Label du bouton nouvelle conversation.
pub const NEW_CONVERSATION_LABEL: &str = "Nouvelle conversation";
/// Raccourci clavier nouvelle conversation.
pub const NEW_CONVERSATION_SHORTCUT: &str = "Ctrl K";
/// Label section conversations récentes.
pub const RECENT_CONVERSATIONS_LABEL: &str = "Conversations r\u{00e9}centes";
/// Label "Voir toutes les conversations".
pub const SEE_ALL_LABEL: &str = "Voir toutes";
/// Label du workspace par défaut.
pub const DEFAULT_WORKSPACE: &str = "Aucun workspace";
/// Label "Connecté" dans le header.
pub const STATUS_CONNECTED: &str = "Connect\u{00e9}";
/// Label "En ligne" dans le profil.
pub const STATUS_ONLINE: &str = "En ligne";
/// Label "Workspace actuel" dans le sélecteur.
pub const WORKSPACE_CURRENT: &str = "Workspace actuel";
/// Label "Raisonnement" dans le toggle.
pub const REASONING_LABEL: &str = "Raisonnement";
/// Disclaimer en bas de l'input.
pub const DISCLAIMER: &str =
    "Sory IA peut faire des erreurs. V\u{00e9}rifiez les informations importantes.";
/// Texte "Demandez n'importe quoi...".
pub const ASK_ANYTHING: &str = "Demandez n'importe quoi \u{00e0} Sory IA\u{2026}";

// ═════════════════════════════════════════════════════════════════════════════
// ICONS (icônes — noms de thème COSMIC ou Unicode)
// ═════════════════════════════════════════════════════════════════════════════

/// Icône de l'application (emoji fallback).
pub const ICON_APP: &str = "\u{2728}";
/// Icône Accueil.
pub const ICON_HOME: &str = "\u{2302}";
/// Icône Historique.
pub const ICON_HISTORY: &str = "\u{25f7}";
/// Icône Favoris.
pub const ICON_FAVORITES: &str = "\u{2605}";
/// Icône Workspace.
pub const ICON_WORKSPACE: &str = "\u{25a4}";
/// Icône Agents.
pub const ICON_AGENTS: &str = "\u{2b23}";
/// Icône Outils.
pub const ICON_TOOLS: &str = "\u{2699}";
/// Icône Paramètres.
pub const ICON_SETTINGS: &str = "\u{2699}";
/// Icône À propos.
pub const ICON_ABOUT: &str = "\u{2139}";
/// Icône utilisateur (bulle).
pub const ICON_USER: &str = "\u{25cf}";
/// Icône assistant (bulle).
pub const ICON_ASSISTANT: &str = "\u{2726}";
/// Icône nouveau message (+).
pub const ICON_NEW: &str = "+";
/// Icône pièce jointe.
pub const ICON_ATTACHMENT: &str = "\u{1f4ce}";
/// Icône mention @.
pub const ICON_MENTION: &str = "@";
/// Icône recherche web.
pub const ICON_WEB: &str = "\u{1f310}";
/// Icône code.
pub const ICON_CODE: &str = ">_";
/// Icône envoyer (flèche).
pub const ICON_SEND: &str = "\u{25b6}";
/// Icône arrêter.
pub const ICON_STOP: &str = "\u{25a0}";
/// Icône copier.
pub const ICON_COPY: &str = "\u{2398}";
/// Icône like.
pub const ICON_LIKE: &str = "\u{1f44d}";
/// Icône dislike.
pub const ICON_DISLIKE: &str = "\u{1f44e}";
/// Icône audio.
pub const ICON_AUDIO: &str = "\u{1f50a}";
/// Icône menu (trois points).
pub const ICON_MENU: &str = "\u{22ef}";
/// Icône vérifié.
pub const ICON_CHECK: &str = "\u{2713}";
/// Icône en cours (spinner).
pub const ICON_SPINNER: &str = "\u{21bb}";
/// Icône erreur.
pub const ICON_ERROR: &str = "\u{26a0}";
/// Icône expand/taille réelle.
pub const ICON_EXPAND: &str = "\u{26f6}";
/// Icône réduire.
pub const ICON_COLLAPSE: &str = "\u{2716}";
/// Icône fermer.
pub const ICON_CLOSE: &str = "\u{2715}";
/// Icône menu hamburger.
pub const ICON_MENU_TOGGLE: &str = "\u{2630}";
/// Icône régénérer.
pub const ICON_REGENERATE: &str = "\u{21bb}";
/// Icône partager.
pub const ICON_SHARE: &str = "\u{2197}";
/// Icône modifier.
pub const ICON_EDIT: &str = "\u{270e}";
/// Icône recherche.
pub const ICON_SEARCH: &str = "\u{1f50d}";
/// Icône dossier.
pub const ICON_FOLDER: &str = "\u{1f4c1}";
/// Icône raisonnement (cerveau).
pub const ICON_REASONING: &str = "\u{1f9e0}";
/// Icône paramètres rapides.
pub const ICON_SLIDERS: &str = "\u{2699}";
