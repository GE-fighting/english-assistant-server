use crate::models::dto::textbook_dto::TextbookDTO;
use crate::models::dto::unit_dto::UnitDTO;
use crate::models::entity::unit::{self, Unit};
use anyhow::{Error, Result};
use sqlx::{query_as, PgPool};

//主键id查询单个单元
pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Unit> {
    let unit = query_as!(Unit, "SELECT * FROM units WHERE id = $1", id)
        .fetch_one(pool)
        .await?;
    Ok(unit)
}

//删除单元
pub async fn delete_unit(pool: &PgPool, unit_dto: &UnitDTO) -> Result<()> {
    // 根据unit_dto中的非空字段 添加where条件
    let mut query = "DELETE FROM units WHERE 1=1".to_string();
    let mut params_index = 1;
    if let Some(_) = unit_dto.id {
        query += &format!(" AND id = ${}", params_index);
        params_index += 1;
    }
    if let Some(_) = unit_dto.textbook_id {
        query += &format!(" AND textbook_id = ${}", params_index);
        params_index += 1;
    }
    println!("query: {}", query);

    let mut query_builder = sqlx::query(&query);
    if let Some(id) = unit_dto.id {
        query_builder = query_builder.bind(id);
    }
    if let Some(textbook_id) = unit_dto.textbook_id {
        query_builder = query_builder.bind(textbook_id);
    }

    query_builder.execute(pool).await?;

    Ok(())
}

//查询单元列表
pub async fn get_unit(pool: &PgPool, unit_dto: &UnitDTO) -> Result<Vec<UnitDTO>> {
    let mut query = "SELECT * FROM units WHERE 1=1".to_string();
    let mut params_index = 1;
    if let Some(_) = unit_dto.id {
        query += &format!(" AND id = ${}", params_index);
        params_index += 1;
    }
    if let Some(_) = unit_dto.textbook_id {
        query += &format!(" AND textbook_id = ${}", params_index);
        params_index += 1;
    }
    if let Some(name) = &unit_dto.name {
        query += &format!(" AND name LIKE '%{}%'", name);
    }
    let mut query_builder = sqlx::query_as::<_, UnitDTO>(&query);
    println!("query: {}", query);
    if let Some(id) = unit_dto.id {
        query_builder = query_builder.bind(id);
    }
    if let Some(textbook_id) = unit_dto.textbook_id {
        query_builder = query_builder.bind(textbook_id);
    }
    let result = query_builder.fetch_all(pool).await?;
    Ok(result)
}

//根据textbook_dto查询unit列表
pub async fn get_unit_by_textbook_dto(
    pool: &PgPool,
    textbook_dto: &TextbookDTO,
) -> Result<Vec<Unit>> {
    let mut query =
        "SELECT u.* FROM units u JOIN textbooks t ON u.textbook_id = t.id WHERE 1=1".to_string();
    let mut params = Vec::new();
    let mut param_index = 1;

    if let Some(id) = textbook_dto.id {
        query.push_str(&format!(" AND t.id = ${}", param_index));
        params.push(id);
        param_index += 1;
    }
    if let Some(version_id) = &textbook_dto.version_id {
        query.push_str(&format!(" AND t.version_id = ${}", param_index));
        params.push(*version_id);
        param_index += 1;
    }
    if let Some(grade_id) = &textbook_dto.grade_id {
        query.push_str(&format!(" AND t.grade_id = ${}", param_index));
        params.push(*grade_id);
        param_index += 1;
    }
    if let Some(semester_id) = &textbook_dto.semester_id {
        query.push_str(&format!(" AND t.semester_id = ${}", param_index));
        params.push(*semester_id);
        param_index += 1;
    }

    let units = params
        .iter()
        .fold(sqlx::query_as::<_, Unit>(&query), |q, p| q.bind(p))
        .fetch_all(pool)
        .await?;

    Ok(units)
}

//更新单元
pub async fn update_unit(pool: &PgPool, unit_entity: &Unit) -> Result<Unit, Error> {
    let id = unit_entity
        .id
        .ok_or_else(|| Error::msg("unit id is required"))?;
    let mut query_builder = sqlx::QueryBuilder::new("UPDATE units SET ");
    let mut first = true;

    // 辅助宏，简化字段更新
    macro_rules! update_field {
        ($field:expr, $value:expr) => {
            if let Some(val) = $value {
                if !first {
                    query_builder.push(", ");
                }
                query_builder.push($field).push(" = ").push_bind(val);
                first = false;
            }
        };
    }

    // 更新所有可能的字段
    update_field!("name", &unit_entity.name);
    update_field!("textbook_id", &unit_entity.textbook_id);
    update_field!("sequence_number", &unit_entity.sequence_number);
    update_field!("word_count", &unit_entity.word_count);

    // 完成查询并执行
    query_builder
        .push(" WHERE id = ")
        .push_bind(id)
        .push(" RETURNING *");

    let updated_unit = query_builder
        .build_query_as::<Unit>()
        .fetch_one(pool)
        .await?;
    Ok(updated_unit)
}
