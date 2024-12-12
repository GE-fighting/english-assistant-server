use crate::models::entity::word_unit_mapping::WordUnitMapping;
use anyhow::Result;
use sqlx::PgPool;

//根据单词单元id查找单词
pub async fn find_by_unit_id(pool: &PgPool, unit_id: i32) -> Result<Vec<WordUnitMapping>> {
    let records = sqlx::query_as!(
        WordUnitMapping,
        "SELECT * FROM word_unit_mappings WHERE unit_id = $1",
        unit_id
    )
    .fetch_all(pool)
    .await?;

    Ok(records)
}

//创建单词单元关联记录
pub async fn create(pool: &PgPool, word_id: i32, unit_id: i32) -> Result<WordUnitMapping> {
    let record = sqlx::query_as!(
        WordUnitMapping,
        "INSERT INTO word_unit_mappings (word_id, unit_id) VALUES ($1, $2) RETURNING *",
        word_id,
        unit_id
    )
    .fetch_one(pool)
    .await?;
    Ok(record)
}

// 根据id删除单词单元关联记录
pub async fn delete_by_id(pool: &PgPool, id: i32) -> Result<()> {
    sqlx::query!("DELETE FROM word_unit_mappings WHERE id = $1", id)
        .execute(pool)
        .await?;
    Ok(())
}
