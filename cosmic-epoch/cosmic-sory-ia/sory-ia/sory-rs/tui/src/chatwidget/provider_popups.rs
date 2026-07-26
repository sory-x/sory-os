use super::*;
use sory_model_provider_info::built_in_model_providers;

impl ChatWidget {
    pub(crate) fn open_provider_popup(&mut self) {
        if !self.is_session_configured() {
            self.add_info_message(
                "Provider selection is disabled until startup completes.".to_string(),
                None,
            );
            return;
        }

        let providers = built_in_model_providers(None);
        let current_provider_id = &self.config.model_provider_id;

        let mut items: Vec<SelectionItem> = providers
            .into_iter()
            .map(|(id, info)| {
                let provider_id = id.clone();
                let is_current = id == *current_provider_id;
                let description = info
                    .base_url
                    .clone()
                    .or_else(|| info.env_key.clone().map(|k| format!("env: {k}")));
                let actions: Vec<SelectionAction> = vec![Box::new(move |tx| {
                    tx.send(AppEvent::PersistProviderSelection {
                        provider_id: provider_id.clone(),
                    });
                })];
                SelectionItem {
                    name: format!("{} ({})", info.name, id),
                    description,
                    is_current,
                    actions,
                    dismiss_on_select: true,
                    ..Default::default()
                }
            })
            .collect();

        items.sort_by(|a, b| b.is_current.cmp(&a.is_current).then(a.name.cmp(&b.name)));

        let header = Self::provider_menu_header();
        self.bottom_pane.show_selection_view(SelectionViewParams {
            footer_hint: Some(standard_popup_hint_line()),
            items,
            header,
            ..Default::default()
        });
    }

    fn provider_menu_header() -> Box<dyn Renderable> {
        let mut header = ColumnRenderable::new();
        header.push(Line::from("Select AI Provider".bold()));
        header.push(Line::from("Choose which AI provider to use for new threads.".dim()));
        Box::new(header)
    }
}
