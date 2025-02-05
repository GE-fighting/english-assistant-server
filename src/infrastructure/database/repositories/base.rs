use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// 基础仓储特征，定义通用的CRUD操作
#[async_trait]
pub trait Repository<T, ID> {
    /// 根据ID查找实体
    async fn find_by_id(&self, id: ID) -> Result<Option<T>>;

    /// 查找所有实体
    async fn find_all(&self) -> Result<Vec<T>>;

    /// 保存或更新实体
    async fn save(&self, entity: &T) -> Result<T>;

    /// 根据ID删除实体
    async fn delete(&self, id: ID) -> Result<()>;
}

/// 分页泛型对象
#[derive(Debug, Deserialize, Serialize)]
pub struct Paginated<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
}

