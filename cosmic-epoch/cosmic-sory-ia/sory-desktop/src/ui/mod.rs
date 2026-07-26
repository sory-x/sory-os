// SPDX-License-Identifier: GPL-3.0-only

use cosmic::{Element, widget};

use crate::{
    components::main_shell,
    events::AppEvent,
    pages,
    state::{ActivePage, AppState},
};

/// Point d'entrée unique de composition de l'interface.
pub fn view(state: &AppState) -> Element<'_, AppEvent> {
    let show_workspace = matches!(state.sidebar.active_page, ActivePage::Chat);

    let center: Element<'_, AppEvent> = match state.sidebar.active_page {
        ActivePage::Chat => pages::chat::view(state),
        ActivePage::History => main_shell::scrollable_page(pages::history::view()),
        ActivePage::Favorites => main_shell::scrollable_page(pages::history::view()),
        ActivePage::Settings => main_shell::scrollable_page(pages::settings::view(state)),
        ActivePage::Workspace => main_shell::scrollable_page(pages::workspace::view(state)),
        ActivePage::About => main_shell::scrollable_page(pages::about::view()),
        ActivePage::ModelPicker => {
            main_shell::scrollable_page(pages::model_picker::view(state))
        }
        ActivePage::ProviderPicker => {
            main_shell::scrollable_page(pages::provider_picker::view(state))
        }
    };

    let layout = main_shell::view(state, center, show_workspace);

    widget::container(layout)
        .width(cosmic::iced::Length::Fill)
        .height(cosmic::iced::Length::Fill)
        .class(cosmic::theme::sory::bg_deep())
        .into()
}
