// Redis key 前缀常量
pub struct CacheKeys;

impl CacheKeys {
    // AI 模型相关
    pub const MODEL_PREFIX: &'static str = "ai:model:";

    // 键生成方法
    pub fn get_key(key_prefix: &str, key: &str) -> String {
        format!("{}{}", key_prefix, key)
    }
}
