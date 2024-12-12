use crate::models::dto::textbook_dto::TextbookDTO;
use crate::models::textbook::Textbook;
use anyhow::Result;
use sqlx::PgPool;

//根据id删除教材
pub async fn delete_textbook(pool: &PgPool, id: i32) -> Result<()> {
    sqlx::query!("DELETE FROM textbooks WHERE id = $1", id)
        .execute(pool)
        .await?;
    Ok(())
}

//查询教材列表
pub async fn get_textbooks(pool: &PgPool, text_dto: &TextbookDTO) -> Result<Vec<Textbook>> {
    let mut query = "SELECT * FROM textbooks".to_string();
    let mut params: Vec<String> = vec![];
    if let Some(id) = text_dto.id {
        query += " WHERE id = $1";
        params.push(id.to_string());
    }
    if let Some(version_id) = &text_dto.version_id {
        query += " AND version_id = $1";
        params.push(version_id.to_string());
    }
    if let Some(grade_id) = &text_dto.grade_id {
        query += " AND grade_id = $1";
        params.push(grade_id.to_string());
    }
    if let Some(semester_id) = &text_dto.semester_id {
        query += " AND semester_id = $1";
        params.push(semester_id.to_string());
    }
    // 执行查询
    let textbooks = sqlx::query_as::<_, Textbook>(&query)
        .bind(params)
        .fetch_all(pool)
        .await?;
    Ok(textbooks)
}
