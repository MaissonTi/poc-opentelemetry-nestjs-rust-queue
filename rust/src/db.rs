use sqlx::PgPool;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use tracing::instrument;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[instrument(skip(pool))]
pub async fn insert_user(pool: &PgPool) -> sqlx::Result<()> {
    let user = User {
        id: Uuid::new_v4().to_string(),
        name: "test_consumer_rust".to_string(),
        email: format!("{}@example.com", Uuid::new_v4()),
    };

    sqlx::query(
        r#"INSERT INTO "users" (id, name, email) VALUES ($1, $2, $3)"#,
    )
    .bind(&user.id)
    .bind(&user.name)
    .bind(&user.email)
    .execute(pool)
    .await?;

    Ok(())
}