use crate::{errors::NoSuchModelError, language_model::LanguageModelV1};
use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;

type Settings = HashMap<String, String>;
pub type LanguageModel = LanguageModelV1;

#[async_trait]
/// Base Provider for LLMProviders to build upon
trait Provider {
    /// Returns the language model with the given id.
    /// The model id is then passed to the provider function to get the model.
    ///
    /// # Arguments
    /// * `model_id` - The id of the model to return.
    ///
    /// # Returns
    /// The language model associated with the id
    ///
    /// # Errors
    /// Throws `NoSuchModelError` if no such model exists.
    fn language_model(&self, model_id: &str) -> Result<LanguageModel, NoSuchModelError>;
}
