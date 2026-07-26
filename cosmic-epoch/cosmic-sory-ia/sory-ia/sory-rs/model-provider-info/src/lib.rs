//! Registry of model providers supported by sory.
//!
//! Providers can be defined in two places:
//!   1. Built-in defaults compiled into the binary so sory works out-of-the-box.
//!   2. User-defined entries inside `~/.sory/config.toml` under the `model_providers`
//!      key. These override or extend the defaults at runtime.

use sory_api::Provider as ApiProvider;
use sory_api::RetryConfig as ApiRetryConfig;
use sory_api::is_azure_responses_provider;
use sory_app_server_protocol::AuthMode;
use sory_protocol::config_types::ModelProviderAuthInfo;
use sory_protocol::error::SoryErr;
use sory_protocol::error::EnvVarError;
use sory_protocol::error::Result as soryResult;
use http::HeaderMap;
use http::header::HeaderName;
use http::header::HeaderValue;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt;
use std::time::Duration;

const DEFAULT_STREAM_IDLE_TIMEOUT_MS: u64 = 300_000;
const DEFAULT_STREAM_MAX_RETRIES: u64 = 5;
const DEFAULT_REQUEST_MAX_RETRIES: u64 = 4;
pub const DEFAULT_WEBSOCKET_CONNECT_TIMEOUT_MS: u64 = 15_000;
/// Hard cap for user-configured `stream_max_retries`.
const MAX_STREAM_MAX_RETRIES: u64 = 100;
/// Hard cap for user-configured `request_max_retries`.
const MAX_REQUEST_MAX_RETRIES: u64 = 100;

const OPENAI_PROVIDER_NAME: &str = "OpenAI";
pub const OPENAI_PROVIDER_ID: &str = "openai";
pub const CHATGPT_SORY_BASE_URL: &str = "https://chatgpt.com/backend-api/sory";
const ANTHROPIC_PROVIDER_NAME: &str = "Anthropic";
pub const ANTHROPIC_PROVIDER_ID: &str = "anthropic";
const MISTRAL_PROVIDER_NAME: &str = "Mistral";
pub const MISTRAL_PROVIDER_ID: &str = "mistral";
const GOOGLE_PROVIDER_NAME: &str = "Google";
pub const GOOGLE_PROVIDER_ID: &str = "google";
const GROQ_PROVIDER_NAME: &str = "Groq";
pub const GROQ_PROVIDER_ID: &str = "groq";
const DEEPSEEK_PROVIDER_NAME: &str = "DeepSeek";
pub const DEEPSEEK_PROVIDER_ID: &str = "deepseek";
const COHERE_PROVIDER_NAME: &str = "Cohere";
pub const COHERE_PROVIDER_ID: &str = "cohere";
const PERPLEXITY_PROVIDER_NAME: &str = "Perplexity";
pub const PERPLEXITY_PROVIDER_ID: &str = "perplexity";
const TOGETHER_PROVIDER_NAME: &str = "Together";
pub const TOGETHER_PROVIDER_ID: &str = "together";
const FIREWORKS_PROVIDER_NAME: &str = "Fireworks";
pub const FIREWORKS_PROVIDER_ID: &str = "fireworks";
const DEEPINFRA_PROVIDER_NAME: &str = "DeepInfra";
pub const DEEPINFRA_PROVIDER_ID: &str = "deepinfra";
const OPENROUTER_PROVIDER_NAME: &str = "OpenRouter";
pub const OPENROUTER_PROVIDER_ID: &str = "openrouter";
const XAI_PROVIDER_NAME: &str = "xAI";
pub const XAI_PROVIDER_ID: &str = "xai";
const NVIDIA_PROVIDER_NAME: &str = "NVIDIA";
pub const NVIDIA_PROVIDER_ID: &str = "nvidia";
const CEREBRAS_PROVIDER_NAME: &str = "Cerebras";
pub const CEREBRAS_PROVIDER_ID: &str = "cerebras";
const GITHUB_COPILOT_PROVIDER_NAME: &str = "GitHub Copilot";
pub const SORYOS_ZEN_PROVIDER_ID: &str = "soryos-zen";
pub const SORYOS_ZEN_PROVIDER_NAME: &str = "SoryOS Zen";
pub const OPENCODE_GO_PROVIDER_ID: &str = "opencode-go";
pub const OPENCODE_GO_PROVIDER_NAME: &str = "OpenCode Go";
pub const GITHUB_COPILOT_PROVIDER_ID: &str = "github-copilot";
const AMAZON_BEDROCK_PROVIDER_NAME: &str = "Amazon Bedrock";
pub const AMAZON_BEDROCK_PROVIDER_ID: &str = "amazon-bedrock";
pub const AMAZON_BEDROCK_GPT_5_4_MODEL_ID: &str = "openai.gpt-5.4";
pub const AMAZON_BEDROCK_DEFAULT_BASE_URL: &str =
    "https://bedrock-mantle.us-east-1.api.aws/openai/v1";
