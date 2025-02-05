pub mod factory;
pub mod impl_deepseek;
pub mod impl_yi;
pub mod interface;
pub mod manager;
pub mod prompts;
pub mod provider;
pub mod utils;

pub use impl_yi::YiServiceImpl;
pub use interface::LLMService;

use crate::infrastructure::llm::factory::LLMServiceFactory;
use crate::infrastructure::llm::manager::LLMManager;
use anyhow::Result;
use once_cell::sync::OnceCell;
use std::sync::Arc;

static LLM_MANAGER: OnceCell<Arc<LLMManager<LLMServiceFactory>>> = OnceCell::new();

/// 初始化大模型管理服务
pub fn init_llm_manager() -> Result<()> {
    let factory = LLMServiceFactory;
    let manager = LLMManager::new(factory);
    LLM_MANAGER
        .set(Arc::new(manager))
        .map_err(|_| anyhow::anyhow!("LLM manager already initialized"))?;
    Ok(())
}

pub fn get_llm_manager() -> &'static Arc<LLMManager<LLMServiceFactory>> {
    LLM_MANAGER.get().expect("LLM manager not initialized")
}
