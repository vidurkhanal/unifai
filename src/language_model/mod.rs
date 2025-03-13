mod call;

use anyhow::Result;
use call::LanguageModelV1CallOptions;

/**
Specification for a language model that implements the language model interface version 1.
 */
pub struct LanguageModelV1 {
    /**
    The language model must specify which language model interface
    version it implements. This will allow us to evolve the language
    model interface and retain backwards compatibility. The different
    implementation versions can be handled as a discriminated union
    on our side.
    */
    specification_version: String,
    /**
    Name of the provider for logging purposes.
    */
    provider: String,
    /**
    Provider-specific model ID for logging purposes.
    */
    model_id: String,
    /**
    Default object generation mode that should be used with this model when
    no mode is specified. Should be the mode with the best results for this
    model. `undefined` can be returned if object generation is not supported.

    This is needed to generate the best objects possible w/o requiring the
    user to explicitly specify the object generation mode.
       */
    default_object_generation_mode: LanguageModelV1ObjectGenerationMode,

    /**
    Flag whether this model supports image/file URLs. Default is `true`.

    When the flag is set to `false`, the unifai will download the image/file and
    pass the image data to the model.
       */
    support_urls: bool,

    /**
    Flag whether this model supports grammar-guided generation,
    i.e. follows JSON schemas for object generation
    when the response format is set to 'json' or
    when the `object-json` mode is used.

    This means that the model guarantees that the generated JSON
    will be a valid JSON object AND that the object will match the
    JSON schema.

    Please note that `generateObject` and `streamObject` will work
    regardless of this flag, but might send different prompts and
    use further optimizations if this flag is set to `true`.

    Defaults to `false`.
    */
    supports_grammar_guided_generation: bool,
}

pub trait CanGenerate {
    fn do_generate(options: LanguageModelV1CallOptions) -> Result<LanguageModelV1GenerationResult>;
}

pub enum LanguageModelV1ObjectGenerationMode {
    Json,
    Tool,
    None,
}

pub struct LanguageModelV1GenerationResult {
    /**
    Text that the model has generated.
    Can be undefined if the model did not generate any text.
         */
    text: Option<String>,
    /**
    Tool calls that the model has generated.
    Can be undefined if the model did not generate any tool calls.
         */
    tool_calls: Option<LanguageModelV1FunctionToolCall>,
    /**
    Finish reason.
         */
    finish_reason: LanguageModelV1FinishReason,
    /**
    Usage information.
       */
    usage: LanguageModelV1Usage,
    warnings: Option<LanguageModelV1CallWarning>,
    /**
    Additional provider-specific metadata. They are passed through
    from the provider to the AI SDK and enable provider-specific
    results that can be fully encapsulated in the provider.
         */
    provider_metadata: Option<LanguageModelV1ProviderMetadata>,

    /**
    Sources that have been used as input to generate the response.
         */
    sources: Option<Vec<LanguageModelV1Source>>,
}

pub struct LanguageModelV1CallWarning;
pub struct LanguageModelV1Source;
pub struct LanguageModelV1FunctionToolCall;

pub enum LanguageModelV1FinishReason {}

pub struct LanguageModelV1Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
}

pub struct LanguageModelV1ProviderMetadata;