const AMAZON_BEDROCK_MANTLE_CLIENT_AGENT_HEADER: &str = "x-amzn-mantle-client-agent";
const AMAZON_BEDROCK_MANTLE_CLIENT_AGENT_VALUE: &str = "sory";
const CHAT_WIRE_API_REMOVED_ERROR: &str = "`wire_api = \"chat\"` is no longer supported.\nHow to fix: set `wire_api = \"chat_completions\"` in your provider config.\nMore info: https://github.com/openai/sory/discussions/7782";
pub const LEGACY_OLLAMA_CHAT_PROVIDER_ID: &str = "ollama-chat";
pub const OLLAMA_CHAT_PROVIDER_REMOVED_ERROR: &str = "`ollama-chat` is no longer supported.\nHow to fix: replace `ollama-chat` with `ollama` in `model_provider`, `oss_provider`, or `--local-provider`.\nMore info: https://github.com/openai/sory/discussions/7782";

/// Wire protocol that the provider speaks.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum WireApi {
    /// The Responses API exposed by OpenAI at `/v1/responses`.
    #[default]
    Responses,
    /// The Chat Completions API (OpenAI-compatible) at `/v1/chat/completions`.
    /// Used by Mistral, Gemini, Groq, DeepSeek, and other OpenAI-compatible providers.
    ChatCompletions,
}

impl fmt::Display for WireApi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Responses => "responses",
            Self::ChatCompletions => "chat_completions",
        };
        f.write_str(value)
    }
}

impl<'de> Deserialize<'de> for WireApi {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "responses" => Ok(Self::Responses),
            "chat_completions" | "chat" => Ok(Self::ChatCompletions),
            _ => Err(serde::de::Error::unknown_variant(&value, &["responses", "chat_completions"])),
        }
    }
}

/// Serializable representation of a provider definition.
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct ModelProviderInfo {
    /// Friendly display name.
    #[serde(default)]
    pub name: String,
    /// Base URL for the provider's OpenAI-compatible API.
    pub base_url: Option<String>,
    /// Environment variable that stores the user's API key for this provider.
    pub env_key: Option<String>,

    /// Optional instructions to help the user get a valid value for the
    /// variable and set it.
    pub env_key_instructions: Option<String>,
    /// Value to use with `Authorization: Bearer <token>` header. Use of this
    /// config is discouraged in favor of `env_key` for security reasons, but
    /// this may be necessary when using this programmatically.
    pub experimental_bearer_token: Option<String>,
    /// Command-backed bearer-token configuration for this provider.
    pub auth: Option<ModelProviderAuthInfo>,
    /// AWS SigV4 auth configuration for this provider.
    pub aws: Option<ModelProviderAwsAuthInfo>,
    /// Which wire protocol this provider expects.
    #[serde(default)]
    pub wire_api: WireApi,
    /// Optional query parameters to append to the base URL.
    pub query_params: Option<HashMap<String, String>>,
    /// Additional HTTP headers to include in requests to this provider where
    /// the (key, value) pairs are the header name and value.
    pub http_headers: Option<HashMap<String, String>>,
    /// Optional HTTP headers to include in requests to this provider where the
    /// (key, value) pairs are the header name and _environment variable_ whose
    /// value should be used. If the environment variable is not set, or the
    /// value is empty, the header will not be included in the request.
    pub env_http_headers: Option<HashMap<String, String>>,
    /// Maximum number of times to retry a failed HTTP request to this provider.
    pub request_max_retries: Option<u64>,
    /// Number of times to retry reconnecting a dropped streaming response before failing.
    pub stream_max_retries: Option<u64>,
    /// Idle timeout (in milliseconds) to wait for activity on a streaming response before treating
    /// the connection as lost.
    pub stream_idle_timeout_ms: Option<u64>,
    /// Maximum time (in milliseconds) to wait for a websocket connection attempt before treating
    /// it as failed.
    pub websocket_connect_timeout_ms: Option<u64>,
    /// Does this provider require an OpenAI API Key or ChatGPT login token? If true,
    /// user is presented with login screen on first run, and login preference and token/key
    /// are stored in auth.json. If false (which is the default), login screen is skipped,
    /// and API key (if needed) comes from the "env_key" environment variable.
    #[serde(default)]
    pub requires_openai_auth: bool,
    /// Whether this provider supports the Responses API WebSocket transport.
    #[serde(default)]
    pub supports_websockets: bool,
}

