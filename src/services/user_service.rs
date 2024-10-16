use sqlx::PgPool;
use crate::models::User;

pub async fn get_user_by_id(pool: &PgPool, user_id: i32) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        "SELECT id, username, email FROM users WHERE id = $1",
        user_id
    )
    .fetch_optional(pool)
    .await
}
