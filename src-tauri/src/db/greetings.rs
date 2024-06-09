use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::State;

use super::Pool;

#[derive(Serialize, Deserialize, Type)]
pub struct Greeting {
    id: i64,
    pub created: NaiveDateTime,
    pub message: String,
}

#[allow(dead_code)]
impl Greeting {
    pub fn new(message: String) -> Self {
        Self {
            id: 0,
            created: Utc::now().naive_utc(),
            message,
        }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub async fn get_all(pool: &Pool) -> Result<Vec<Greeting>, sqlx::Error> {
        let results = sqlx::query_as!(Greeting, "SELECT * FROM greeting")
            .fetch_all(pool)
            .await?;

        Ok(results)
    }

    pub async fn create(&self, pool: &Pool) -> Result<i64, sqlx::Error> {
        let id = sqlx::query!(
            "INSERT INTO greeting (created, message) VALUES ($1, $2)",
            self.created,
            self.message
        )
        .execute(pool)
        .await?
        .rows_affected();

        Ok(id as i64)
    }

    pub async fn delete(&self, pool: &Pool) -> Result<bool, sqlx::Error> {
        Self::delete_id(pool, self.id).await
    }

    pub async fn delete_id(pool: &Pool, id: i64) -> Result<bool, sqlx::Error> {
        let deleted = sqlx::query!("DELETE FROM greeting WHERE id = $1", id)
            .execute(pool)
            .await?
            .rows_affected();

        Ok(deleted >= 1)
    }
}

#[tauri::command]
#[specta::specta]
pub async fn get_all_grettings(pool: State<'_, Pool>) -> Result<Vec<Greeting>, String> {
    Greeting::get_all(&pool).await.map_err(|err| {
        human_errors::system_with_internal(
            "Failed to get greetings from database",
            "Try closing this app and deleting the database in the apps config folder",
            err,
        )
        .to_string()
    })
}

#[tauri::command]
#[specta::specta]
pub async fn create_greeting(pool: State<'_, Pool>, greeting: Greeting) -> Result<i64, String> {
    greeting.create(&pool).await.map_err(|err| {
        human_errors::system_with_internal(
            "Failed to create greetings from database",
            "Try closing this app and deleting the database in the apps config folder",
            err,
        )
        .to_string()
    })
}

#[tauri::command]
#[specta::specta]
pub async fn delete_greeting(pool: State<'_, Pool>, id: i64) -> Result<bool, String> {
    Greeting::delete_id(&pool, id).await.map_err(|err| {
        human_errors::system_with_internal(
            "Failed to create greetings from database",
            "Try closing this app and deleting the database in the apps config folder",
            err,
        )
        .to_string()
    })
}