/// AWS SigV4 auth configuration for a model provider.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct ModelProviderAwsAuthInfo {
    /// AWS profile name to use. When unset, the AWS SDK default chain decides.
    pub profile: Option<String>,
    /// AWS region to use for provider-specific endpoints.
    pub region: Option<String>,
}

impl ModelProviderInfo {
    pub fn validate(&self) -> std::result::Result<(), String> {
        if self.aws.is_some() {
            if self.supports_websockets {
                // TODO(celia-oai): Support AWS SigV4 signing for WebSocket
                // upgrade requests before allowing AWS-authenticated providers
                // to enable Responses-over-WebSocket.
                return Err("provider aws cannot be combined with supports_websockets".to_string());
            }

            let mut conflicts = Vec::new();
            if self.env_key.is_some() {
                conflicts.push("env_key");
            }
            if self.experimental_bearer_token.is_some() {
                conflicts.push("experimental_bearer_token");
            }
            if self.auth.is_some() {
                conflicts.push("auth");
            }
            if self.requires_openai_auth {
                conflicts.push("requires_openai_auth");
            }

            if !conflicts.is_empty() {
                return Err(format!(
                    "provider aws cannot be combined with {}",
                    conflicts.join(", ")
                ));
            }
        }

        let Some(auth) = self.auth.as_ref() else {
            return Ok(());
        };

        if auth.command.trim().is_empty() {
            return Err("provider auth.command must not be empty".to_string());
        }

        let mut conflicts = Vec::new();
        if self.env_key.is_some() {
            conflicts.push("env_key");
        }
        if self.experimental_bearer_token.is_some() {
            conflicts.push("experimental_bearer_token");
        }
        if self.requires_openai_auth {
            conflicts.push("requires_openai_auth");
        }

        if conflicts.is_empty() {
            Ok(())
        } else {
            Err(format!(
                "provider auth cannot be combined with {}",
                conflicts.join(", ")
            ))
        }
    }

    fn build_header_map(&self) -> soryResult<HeaderMap> {
        let capacity = self.http_headers.as_ref().map_or(0, HashMap::len)
            + self.env_http_headers.as_ref().map_or(0, HashMap::len);
        let mut headers = HeaderMap::with_capacity(capacity);
        if let Some(extra) = &self.http_headers {
            for (k, v) in extra {
                if let (Ok(name), Ok(value)) = (HeaderName::try_from(k), HeaderValue::try_from(v)) {
                    headers.insert(name, value);
                }
            }
        }

        if let Some(env_headers) = &self.env_http_headers {
            for (header, env_var) in env_headers {
                if let Ok(val) = std::env::var(env_var)
                    && !val.trim().is_empty()
                    && let (Ok(name), Ok(value)) =
                        (HeaderName::try_from(header), HeaderValue::try_from(val))
                {
                    headers.insert(name, value);
                }
            }
        }

        Ok(headers)
    }

    pub fn to_api_provider(&self, auth_mode: Option<AuthMode>) -> soryResult<ApiProvider> {
        let default_base_url = if matches!(
            auth_mode,
            Some(AuthMode::Chatgpt | AuthMode::ChatgptAuthTokens | AuthMode::AgentIdentity)
        ) {
            CHATGPT_SORY_BASE_URL
        } else {
            "https://api.openai.com/v1"
        };
        let base_url = self
            .base_url
            .clone()
            .unwrap_or_else(|| default_base_url.to_string());

        let headers = self.build_header_map()?;
        let retry = ApiRetryConfig {
            max_attempts: self.request_max_retries(),
            base_delay: Duration::from_millis(200),
            retry_429: false,
            retry_5xx: true,
            retry_transport: true,
        };

        Ok(ApiProvider {
            name: self.name.clone(),
            base_url,
            query_params: self.query_params.clone(),
            headers,
            retry,
            stream_idle_timeout: self.stream_idle_timeout(),
        })
    }

    /// If `env_key` is Some, returns the API key for this provider if present
    /// (and non-empty) in the environment. If `env_key` is required but
    /// cannot be found, returns an error.
    pub fn api_key(&self) -> soryResult<Option<String>> {
        match &self.env_key {
            Some(env_key) => {
                let api_key = std::env::var(env_key)
                    .ok()
                    .filter(|v| !v.trim().is_empty())
                    .ok_or_else(|| {
                        SoryErr::EnvVar(EnvVarError {
                            var: env_key.clone(),
                            instructions: self.env_key_instructions.clone(),
                        })
                    })?;
                Ok(Some(api_key))
            }
            None => Ok(None),
        }
    }

