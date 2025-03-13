use schemars::{schema_for, JsonSchema};
use serde::Serialize;
use std::collections::HashMap;

pub struct LanguageModelV1CallSettings {
    /// Maximum number of tokens to generate.
    pub max_tokens: Option<u32>,

    /// Temperature setting.
    /// It is recommended to set either `temperature` or `top_p`, but not both.
    pub temperature: Option<f32>,

    /// Stop sequences.
    /// If set, the model will stop generating text when one of the stop sequences is generated.
    /// Providers may have limits on the number of stop sequences.
    pub stop_sequences: Option<Vec<String>>,

    /// Nucleus sampling.
    /// It is recommended to set either `temperature` or `top_p`, but not both.
    pub top_p: Option<f32>,

    /// Only sample from the top K options for each subsequent token.
    /// Used to remove "long tail" low probability responses.
    /// Recommended for advanced use cases only. You usually only need to use temperature.
    pub top_k: Option<u32>,

    /// Presence penalty setting. It affects the likelihood of the model to
    /// repeat information that is already in the prompt.
    pub presence_penalty: Option<f32>,

    /// Frequency penalty setting. It affects the likelihood of the model
    /// to repeatedly use the same words or phrases.
    pub frequency_penalty: Option<f32>,

    /// Response format. The output can either be text or JSON.
    pub response_format: Option<ResponseFormat>,

    /// The seed (integer) to use for random sampling. If set and supported
    /// by the model, calls will generate deterministic results.
    pub seed: Option<u64>,

    /// Abort signal for cancelling the operation.
    pub abort_signal: Option<AbortSignal>,

    /// Additional HTTP headers to be sent with the request.
    /// Only applicable for HTTP-based providers.
    pub headers: Option<HashMap<String, Option<String>>>,
}

/// Response format options
pub enum ResponseFormat {
    /// Plain text output
    Text,
    /// JSON output with optional schema and metadata
    Json {
        /// JSON schema that the generated output should conform to.
        schema: ResponseFormatJsonSchema,
        /// Name of output that should be generated. Used by some providers for additional LLM guidance.
        name: Option<String>,
        /// Description of the output that should be generated. Used by some providers for additional LLM guidance.
        description: Option<String>,
    },
}

pub trait JsonSchemaSerializable {
    fn to_json_schema(&self) -> String;
}

impl<T> JsonSchemaSerializable for T
where
    T: Serialize + JsonSchema + 'static,
{
    fn to_json_schema(&self) -> String {
        let schema = schema_for!(T);
        serde_json::to_string(&schema).unwrap_or_else(|_| "{}".to_string())
    }
}

pub enum ResponseFormatJsonSchema {
    Struct(Box<dyn JsonSchemaSerializable>),
    JsonSchemaString(String),
}

/// Type for abort signal
pub struct AbortSignal {
    // Implementation details would depend on your specific cancellation mechanism
}

pub struct LanguageModelV1CallOptions {}
