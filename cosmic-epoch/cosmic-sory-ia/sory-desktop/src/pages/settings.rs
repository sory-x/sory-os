// SPDX-License-Identifier: GPL-3.0-only

//! Page de paramètres de Sory IA.
//!
//! Affiche la liste des fournisseurs IA disponibles, leur configuration
//! (endpoint, clé API, modèle), et les paramètres généraux.
//! Les changements sont sauvegardés automatiquement.

use std::borrow::Cow;

use cosmic::widget::{self, button, column, container, row, text_input, toggler};
use cosmic::{Element, iced::Length};

use crate::{
    components::page_header,
    events::AppEvent,
    models::{catalog, known_providers, ProviderDefinition},
    state::AppState,
    theme::tokens,
};

/// Largeur fixe pour les labels des champs de formulaire.
const LABEL_WIDTH: f32 = 120.0;

/// Hauteur maximale du sélecteur de provider avant scroll.
const PROVIDER_LIST_MAX_HEIGHT: u16 = 300;

// ── helpers ────────────────────────────────────────────────────────────────

/// Raccourci pour une ligne label + input.
/// Le placeholder accepte un `Cow<'a, str>` pour gérer les chaînes
/// possédées ou empruntées avec la bonne durée de vie.
fn labeled_input<'a>(
    label: &'a str,
    placeholder: Cow<'a, str>,
    value: &'a str,
    on_input: impl Fn(String) -> AppEvent + 'a,
) -> Element<'a, AppEvent> {
    row(Vec::new())
        .spacing(tokens::SPACE_SM)
        .push(container(widget::text(label).size(14)).width(Length::Fixed(LABEL_WIDTH)))
        .push(
            text_input(placeholder, value)
                .on_input(on_input)
                .width(Length::Fill),
        )
        .into()
}

/// Raccourci pour une ligne label + texte statique.
fn labeled_text<'a>(label: &'a str, value: impl Into<Cow<'a, str>> + 'a) -> Element<'a, AppEvent> {
    row(Vec::new())
        .spacing(tokens::SPACE_SM)
        .push(container(widget::text(label).size(14)).width(Length::Fixed(LABEL_WIDTH)))
        .push(widget::text(value).size(14))
        .into()
}

/// Ligne on/off pour un booléen.
fn labeled_toggle<'a>(
    label: &'a str,
    is_on: bool,
    on_toggle: impl Fn(bool) -> AppEvent + 'a,
) -> Element<'a, AppEvent> {
    row(Vec::new())
        .spacing(tokens::SPACE_SM)
        .push(container(widget::text(label).size(14)).width(Length::Fixed(LABEL_WIDTH)))
        .push(toggler(is_on).on_toggle(on_toggle))
        .into()
}

/// Affiche le sélecteur de fournisseur avec catégories.
/// Le modèle sélectionné est affiché directement sous le provider actif
/// sous forme de dropdown, pour que l'utilisateur choisisse son modèle
/// immédiatement sans scroller vers une autre section.
fn provider_selector<'a>(state: &'a AppState) -> Element<'a, AppEvent> {
    let active_id = &state.settings.settings.provider_id;

    // Catégories visuelles pour grouper les providers
    let categories: &[(&str, &[&str])] = &[
        ("Premium", &["openai", "anthropic", "google", "mistral"]),
        (
            "Open Source / Rapide",
            &[
                "groq", "deepinfra", "togetherai", "fireworks", "deepseek", "cerebras",
            ],
        ),
        ("Agrégateurs", &["openrouter"]),
        (
            "Cloud",
            &["alibaba", "azure", "bedrock", "cloudflare", "nvidia"],
        ),
        ("Recherche", &["xai", "perplexity", "cohere"]),
        (
            "Plateformes Dev",
            &["github-copilot", "huggingface", "replicate", "soryos-zen", "opencode-go"],
        ),
        ("Autres", &["venice", "kilo", "sap-ai-core", "gateway", "sorycode"]),
    ];

    let all_providers = known_providers();

    let mut col = column(Vec::new()).spacing(4);

    for (cat_name, cat_ids) in categories {
        // Trouver les providers de cette catégorie
        let cat_providers: Vec<&ProviderDefinition> = all_providers
            .iter()
            .filter(|p| cat_ids.contains(&p.id.as_str()))
            .collect();

        if cat_providers.is_empty() {
            continue;
        }

        // En-tête de catégorie
        col = col.push(
            container(widget::text(*cat_name).size(11))
                .padding([tokens::SPACE_XS, 0, tokens::SPACE_XXS, 0])
                .width(Length::Fill),
        );

        // Boutons providers
        if *cat_name == "Autres" && cat_providers.len() <= 4 {
            let mut row_btns = row(Vec::new()).spacing(tokens::SPACE_XS);
            for def in &cat_providers {
                let is_active = def.id == *active_id;
                let pid = def.id.clone();
                let label = def.name.clone();
                let btn_class = if is_active {
                    cosmic::theme::Button::Suggested
                } else {
                    cosmic::theme::Button::Standard
                };
                row_btns = row_btns.push(
                    button::text(label)
                        .on_press(AppEvent::ProviderChanged(pid))
                        .padding([tokens::SPACE_XS, tokens::SPACE_SM])
                        .class(btn_class),
                );
            }
            col = col.push(row_btns);
            if let Some(active_def) = cat_providers.iter().find(|d| d.id == *active_id) {
                col = col.push(inline_model_dropdown(state, active_def));
            }
        } else {
            for def in cat_providers {
                let is_active = def.id == *active_id;
                let pid = def.id.clone();
                let label = def.name.clone();
                let btn_class = if is_active {
                    cosmic::theme::Button::Suggested
                } else {
                    cosmic::theme::Button::Standard
                };
                let btn = button::text(if is_active {
                    format!("\u{25cf} {label}")
                } else {
                    label
                })
                .on_press(AppEvent::ProviderChanged(pid))
                .width(Length::Fill)
                .class(btn_class);
                col = col.push(btn);

                if is_active {
                    col = col.push(inline_model_dropdown(state, def));
                }
            }
        }
    }

    container(
        widget::scrollable(col)
            .height(Length::Fixed(PROVIDER_LIST_MAX_HEIGHT.into())),
    )
    .width(Length::Fill)
    .into()
}