    /// Effective maximum number of request retries for this provider.
    pub fn request_max_retries(&self) -> u64 {
        self.request_max_retries
            .unwrap_or(DEFAULT_REQUEST_MAX_RETRIES)
            .min(MAX_REQUEST_MAX_RETRIES)
    }

    /// Effective maximum number of stream reconnection attempts for this provider.
    pub fn stream_max_retries(&self) -> u64 {
        self.stream_max_retries
            .unwrap_or(DEFAULT_STREAM_MAX_RETRIES)
            .min(MAX_STREAM_MAX_RETRIES)
    }

    /// Effective idle timeout for streaming responses.
    pub fn stream_idle_timeout(&self) -> Duration {
        self.stream_idle_timeout_ms
            .map(Duration::from_millis)
            .unwrap_or(Duration::from_millis(DEFAULT_STREAM_IDLE_TIMEOUT_MS))
    }

    /// Effective timeout for websocket connect attempts.
    pub fn websocket_connect_timeout(&self) -> Duration {
        self.websocket_connect_timeout_ms
            .map(Duration::from_millis)
            .unwrap_or(Duration::from_millis(DEFAULT_WEBSOCKET_CONNECT_TIMEOUT_MS))
    }

    pub fn create_openai_provider(base_url: Option<String>) -> ModelProviderInfo {
        ModelProviderInfo {
            name: OPENAI_PROVIDER_NAME.into(),
            base_url,
            env_key: None,
            env_key_instructions: None,
            experimental_bearer_token: None,
            auth: None,
            aws: None,
            wire_api: WireApi::Responses,
            query_params: None,
            http_headers: Some(
                [("version".to_string(), env!("CARGO_PKG_VERSION").to_string())]
                    .into_iter()
                    .collect(),
            ),
            env_http_headers: Some(
                [
                    (
                        "OpenAI-Organization".to_string(),
                        "OPENAI_ORGANIZATION".to_string(),
                    ),
                    ("OpenAI-Project".to_string(), "OPENAI_PROJECT".to_string()),
                ]
                .into_iter()
                .collect(),
            ),
            // Use global defaults for retry/timeout unless overridden in config.toml.
            request_max_retries: None,
            stream_max_retries: None,
            stream_idle_timeout_ms: None,
            websocket_connect_timeout_ms: None,
            requires_openai_auth: true,
            supports_websockets: true,
        }
    }

    pub fn create_amazon_bedrock_provider(
        aws: Option<ModelProviderAwsAuthInfo>,
    ) -> ModelProviderInfo {
        ModelProviderInfo {
            name: AMAZON_BEDROCK_PROVIDER_NAME.into(),
            base_url: Some(AMAZON_BEDROCK_DEFAULT_BASE_URL.into()),
            env_key: None,
            env_key_instructions: None,
            experimental_bearer_token: None,
            auth: None,
            aws: Some(aws.unwrap_or(ModelProviderAwsAuthInfo {
                profile: None,
                region: None,
            })),
            wire_api: WireApi::ChatCompletions,
            query_params: None,
            http_headers: Some(HashMap::from([(
                AMAZON_BEDROCK_MANTLE_CLIENT_AGENT_HEADER.to_string(),
                AMAZON_BEDROCK_MANTLE_CLIENT_AGENT_VALUE.to_string(),
            )])),
            env_http_headers: None,
            request_max_retries: None,
            stream_max_retries: None,
            stream_idle_timeout_ms: None,
            websocket_connect_timeout_ms: None,
            requires_openai_auth: false,
            supports_websockets: false,
        }
    }

    pub fn create_anthropic_provider() -> ModelProviderInfo {
        ModelProviderInfo {
            name: ANTHROPIC_PROVIDER_NAME.into(),
            base_url: Some("https://api.anthropic.com/v1".into()),
            env_key: Some("ANTHROPIC_API_KEY".into()),
            env_key_instructions: Some("Get your API key from https://console.anthropic.com/".into()),
            experimental_bearer_token: None,
            auth: None,
            aws: None,
            wire_api: WireApi::ChatCompletions,
            query_params: None,
            http_headers: None,
            env_http_headers: None,
            request_max_retries: None,
            stream_max_retries: None,
            stream_idle_timeout_ms: None,
            websocket_connect_timeout_ms: None,
            requires_openai_auth: false,
            supports_websockets: false,
        }
    }

