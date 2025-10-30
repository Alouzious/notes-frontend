use axum::{
    extract::{Path, Extension},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::notes::Note;

#[axum::debug_handler]
async fn list_notes(Extension(pool): Extension<PgPool>) -> Json<Vec<Note>> {
    let notes = sqlx::query_as::<_, Note>("SELECT * FROM notes")
        .fetch_all(&pool)
        .await
        .unwrap_or_else(|_| vec![]);
    Json(notes)
}

#[axum::debug_handler]
async fn get_note(
    Path(id): Path<Uuid>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Note>, StatusCode> {
    let note = sqlx::query_as::<_, Note>("SELECT * FROM notes WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    
    Ok(Json(note))
}

#[axum::debug_handler]
async fn create_note(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<Note>,
) -> Result<Json<Note>, StatusCode> {
    let new_note = Note {
        id: Uuid::new_v4(),
        title: payload.title,
        content: payload.content,
    };

    sqlx::query("INSERT INTO notes (id, title, content) VALUES ($1, $2, $3)")
        .bind(new_note.id)
        .bind(&new_note.title)
        .bind(&new_note.content)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(new_note))
}

#[axum::debug_handler]
async fn update_note(
    Path(id): Path<Uuid>,
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<Note>,
) -> Result<Json<Note>, StatusCode> {
    let updated = sqlx::query("UPDATE notes SET title = $1, content = $2 WHERE id = $3")
        .bind(&payload.title)
        .bind(&payload.content)
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if updated.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(Json(payload))
}

#[axum::debug_handler]
async fn delete_note(
    Path(id): Path<Uuid>,
    Extension(pool): Extension<PgPool>,
) -> Result<StatusCode, StatusCode> {
    let deleted = sqlx::query("DELETE FROM notes WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if deleted.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}

pub fn notes_router(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(list_notes).post(create_note))
        .route("/:id", get(get_note).put(update_note).delete(delete_note))
        .layer(Extension(pool))
}