/// Dropdown modèle compact affiché sous le provider actif dans la liste.
///
/// Propose **tous les modèles connus** du provider depuis le catalogue
/// `models.dev` (via `catalog::models_for()`). Si l'utilisateur a choisi
/// un modèle custom qui n'est pas dans la liste, il apparaît en tête.
fn inline_model_dropdown<'a>(
    state: &'a AppState,
    _def: &ProviderDefinition,
) -> Element<'a, AppEvent> {
    let provider_id = &state.settings.settings.provider_id;

    // Charger les modèles depuis le catalogue models.dev (1500+ modèles)
    let current_model: &str = state
        .settings
        .settings
        .provider_configs
        .get(provider_id)
        .map(|c| c.model.as_str())
        .filter(|m| !m.is_empty())
        .unwrap_or("auto");

    let models = catalog::resolved_models_for(provider_id, current_model);

    let selected_index = models.iter().position(|m| m == current_model);
    let models_for_closure = models.clone();
    let pid = provider_id.clone();

    // Ligne compacte : label + dropdown, indentée à gauche
    row(Vec::new())
        .spacing(tokens::SPACE_XS)
        .push(
            container(widget::text("Modèle").size(12))
                .width(Length::Fixed(60.0)),
        )
        .push(
            cosmic::widget::dropdown::dropdown(
                models,
                selected_index,
                move |idx| {
                    AppEvent::ProviderModelChanged(
                        pid.clone(),
                        models_for_closure[idx].clone(),
                    )
                },
            )
            .width(Length::Fill),
        )
        .padding([0, 0, 0, tokens::SPACE_SM]) // indentation sous le bouton
        .into()
}

