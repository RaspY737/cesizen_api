use actix_web::{HttpResponse, delete, get, post, put, web};
use serde::Deserialize;
use sqlx::PgPool;

use crate::errors::AppError;
use crate::middleware::auth::AuthUser;
use crate::models::tracker::{
    CreateEntree, EmotionStat, EntreeListItem, EntreeTracker, TrackerStats,
};

#[derive(Deserialize)]
pub struct TrackerQuery {
    pub period: Option<String>,
    pub emotion_base_id: Option<i32>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[get("/api/tracker/entries")]
pub async fn list_entries(
    pool: web::Data<PgPool>,
    auth: AuthUser,
    query: web::Query<TrackerQuery>,
) -> Result<HttpResponse, AppError> {
    let limit = query.limit.unwrap_or(20).min(50);
    let offset = (query.page.unwrap_or(1) - 1) * limit;

    let period_filter = match query.period.as_deref() {
        Some("week") => "AND e.date_entree >= CURRENT_TIMESTAMP - INTERVAL '7 days'",
        Some("month") => "AND e.date_entree >= CURRENT_TIMESTAMP - INTERVAL '30 days'",
        Some("quarter") => "AND e.date_entree >= CURRENT_TIMESTAMP - INTERVAL '90 days'",
        Some("year") => "AND e.date_entree >= CURRENT_TIMESTAMP - INTERVAL '365 days'",
        _ => "",
    };

    let emotion_filter = if query.emotion_base_id.is_some() {
        "AND eb.id = $4"
    } else {
        "AND ($4::int IS NULL OR TRUE)"
    };

    let sql = format!(
        "SELECT e.id, e.sous_emotion_id, se.nom as sous_emotion_nom,
                eb.nom as emotion_base_nom, eb.emoji as emotion_base_emoji,
                e.intensite, e.note, e.date_entree
         FROM entree_tracker e
         JOIN sous_emotion se ON e.sous_emotion_id = se.id
         JOIN emotion_base eb ON se.emotion_base_id = eb.id
         WHERE e.utilisateur_id = $1 {period_filter} {emotion_filter}
         ORDER BY e.date_entree DESC LIMIT $2 OFFSET $3"
    );

    let entries = sqlx::query_as::<_, EntreeListItem>(&sql)
        .bind(auth.user_id)
        .bind(limit)
        .bind(offset)
        .bind(query.emotion_base_id)
        .fetch_all(pool.get_ref())
        .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": entries,
    })))
}

#[get("/api/tracker/entries/{id}")]
pub async fn get_entry(
    pool: web::Data<PgPool>,
    auth: AuthUser,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let entry = sqlx::query_as::<_, EntreeTracker>(
        "SELECT * FROM entree_tracker WHERE id = $1 AND utilisateur_id = $2",
    )
    .bind(path.into_inner())
    .bind(auth.user_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("Entrée non trouvée".to_string()))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": entry,
    })))
}

#[post("/api/tracker/entries")]
pub async fn create_entry(
    pool: web::Data<PgPool>,
    auth: AuthUser,
    body: web::Json<CreateEntree>,
) -> Result<HttpResponse, AppError> {
    if body.intensite < 1 || body.intensite > 10 {
        return Err(AppError::BadRequest(
            "L'intensité doit être entre 1 et 10".to_string(),
        ));
    }

    let date = body
        .date_entree
        .unwrap_or_else(|| chrono::Local::now().naive_local());

    let entry = sqlx::query_as::<_, EntreeTracker>(
        "INSERT INTO entree_tracker (utilisateur_id, sous_emotion_id, intensite, note, date_entree)
         VALUES ($1, $2, $3, $4, $5) RETURNING *",
    )
    .bind(auth.user_id)
    .bind(body.sous_emotion_id)
    .bind(body.intensite)
    .bind(&body.note)
    .bind(date)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "success": true,
        "data": entry,
    })))
}

#[put("/api/tracker/entries/{id}")]
pub async fn update_entry(
    pool: web::Data<PgPool>,
    auth: AuthUser,
    path: web::Path<i32>,
    body: web::Json<CreateEntree>,
) -> Result<HttpResponse, AppError> {
    if body.intensite < 1 || body.intensite > 10 {
        return Err(AppError::BadRequest(
            "L'intensité doit être entre 1 et 10".to_string(),
        ));
    }

    let entry = sqlx::query_as::<_, EntreeTracker>(
        "UPDATE entree_tracker SET sous_emotion_id = $1, intensite = $2, note = $3,
         date_modification = CURRENT_TIMESTAMP WHERE id = $4 AND utilisateur_id = $5 RETURNING *",
    )
    .bind(body.sous_emotion_id)
    .bind(body.intensite)
    .bind(&body.note)
    .bind(path.into_inner())
    .bind(auth.user_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("Entrée non trouvée".to_string()))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": entry,
    })))
}

#[delete("/api/tracker/entries/{id}")]
pub async fn delete_entry(
    pool: web::Data<PgPool>,
    auth: AuthUser,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let result = sqlx::query("DELETE FROM entree_tracker WHERE id = $1 AND utilisateur_id = $2")
        .bind(path.into_inner())
        .bind(auth.user_id)
        .execute(pool.get_ref())
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Entrée non trouvée".to_string()));
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "Entrée supprimée",
    })))
}

#[get("/api/tracker/stats")]
pub async fn get_stats(pool: web::Data<PgPool>, auth: AuthUser) -> Result<HttpResponse, AppError> {
    let total = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM entree_tracker WHERE utilisateur_id = $1",
    )
    .bind(auth.user_id)
    .fetch_one(pool.get_ref())
    .await?;

    let week_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM entree_tracker WHERE utilisateur_id = $1 AND date_entree >= CURRENT_TIMESTAMP - INTERVAL '7 days'",
    )
    .bind(auth.user_id)
    .fetch_one(pool.get_ref())
    .await?;

    let avg_intensity: f64 = sqlx::query_scalar::<_, Option<f64>>(
        "SELECT AVG(intensite::float) FROM entree_tracker WHERE utilisateur_id = $1",
    )
    .bind(auth.user_id)
    .fetch_one(pool.get_ref())
    .await?
    .unwrap_or(0.0);

    let dominant = sqlx::query_scalar::<_, String>(
        "SELECT eb.nom FROM entree_tracker et
         JOIN sous_emotion se ON et.sous_emotion_id = se.id
         JOIN emotion_base eb ON se.emotion_base_id = eb.id
         WHERE et.utilisateur_id = $1
         GROUP BY eb.nom ORDER BY COUNT(*) DESC LIMIT 1",
    )
    .bind(auth.user_id)
    .fetch_optional(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": TrackerStats {
            total_entrees: total,
            entrees_semaine: week_count,
            intensite_moyenne: (avg_intensity * 10.0).round() / 10.0,
            emotion_dominante: dominant,
        },
    })))
}

#[get("/api/reports/distribution")]
pub async fn get_distribution(
    pool: web::Data<PgPool>,
    auth: AuthUser,
) -> Result<HttpResponse, AppError> {
    let stats = sqlx::query_as::<_, EmotionStat>(
        "SELECT eb.nom as emotion_base_nom, eb.emoji as emotion_base_emoji, COUNT(*) as count
         FROM entree_tracker et
         JOIN sous_emotion se ON et.sous_emotion_id = se.id
         JOIN emotion_base eb ON se.emotion_base_id = eb.id
         WHERE et.utilisateur_id = $1
         GROUP BY eb.nom, eb.emoji ORDER BY count DESC",
    )
    .bind(auth.user_id)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": stats,
    })))
}
