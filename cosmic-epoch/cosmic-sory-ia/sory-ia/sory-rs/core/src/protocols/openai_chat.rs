use sory_api::ApiError;
use sory_api::ResponseEvent;
use sory_protocol::error::Result as SoryResult;
use sory_protocol::models::ContentItem;
use sory_protocol::models::ResponseItem;
use sory_protocol::openai_models::ModelInfo;
use eventsource_stream::Eventsource;
use futures::StreamExt;
use serde_json::Value;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use tracing::debug;

use crate::client_common::Prompt;
use crate::client_common::ResponseStream;

pub(crate) const ENDPOINT_PATH: &str = "/chat/completions";

#[derive(serde::Serialize)]
struct ChatMessage {
    role: String,
    content: Vec<ChatContentPart>,
}

#[derive(serde::Serialize)]
#[serde(untagged)]
enum ChatContentPart {
    Text { r#type: String, text: String },
}

#[derive(serde::Deserialize, Debug)]
struct ChunkChoice {
    #[serde(default)]
    delta: ChunkDelta,
    #[serde(default)]
    finish_reason: Option<String>,
}

#[derive(serde::Deserialize, Debug, Default)]
struct ChunkDelta {
    #[serde(default)]
    role: Option<String>,
    #[serde(default)]
    content: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
struct ChatCompletionsChunk {
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    choices: Vec<ChunkChoice>,
}

fn convert_response_items_to_messages(input: &[ResponseItem]) -> Result<Vec<Value>, ApiError> {
    let mut messages = Vec::new();
    for item in input {
        match item {
            ResponseItem::Message { role, content, .. } => {
                let parts: Vec<ChatContentPart> = content
                    .iter()
                    .filter_map(|c| match c {
                        ContentItem::InputText { text } => Some(ChatContentPart::Text {
                            r#type: "text".to_string(),
                            text: text.clone(),
                        }),
                        ContentItem::OutputText { text } => Some(ChatContentPart::Text {
                            r#type: "text".to_string(),
                            text: text.clone(),
                        }),
                        _ => None,
                    })
                    .collect();
                if !parts.is_empty() {
                    messages.push(serde_json::to_value(ChatMessage {
                        role: role.clone(),
                        content: parts,
                    }).map_err(|e| ApiError::Stream(format!("failed to serialize message: {e}")))?);
                }
            }
            ResponseItem::FunctionCallOutput { call_id, output, .. } => {
                let content = output.text_content().unwrap_or_default();
                messages.push(serde_json::json!({
                    "role": "tool",
                    "tool_call_id": call_id,
                    "content": content,
                }));
            }
            ResponseItem::FunctionCall { name, arguments, call_id, .. } => {
                messages.push(serde_json::json!({
                    "role": "assistant",
                    "tool_calls": [{
                        "id": call_id,
                        "type": "function",
                        "function": {
                            "name": name,
                            "arguments": arguments,
                        }
                    }],
                    "content": null,
                }));
            }
            _ => {
                debug!("skipping unsupported response item in chat completions: {item:?}");
            }
        }
    }
    Ok(messages)
}

pub(crate) fn build_request(
    prompt: &Prompt,
    _model_info: &ModelInfo,
    model: &str,
) -> Result<Value, ApiError> {
    let input = prompt.get_formatted_input();
    let instructions = &prompt.base_instructions.text;
    let messages = convert_response_items_to_messages(&input)?;

    let mut body = serde_json::json!({
        "model": model,
        "messages": messages,
        "stream": true,
    });

    if !instructions.is_empty() {
        if let Some(arr) = body["messages"].as_array_mut() {
            if arr.is_empty() || arr[0]["role"] != "system" {
                arr.insert(0, serde_json::json!({
                    "role": "system",
                    "content": instructions,
                }));
            }
        }
    }

    Ok(body)
}

pub(crate) fn spawn_stream(byte_stream: sory_client::ByteStream) -> ResponseStream {
    let (tx_event, rx_event) = mpsc::channel::<SoryResult<ResponseEvent>>(1600);
    let consumer_dropped = CancellationToken::new();
    let consumer_dropped_for_stream = consumer_dropped.clone();

    tokio::spawn(async move {
        tokio::select! {
            _ = consumer_dropped.cancelled() => {}
            _ = process_stream(byte_stream, tx_event) => {}
        }
    });

    ResponseStream {
        rx_event,
        consumer_dropped: consumer_dropped_for_stream,
    }
}

async fn process_stream(
    byte_stream: sory_client::ByteStream,
    tx_event: mpsc::Sender<SoryResult<ResponseEvent>>,
) {
    const IDLE_TIMEOUT: Duration = Duration::from_secs(60);

    let _ = tx_event.send(Ok(ResponseEvent::Created)).await;

    let mut event_stream = byte_stream.eventsource();
    let mut full_content = String::new();
    let mut has_content = false;
    let deadline = tokio::time::Instant::now() + IDLE_TIMEOUT;

    loop {
        if tokio::time::Instant::now() > deadline {
            break;
        }
        let next = tokio::time::timeout(Duration::from_secs(5), event_stream.next());
        match next.await {
            Ok(Some(Ok(event))) => {
                let data = event.data.trim().to_string();
                if data == "[DONE]" || data.is_empty() {
                    if data == "[DONE]" {
                        break;
                    }
                    continue;
                }

                match serde_json::from_str::<ChatCompletionsChunk>(&data) {
                    Ok(chunk) => {
                        for choice in &chunk.choices {
                            if let Some(ref delta_content) = choice.delta.content {
                                if !delta_content.is_empty() {
                                    full_content.push_str(delta_content);
                                    has_content = true;
                                    if tx_event
                                        .send(Ok(ResponseEvent::OutputTextDelta(
                                            delta_content.clone(),
                                        )))
                                        .await
                                        .is_err()
                                    {
                                        return;
                                    }
                                }
                            }
                            if let Some(finish_reason) = &choice.finish_reason {
                                if !finish_reason.is_empty() && finish_reason != "null" {
                                    let response_id = chunk.id.clone().unwrap_or_default();
                                    if has_content {
                                        let _ = tx_event
                                            .send(Ok(ResponseEvent::OutputItemDone(
                                                ResponseItem::Message {
                                                    id: Some(response_id.clone()),
                                                    role: "assistant".to_string(),
                                                    content: vec![ContentItem::OutputText {
                                                        text: full_content.clone(),
                                                    }],
                                                    phase: None,
                                                },
                                            )))
                                            .await;
                                    }
                                    let _ = tx_event
                                        .send(Ok(ResponseEvent::Completed {
                                            response_id,
                                            token_usage: None,
                                            end_turn: Some(true),
                                        }))
                                        .await;
                                    return;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        debug!("failed to parse chat completions chunk: {e}, data: {data}");
                    }
                }
            }
            Ok(Some(Err(e))) => {
                debug!("chat completions stream error: {e}");
                break;
            }
            Ok(None) => break,
            Err(_) => break,
        }
    }

    let _ = tx_event
        .send(Ok(ResponseEvent::Completed {
            response_id: String::new(),
            token_usage: None,
            end_turn: Some(true),
        }))
        .await;
}