    pub fn create_mistral_provider() -> ModelProviderInfo {
        ModelProviderInfo {
            name: MISTRAL_PROVIDER_NAME.into(),
            base_url: Some("https://api.mistral.ai/v1".into()),
            env_key: Some("MISTRAL_API_KEY".into()),
            env_key_instructions: Some("Get your API key from https://console.mistral.ai/".into()),
            experimental_bearer_token: None,
            auth: None,
            aws: None,
            wire_api: WireApi::ChatCompletions,
            query_params: None,
            http_headers: None,
            env_http_headers: None,
            request_max_retries: None,
            stream_max_retries: None,
            stream_idle_timeout_ms: None,
            websocket_connect_timeout_ms: None,
            requires_openai_auth: false,
            supports_websockets: false,
        }
    }

    pub fn create_google_provider() -> ModelProviderInfo {
        ModelProviderInfo {
            name: GOOGLE_PROVIDER_NAME.into(),
            base_url: Some("https://generativelanguage.googleapis.com/v1beta/openai".into()),
            env_key: Some("GOOGLE_API_KEY".into()),
            env_key_instructions: Some("Get your API key from https://aistudio.google.com/apikey".into()),
            experimental_bearer_token: None,
            auth: None,
            aws: None,
            wire_api: WireApi::ChatCompletions,
            query_params: None,
            http_headers: None,
            env_http_headers: None,
            request_max_retries: None,
            stream_max_retries: None,
            stream_idle_timeout_ms: None,
            websocket_connect_timeout_ms: None,
            requires_openai_auth: false,
            supports_websockets: false,
        }
    }

    pub fn create_groq_provider() -> ModelProviderInfo {
        ModelProviderInfo {
            name: GROQ_PROVIDER_NAME.into(),
            base_url: Some("https://api.groq.com/openai/v1".into()),
            env_key: Some("GROQ_API_KEY".into()),
            env_key_instructions: Some("Get your API key from https://console.groq.com/keys".into()),
            experimental_bearer_token: None,
            auth: None,
            aws: None,
            wire_api: WireApi::ChatCompletions,
            query_params: None,
            http_headers: None,
            env_http_headers: None,
            request_max_retries: None,
            stream_max_retries: None,
            stream_idle_timeout_ms: None,
            websocket_connect_timeout_ms: None,
            requires_openai_auth: false,
            supports_websockets: false,
        }
    }

    pub fn create_deepseek_provider() -> ModelProviderInfo {
        ModelProviderInfo {
            name: DEEPSEEK_PROVIDER_NAME.into(),
            base_url: Some("https://api.deepseek.com/v1".into()),
            env_key: Some("DEEPSEEK_API_KEY".into()),
            env_key_instructions: Some("Get your API key from https://platform.deepseek.com/api_keys".into()),
            experimental_bearer_token: None,
            auth: None,
            aws: None,
            wire_api: WireApi::ChatCompletions,
            query_params: None,
            http_headers: None,
            env_http_headers: None,
            request_max_retries: None,
            stream_max_retries: None,
            stream_idle_timeout_ms: None,
            websocket_connect_timeout_ms: None,
            requires_openai_auth: false,
            supports_websockets: false,
        }
    }

    pub fn create_cohere_provider() -> ModelProviderInfo {
        ModelProviderInfo {
            name: COHERE_PROVIDER_NAME.into(),
            base_url: Some("https://api.cohere.ai/v1".into()),
            env_key: Some("COHERE_API_KEY".into()),
            env_key_instructions: Some("Get your API key from https://dashboard.cohere.com/api-keys".into()),
            experimental_bearer_token: None,
            auth: None,
            aws: None,
            wire_api: WireApi::ChatCompletions,
            query_params: None,
            http_headers: None,
            env_http_headers: None,
            request_max_retries: None,
            stream_max_retries: None,
            stream_idle_timeout_ms: None,
            websocket_connect_timeout_ms: None,
            requires_openai_auth: false,
            supports_websockets: false,
        }
    }

    pub fn create_perplexity_provider() -> ModelProviderInfo {
        ModelProviderInfo {
            name: PERPLEXITY_PROVIDER_NAME.into(),
            base_url: Some("https://api.perplexity.ai/v1".into()),
            env_key: Some("PERPLEXITY_API_KEY".into()),
            env_key_instructions: Some("Get your API key from https://www.perplexity.ai/settings/api".into()),
            experimental_bearer_token: None,
            auth: None,
            aws: None,
            wire_api: WireApi::ChatCompletions,
            query_params: None,
            http_headers: None,
            env_http_headers: None,
            request_max_retries: None,
            stream_max_retries: None,
            stream_idle_timeout_ms: None,
            websocket_connect_timeout_ms: None,
            requires_openai_auth: false,
            supports_websockets: false,
        }
    }