/// Affiche le champ de clé API avec boutons copier / coller / enregistrer.
fn api_key_field<'a>(state: &'a AppState) -> Element<'a, AppEvent> {
    let provider_id = &state.settings.settings.provider_id;
    let cfg = state.settings.settings.provider_configs.get(provider_id);
    let current_key = cfg.map(|c| c.api_key.as_str()).unwrap_or("");
    let pid_input = provider_id.clone();
    let pid_copy = provider_id.clone();
    let pid_paste = provider_id.clone();

    let (api_key_url, api_key_hint) = known_providers()
        .into_iter()
        .find(|p| p.id == *provider_id)
        .map(|p| (p.api_key_url, p.api_key_hint))
        .unwrap_or_default();

    let action_btn = |label: &'static str, event: AppEvent| -> Element<'a, AppEvent> {
        button::text(label)
            .on_press(event)
            .padding([tokens::SPACE_XS, tokens::SPACE_SM])
            .class(cosmic::theme::Button::Text)
            .into()
    };

    let mut field = column(Vec::new()).spacing(tokens::SPACE_SM);

    field = field.push(
        row(Vec::new())
            .spacing(tokens::SPACE_SM)
            .align_y(cosmic::iced::Alignment::Center)
            .push(
                container(widget::text("Cl\u{00e9} API").size(14))
                    .width(Length::Fixed(LABEL_WIDTH)),
            )
            .push(
                text_input("sk-...", current_key)
                    .on_input(move |v| AppEvent::ProviderApiKeyChanged(pid_input.clone(), v))
                    .width(Length::Fill)
                    .padding(tokens::SPACE_SM),
            ),
    );

    field = field.push(
        row(Vec::new())
            .spacing(tokens::SPACE_SM)
            .push(
                container(widget::text("").size(14))
                    .width(Length::Fixed(LABEL_WIDTH)),
            )
            .push(action_btn("\u{2398} Copier", AppEvent::CopyApiKey(pid_copy)))
            .push(action_btn("\u{1f4cb} Coller", AppEvent::PasteApiKey(pid_paste)))
            .push(
                button::suggested("\u{2713} Enregistrer")
                    .on_press(AppEvent::SaveSettings)
                    .padding([tokens::SPACE_XS, tokens::SPACE_MD]),
            ),
    );

    if !api_key_url.is_empty() {
        field = field.push(
            row(Vec::new())
                .spacing(tokens::SPACE_SM)
                .push(
                    container(widget::text("").size(14))
                        .width(Length::Fixed(LABEL_WIDTH)),
                )
                .push(
                    widget::text(format!("\u{1f517} {api_key_hint}"))
                        .size(12),
                ),
        );
    }

    field.into()
}

fn feedback_banner(state: &AppState) -> Element<AppEvent> {
    if let Some(msg) = &state.settings_feedback {
        crate::components::notification::view(msg)
    } else {
        cosmic::widget::Space::new()
            .height(Length::Shrink)
            .into()
    }
}

// ── Vue principale ─────────────────────────────────────────────────────────

pub fn view(state: &AppState) -> Element<AppEvent> {
    let settings = &state.settings.settings;
    let provider_id = &settings.provider_id;

    // Récupérer l'endpoint par défaut du provider actif pour le placeholder
    let (default_endpoint, _default_model_name) = known_providers()
        .into_iter()
        .find(|p| p.id == *provider_id)
        .map(|p| (p.endpoint, p.default_model))
        .unwrap_or_default();

    let cfg = settings.provider_configs.get(provider_id);
    let current_endpoint = cfg.map(|c| c.endpoint.as_str()).unwrap_or("");

    let pid_endpoint = provider_id.clone();

    // Placeholder de l'endpoint (Owned pour la lifetime)
    let endpoint_placeholder: Cow<'_, str> = if default_endpoint.is_empty() {
        Cow::Borrowed("https://api.example.com/v1")
    } else {
        Cow::Owned(default_endpoint)
    };

    column(Vec::new())
        .spacing(tokens::SPACE_MD)
        .width(Length::Fill)
        .push(page_header::view(
            "Param\u{00e8}tres",
            Some("Fournisseurs, mod\u{00e8}les et configuration"),
        ))
        .push(widget::divider::horizontal::default())
        // ── Fournisseur + Modèle (inline dans provider_selector) ──
        .push(widget::text("Fournisseur IA").size(18))
        .push(provider_selector(state))
        .push(feedback_banner(state))
        .push(widget::divider::horizontal::default())
        // ── Configuration du provider actif ──
        .push(widget::text(format!("Configuration : {provider_id}")).size(14))
        .push(
            container(
                column(Vec::new())
                    .spacing(tokens::SPACE_SM)
                    .padding(tokens::SPACE_SM)
                    .push(labeled_input(
                        "Endpoint",
                        endpoint_placeholder,
                        current_endpoint,
                        move |v| AppEvent::ProviderEndpointChanged(pid_endpoint.clone(), v),
                    ))
                    .push(api_key_field(state)),
                // ← Le modèle est maintenant géré dans provider_selector ci-dessus,
                //    plus besoin de le dupliquer ici.
            )
            .width(Length::Fill),
        )
        .push(widget::divider::horizontal::default())
        // ── Paramètres généraux ──
        .push(widget::text("Général").size(18))
        .push(labeled_text(
            "Température",
            format!("{:.1}", settings.temperature),
        ))
        .push(labeled_text("Commande runtime", &settings.runtime_command))
        .push(labeled_text("Langue", &settings.language))
        .push(labeled_toggle(
            "Démarrage auto",
            settings.auto_start_runtime,
            |_| AppEvent::None,
        ))
        .push(widget::divider::horizontal::default())
        // ── Pied de page ──
        .push(labeled_text("Version", env!("CARGO_PKG_VERSION")))
        .into()
}
