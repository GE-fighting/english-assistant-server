use crate::models::entity::textbook_version::TextbookVersion;
use sqlx::PgPool;

pub async fn get_textbook_version(pool: &PgPool, id: i32) -> Result<TextbookVersion, sqlx::Error> {
    sqlx::query_as!(
        TextbookVersion,
        r#"
        SELECT * FROM textbook_versions
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn create_textbook_version(
    pool: &PgPool,
    textbook_version: &TextbookVersion,
) -> Result<TextbookVersion, sqlx::Error> {
    sqlx::query_as!(
        TextbookVersion,
        r#"
        INSERT INTO textbook_versions (name)
        VALUES ($1)
        RETURNING *
        "#,
        textbook_version.name
    )
    .fetch_one(pool)
    .await
}

pub async fn update_textbook_version(
    pool: &PgPool,
    textbook_version: &TextbookVersion,
) -> Result<TextbookVersion, sqlx::Error> {
    sqlx::query_as!(
        TextbookVersion,
        r#"
        UPDATE textbook_versions
        SET name = $1
        WHERE id = $2
        RETURNING *
        "#,
        textbook_version.name,
        textbook_version.id
    )
    .fetch_one(pool)
    .await
}

pub async fn delete_textbook_version(pool: &PgPool, id: i32) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM textbook_versions
        WHERE id = $1
        "#,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}
