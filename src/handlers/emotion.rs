use actix_web::{HttpResponse, get, web};
use sqlx::PgPool;

use crate::errors::AppError;
use crate::models::emotion::{EmotionBase, SousEmotion};

#[get("/api/emotions")]
pub async fn list_emotions(pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    let emotions = sqlx::query_as::<_, EmotionBase>(
        "SELECT id, nom, emoji, couleur FROM emotion_base ORDER BY id",
    )
    .fetch_all(pool.get_ref())
    .await?;

    // emotions = 2;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": emotions,
    })))
}

#[get("/api/emotions/{id}/sub-emotions")]
pub async fn list_sub_emotions(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let subs = sqlx::query_as::<_, SousEmotion>(
        "SELECT id, nom, emotion_base_id FROM sous_emotion WHERE emotion_base_id = $1 ORDER BY id",
    )
    .bind(path.into_inner())
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": subs,
    })))
}
