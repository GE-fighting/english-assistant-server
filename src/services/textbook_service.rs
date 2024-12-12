use crate::models::dto::textbook_dto::TextbookDTO;
use crate::models::dto::unit_dto::UnitDTO;
use crate::models::response::ApiResponse;
use crate::models::textbook::Textbook;
use crate::repositories::{textbook_repository, unit_repository};
use anyhow::Result;
use sqlx::PgPool;

//插入textbook
pub async fn create_textbook(pool: &PgPool, textbook: &Textbook) -> ApiResponse<Textbook> {
    //根据textbook_version id查出name
    let textbook_version_name = sqlx::query_scalar!(
        "SELECT name FROM textbook_versions WHERE id = $1",
        textbook.version_id
    )
    .fetch_one(pool)
    .await;
    //根据grade id查出name
    let grade_name =
        sqlx::query_scalar!("SELECT name FROM grades WHERE id = $1", textbook.grade_id)
            .fetch_one(pool)
            .await;
    //根据semester id查出name
    let semester_name = sqlx::query_scalar!(
        "SELECT name FROM semesters WHERE id = $1",
        textbook.semester_id
    )
    .fetch_one(pool)
    .await;
    match sqlx::query_as!(
        Textbook,
        "INSERT INTO textbooks (version_id, grade_id, semester_id, name, textbook_version, grade, semester)\
         VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
        textbook.version_id,
        textbook.grade_id,
        textbook.semester_id,
        textbook.name,
        textbook_version_name.unwrap(),
        grade_name.unwrap() ,
        semester_name.unwrap()
    )
    .fetch_one(pool)
    .await
    {
        Ok(textbook) => ApiResponse::success(textbook),
        Err(err) => ApiResponse::error(500, format!("Error creating textbook: {}", err)),
    }
}

//查询textbook列表
pub async fn get_textbooks(pool: &PgPool) -> ApiResponse<Vec<TextbookDTO>> {
    match sqlx::query_as!(Textbook, "SELECT * FROM textbooks")
        .fetch_all(pool)
        .await
    {
        Ok(textbooks) => {
            let mut result: Vec<TextbookDTO> = Vec::new();
            for textbook in textbooks {
                result.push(TextbookDTO::from(textbook));
            }
            ApiResponse::success(result)
        }
        Err(err) => ApiResponse::error(500, format!("Error fetching textbooks: {}", err)),
    }
}

//根据id删除textbook
pub async fn delete_textbook(pool: &PgPool, textbook_dto: &TextbookDTO) -> Result<()> {
    textbook_repository::delete_textbook(pool, textbook_dto.id.unwrap()).await
}

//根据textbook_dto 查询unit列表
pub async fn get_unit_by_textbook(
    pool: &PgPool,
    textbook_dto: &TextbookDTO,
) -> Result<Vec<UnitDTO>> {
    unit_repository::get_unit_by_textbook_dto(pool, textbook_dto)
        .await
        .map(|units| units.into_iter().map(|unit| UnitDTO::from(unit)).collect())
}
