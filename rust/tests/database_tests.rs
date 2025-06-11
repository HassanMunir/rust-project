use sqlx::{Sqlite, SqlitePool, Row};
use actix_web::{test};
use rust_backend_app::handlers::{AppState, TodoItem};
use chrono::Utc;
use uuid::Uuid;

#[actix_web::test]
async fn test_database_operations() {

    let pool = SqlitePool::connect("sqlite::memory:")
        .await
        .expect("Database connection failed");

    sqlx::query(
        r#"
        CREATE TABLE todos (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            completed BOOLEAN NOT NULL,
            created_at TEXT NOT NULL
        );
        "#,
    )
    .execute(&pool)
    .await
    .expect("Failed to create table");

    let result = sqlx::query("INSERT INTO todos (id, title, completed, created_at) VALUES ($1, $2, $3, $4)")
        .bind(Uuid::new_v4().to_string())
        .bind("Test Todo")
        .bind(false)
        .bind(Utc::now().to_rfc3339())
        .execute(&pool)
        .await;

    assert!(result.is_ok());

    let row = sqlx::query("SELECT title FROM todos WHERE title = $1")
        .bind("Test Todo")
        .fetch_one(&pool)
        .await
        .expect("Query failed");

    assert_eq!(row.get::<String, _>("title"), "Test Todo");
}
