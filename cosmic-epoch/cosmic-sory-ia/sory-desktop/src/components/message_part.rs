use cosmic::{
    Element,
    widget::{self, column, container},
};

use crate::{
    components::{basic_tool, collapsible, markdown_view, thinking_indicator},
    events::AppEvent,
    models::{Message, MessageRole, MessageStatus, ToolStatus},
    state::AppState,
    theme::tokens,
};

pub fn render_parts<'a>(
    message: &'a Message,
    state: &'a AppState,
) -> Vec<Element<'a, AppEvent>> {
    let mut parts: Vec<Element<'a, AppEvent>> = Vec::new();

    match message.role {
        MessageRole::User => {
            if !message.content.is_empty() {
                parts.push(
                    container(
                        widget::text(&message.content)
                            .size(f32::from(tokens::FONT_LG))
                            .font(cosmic::font::default()),
                    )
                    .width(cosmic::iced::Length::Fixed(560.0))
                    .padding(tokens::BUBBLE_PADDING)
                    .class(cosmic::theme::sory::dialog_panel())
                    .into(),
                );
            }
        }
        MessageRole::Assistant => {
            if !message.content.is_empty() {
                parts.push(markdown_view::view(&message.content));
            }

            for tool in &message.tool_calls {
                let is_expanded = state.is_collapsible_expanded(
                    &tool.id.to_string(),
                ) || tool.status == ToolStatus::Started;

                let tool_id = tool.id.to_string();
                let header = basic_tool::header(tool);
                let content = column(Vec::new()).spacing(tokens::SPACE_SM);

                let expanded = collapsible::view(
                    header,
                    is_expanded,
                    AppEvent::ToggleCollapsible(tool_id),
                    content,
                );

                parts.push(
                    container(expanded)
                        .width(cosmic::iced::Length::Fill)
                        .padding([tokens::SPACE_XS, 0])
                        .into(),
                );
            }

            if message.status == MessageStatus::Streaming
                && message.content.is_empty()
                && message.tool_calls.is_empty()
            {
                parts.push(thinking_indicator::view());
            }
        }
        MessageRole::System => {
            if !message.content.is_empty() {
                parts.push(
                    widget::text(&message.content)
                        .size(f32::from(tokens::FONT_SM))
                        .font(cosmic::font::default())
                        .into(),
                );
            }
        }
        MessageRole::Tool => {
            if !message.content.is_empty() {
                parts.push(
                    container(
                        widget::text(&message.content)
                            .size(f32::from(tokens::FONT_SM))
                            .font(cosmic::font::default()),
                    )
                    .width(cosmic::iced::Length::Fill)
                    .padding(tokens::SPACE_SM)
                    .class(cosmic::theme::sory::context_content())
                    .into(),
                );
            }
        }
    }

    parts
}