    pub fn create_together_provider() -> ModelProviderInfo {
        ModelProviderInfo {
            name: TOGETHER_PROVIDER_NAME.into(),
            base_url: Some("https://api.together.xyz/v1".into()),
            env_key: Some("TOGETHER_API_KEY".into()),
            env_key_instructions: Some("Get your API key from https://api.together.xyz/settings/api-keys".into()),
            experimental_bearer_token: None,
            auth: None,
            aws: None,
            wire_api: WireApi::ChatCompletions,
            query_params: None,
            http_headers: None,
            env_http_headers: None,
            request_max_retries: None,
            stream_max_retries: None,
            stream_idle_timeout_ms: None,
            websocket_connect_timeout_ms: None,
            requires_openai_auth: false,
            supports_websockets: false,
        }
    }

    pub fn create_fireworks_provider() -> ModelProviderInfo {
        ModelProviderInfo {
            name: FIREWORKS_PROVIDER_NAME.into(),
            base_url: Some("https://api.fireworks.ai/inference/v1".into()),
            env_key: Some("FIREWORKS_API_KEY".into()),
            env_key_instructions: Some("Get your API key from https://fireworks.ai/api-keys".into()),
            experimental_bearer_token: None,
            auth: None,
            aws: None,
            wire_api: WireApi::ChatCompletions,
            query_params: None,
            http_headers: None,
            env_http_headers: None,
            request_max_retries: None,
            stream_max_retries: None,
            stream_idle_timeout_ms: None,
            websocket_connect_timeout_ms: None,
            requires_openai_auth: false,
            supports_websockets: false,
        }
    }

    pub fn create_deepinfra_provider() -> ModelProviderInfo {
        ModelProviderInfo {
            name: DEEPINFRA_PROVIDER_NAME.into(),
            base_url: Some("https://api.deepinfra.com/v1/openai".into()),
            env_key: Some("DEEPINFRA_API_KEY".into()),
            env_key_instructions: Some("Get your API key from https://deepinfra.com/dash/api_keys".into()),
            experimental_bearer_token: None,
            auth: None,
            aws: None,
            wire_api: WireApi::ChatCompletions,
            query_params: None,
            http_headers: None,
            env_http_headers: None,
            request_max_retries: None,
            stream_max_retries: None,
            stream_idle_timeout_ms: None,
            websocket_connect_timeout_ms: None,
            requires_openai_auth: false,
            supports_websockets: false,
        }
    }

    pub fn create_openrouter_provider() -> ModelProviderInfo {
        ModelProviderInfo {
            name: OPENROUTER_PROVIDER_NAME.into(),
            base_url: Some("https://openrouter.ai/api/v1".into()),
            env_key: Some("OPENROUTER_API_KEY".into()),
            env_key_instructions: Some("Get your API key from https://openrouter.ai/keys".into()),
            experimental_bearer_token: None,
            auth: None,
            aws: None,
            wire_api: WireApi::ChatCompletions,
            query_params: None,
            http_headers: None,
            env_http_headers: None,
            request_max_retries: None,
            stream_max_retries: None,
            stream_idle_timeout_ms: None,
            websocket_connect_timeout_ms: None,
            requires_openai_auth: false,
            supports_websockets: false,
        }
    }

    pub fn create_xai_provider() -> ModelProviderInfo {
        ModelProviderInfo {
            name: XAI_PROVIDER_NAME.into(),
            base_url: Some("https://api.x.ai/v1".into()),
            env_key: Some("XAI_API_KEY".into()),
            env_key_instructions: Some("Get your API key from https://console.x.ai/".into()),
            experimental_bearer_token: None,
            auth: None,
            aws: None,
            wire_api: WireApi::ChatCompletions,
            query_params: None,
            http_headers: None,
            env_http_headers: None,
            request_max_retries: None,
            stream_max_retries: None,
            stream_idle_timeout_ms: None,
            websocket_connect_timeout_ms: None,
            requires_openai_auth: false,
            supports_websockets: false,
        }
    }

    pub fn create_nvidia_provider() -> ModelProviderInfo {
        ModelProviderInfo {
            name: NVIDIA_PROVIDER_NAME.into(),
            base_url: Some("https://integrate.api.nvidia.com/v1".into()),
            env_key: Some("NVIDIA_API_KEY".into()),
            env_key_instructions: Some("Get your API key from https://build.nvidia.com/".into()),
            experimental_bearer_token: None,
            auth: None,
            aws: None,
            wire_api: WireApi::ChatCompletions,
            query_params: None,
            http_headers: None,
            env_http_headers: None,
            request_max_retries: None,
            stream_max_retries: None,
            stream_idle_timeout_ms: None,
            websocket_connect_timeout_ms: None,
            requires_openai_auth: false,
            supports_websockets: false,
        }
    }

