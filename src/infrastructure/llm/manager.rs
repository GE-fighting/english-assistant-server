use crate::infrastructure::llm::factory::LLMServiceFactoryTrait;
use crate::infrastructure::llm::LLMService;
use anyhow::Result;
use std::sync::Arc;

pub struct LLMManager<F>
where
    F: LLMServiceFactoryTrait,
{
    factory: F,
}

impl<F> LLMManager<F>
where
    F: LLMServiceFactoryTrait,
{
    pub fn new(factory: F) -> Self {
        LLMManager { factory }
    }

    pub fn get_llm_service(
        &self,
        provider_name: &str,
    ) -> Result<Arc<dyn LLMService + Send + Sync>> {
        self.factory.create_from_name(provider_name)
    }
}
