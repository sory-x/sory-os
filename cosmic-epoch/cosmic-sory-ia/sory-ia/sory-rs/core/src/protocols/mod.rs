use sory_api::ApiError;
use sory_protocol::openai_models::ModelInfo;
use serde_json::Value;
use sory_client::ByteStream;
use sory_model_provider_info::WireApi;

use crate::client_common::Prompt;
use crate::client_common::ResponseStream;

mod openai_chat;

pub(crate) enum Protocol {
    OpenAIChat,
}

impl Protocol {
    pub(crate) fn from_wire_api(wire_api: WireApi) -> Result<Self, ApiError> {
        match wire_api {
            WireApi::ChatCompletions => Ok(Protocol::OpenAIChat),
            WireApi::Responses => Err(ApiError::Stream(
                "Responses API should use the existing stream_responses_api path, not the protocol router".into(),
            )),
        }
    }

    pub(crate) fn endpoint_path(&self) -> &str {
        match self {
            Protocol::OpenAIChat => openai_chat::ENDPOINT_PATH,
        }
    }

    pub(crate) fn build_request(
        &self,
        prompt: &Prompt,
        model_info: &ModelInfo,
        model: &str,
    ) -> Result<Value, ApiError> {
        match self {
            Protocol::OpenAIChat => openai_chat::build_request(prompt, model_info, model),
        }
    }

    pub(crate) fn spawn_stream(&self, byte_stream: ByteStream) -> ResponseStream {
        match self {
            Protocol::OpenAIChat => openai_chat::spawn_stream(byte_stream),
        }
    }
}