    pub fn create_cerebras_provider() -> ModelProviderInfo {
        ModelProviderInfo {
            name: CEREBRAS_PROVIDER_NAME.into(),
            base_url: Some("https://api.cerebras.ai/v1".into()),
            env_key: Some("CEREBRAS_API_KEY".into()),
            env_key_instructions: Some("Get your API key from https://cloud.cerebras.ai/".into()),
            experimental_bearer_token: None,
            auth: None,
            aws: None,
            wire_api: WireApi::ChatCompletions,
            query_params: None,
            http_headers: None,
            env_http_headers: None,
            request_max_retries: None,
            stream_max_retries: None,
            stream_idle_timeout_ms: None,
            websocket_connect_timeout_ms: None,
            requires_openai_auth: false,
            supports_websockets: false,
        }
    }

pub fn create_soryos_zen_provider() -> ModelProviderInfo {
        ModelProviderInfo {
            name: SORYOS_ZEN_PROVIDER_NAME.into(),
            base_url: Some("https://opencode.ai/zen/v1".into()),
            env_key: Some("OPENCODE_API_KEY".into()),
            env_key_instructions: Some("Set OPENCODE_API_KEY for paid models, or leave unset to use the free public key.".into()),
            experimental_bearer_token: Some("public".into()),
            auth: None,
            aws: None,
            wire_api: WireApi::ChatCompletions,
            query_params: None,
            http_headers: None,
            env_http_headers: None,
            request_max_retries: None,
            stream_max_retries: None,
            stream_idle_timeout_ms: None,
            websocket_connect_timeout_ms: None,
            requires_openai_auth: false,
            supports_websockets: false,
        }
    }

    pub fn create_opencode_go_provider() -> ModelProviderInfo {
        ModelProviderInfo {
            name: OPENCODE_GO_PROVIDER_NAME.into(),
            base_url: Some("https://opencode.ai/zen/go/v1".into()),
            env_key: Some("OPENCODE_API_KEY".into()),
            env_key_instructions: Some("Get your API key from https://opencode.ai/docs/zen".into()),
            experimental_bearer_token: None,
            auth: None,
            aws: None,
            wire_api: WireApi::ChatCompletions,
            query_params: None,
            http_headers: None,
            env_http_headers: None,
            request_max_retries: None,
            stream_max_retries: None,
            stream_idle_timeout_ms: None,
            websocket_connect_timeout_ms: None,
            requires_openai_auth: false,
            supports_websockets: false,
        }
    }

    pub fn create_github_copilot_provider() -> ModelProviderInfo {
        ModelProviderInfo {
            name: GITHUB_COPILOT_PROVIDER_NAME.into(),
            base_url: Some("https://api.githubcopilot.com/v1".into()),
            env_key: Some("GITHUB_TOKEN".into()),
            env_key_instructions: Some("Get your token from https://github.com/settings/tokens".into()),
            experimental_bearer_token: None,
            auth: None,
            aws: None,
            wire_api: WireApi::ChatCompletions,
            query_params: None,
            http_headers: None,
            env_http_headers: None,
            request_max_retries: None,
            stream_max_retries: None,
            stream_idle_timeout_ms: None,
            websocket_connect_timeout_ms: None,
            requires_openai_auth: false,
            supports_websockets: false,
        }
    }

    pub fn is_openai(&self) -> bool {
        self.name == OPENAI_PROVIDER_NAME
    }

    pub fn is_amazon_bedrock(&self) -> bool {
        self.name == AMAZON_BEDROCK_PROVIDER_NAME
    }

    pub fn supports_remote_compaction(&self) -> bool {
        self.is_openai() || is_azure_responses_provider(&self.name, self.base_url.as_deref())
    }

    pub fn has_command_auth(&self) -> bool {
        self.auth.is_some()
    }
}

pub const DEFAULT_LMSTUDIO_PORT: u16 = 1234;
pub const DEFAULT_OLLAMA_PORT: u16 = 11434;

pub const LMSTUDIO_OSS_PROVIDER_ID: &str = "lmstudio";
pub const OLLAMA_OSS_PROVIDER_ID: &str = "ollama";

/// Built-in default provider list.
pub fn built_in_model_providers(
    openai_base_url: Option<String>,
) -> HashMap<String, ModelProviderInfo> {
    use ModelProviderInfo as P;
    let openai_provider = P::create_openai_provider(openai_base_url);
    let amazon_bedrock_provider = P::create_amazon_bedrock_provider(/*aws*/ None);

    [
        (OPENAI_PROVIDER_ID, openai_provider),
        (ANTHROPIC_PROVIDER_ID, P::create_anthropic_provider()),
        (MISTRAL_PROVIDER_ID, P::create_mistral_provider()),
        (GOOGLE_PROVIDER_ID, P::create_google_provider()),
        (GROQ_PROVIDER_ID, P::create_groq_provider()),
        (DEEPSEEK_PROVIDER_ID, P::create_deepseek_provider()),
        (COHERE_PROVIDER_ID, P::create_cohere_provider()),
        (PERPLEXITY_PROVIDER_ID, P::create_perplexity_provider()),
        (TOGETHER_PROVIDER_ID, P::create_together_provider()),
        (FIREWORKS_PROVIDER_ID, P::create_fireworks_provider()),
        (DEEPINFRA_PROVIDER_ID, P::create_deepinfra_provider()),
        (OPENROUTER_PROVIDER_ID, P::create_openrouter_provider()),
        (XAI_PROVIDER_ID, P::create_xai_provider()),
        (NVIDIA_PROVIDER_ID, P::create_nvidia_provider()),
        (CEREBRAS_PROVIDER_ID, P::create_cerebras_provider()),
        (GITHUB_COPILOT_PROVIDER_ID, P::create_github_copilot_provider()),
        (SORYOS_ZEN_PROVIDER_ID, P::create_soryos_zen_provider()),
        (OPENCODE_GO_PROVIDER_ID, P::create_opencode_go_provider()),
        (AMAZON_BEDROCK_PROVIDER_ID, amazon_bedrock_provider),
        (
            OLLAMA_OSS_PROVIDER_ID,
            create_oss_provider(DEFAULT_OLLAMA_PORT, WireApi::ChatCompletions),
        ),
        (
            LMSTUDIO_OSS_PROVIDER_ID,
            create_oss_provider(DEFAULT_LMSTUDIO_PORT, WireApi::ChatCompletions),
        ),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v))
    .collect()
}

/// Merge configured providers into the built-in provider catalog.
///
/// Configured providers extend the built-in set. Built-in providers are not
/// generally overridable, but the built-in Amazon Bedrock provider allows the
/// user to set `aws.profile` and `aws.region`.
pub fn merge_configured_model_providers(
    mut model_providers: HashMap<String, ModelProviderInfo>,
    configured_model_providers: HashMap<String, ModelProviderInfo>,
) -> Result<HashMap<String, ModelProviderInfo>, String> {
    for (key, mut provider) in configured_model_providers {
        if key == AMAZON_BEDROCK_PROVIDER_ID {
            let aws_override = provider.aws.take();
            if provider != ModelProviderInfo::default() {
                return Err(format!(
                    "model_providers.{AMAZON_BEDROCK_PROVIDER_ID} only supports changing \
`aws.profile` and `aws.region`; other non-default provider fields are not supported"
                ));
            }

            if let Some(aws_override) = aws_override
                && let Some(built_in_provider) = model_providers.get_mut(AMAZON_BEDROCK_PROVIDER_ID)
                && let Some(built_in_aws) = built_in_provider.aws.as_mut()
            {
                if let Some(profile) = aws_override.profile {
                    built_in_aws.profile = Some(profile);
                }
                if let Some(region) = aws_override.region {
                    built_in_aws.region = Some(region);
                }
            }
        } else {
            model_providers.entry(key).or_insert(provider);
        }
    }

    Ok(model_providers)
}

pub fn create_oss_provider(default_provider_port: u16, wire_api: WireApi) -> ModelProviderInfo {
    // These sory_OSS_ environment variables are experimental: we may
    // switch to reading values from config.toml instead.
    let default_sory_oss_base_url = format!(
        "http://localhost:{sory_oss_port}/v1",
        sory_oss_port = std::env::var("sory_OSS_PORT")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .and_then(|value| value.parse::<u16>().ok())
            .unwrap_or(default_provider_port)
    );

    let sory_oss_base_url = std::env::var("sory_OSS_BASE_URL")
        .ok()
        .filter(|v| !v.trim().is_empty())
        .unwrap_or(default_sory_oss_base_url);
    create_oss_provider_with_base_url(&sory_oss_base_url, wire_api)
}

pub fn create_oss_provider_with_base_url(base_url: &str, wire_api: WireApi) -> ModelProviderInfo {
    ModelProviderInfo {
        name: "gpt-oss".into(),
        base_url: Some(base_url.into()),
        env_key: None,
        env_key_instructions: None,
        experimental_bearer_token: None,
        auth: None,
        aws: None,
        wire_api,
        query_params: None,
        http_headers: None,
        env_http_headers: None,
        request_max_retries: None,
        stream_max_retries: None,
        stream_idle_timeout_ms: None,
        websocket_connect_timeout_ms: None,
        requires_openai_auth: false,
        supports_websockets: false,
    }
}

#[cfg(test)]
#[path = "model_provider_info_tests.rs"]
mod tests;
