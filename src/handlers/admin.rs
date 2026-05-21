use actix_web::{HttpResponse, delete, get, patch, post, put, web};
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHasher};
use serde::Deserialize;
use sqlx::PgPool;

use crate::errors::AppError;
use crate::middleware::auth::AdminUser;
use crate::models::emotion::{CreateEmotionBase, CreateSousEmotion, EmotionBase, SousEmotion};
use crate::models::information::{CreatePage, PageInformation, PageListItem};
use crate::storage::S3Storage;

// --- Content Management ---

#[derive(Deserialize)]
pub struct AdminPageQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[get("/api/admin/contents")]
pub async fn list_contents(
    pool: web::Data<PgPool>,
    _admin: AdminUser,
    query: web::Query<AdminPageQuery>,
) -> Result<HttpResponse, AppError> {
    let limit = query.limit.unwrap_or(20).min(50);
    let offset = (query.page.unwrap_or(1) - 1) * limit;

    let pages = sqlx::query_as::<_, PageListItem>(
        "SELECT p.id, p.titre, p.resume, p.categorie_id, c.nom as categorie_nom, p.est_publiee, p.date_creation
         FROM page_information p LEFT JOIN categorie_contenu c ON p.categorie_id = c.id
         ORDER BY p.date_creation DESC LIMIT $1 OFFSET $2",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await?;

    let total = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM page_information")
        .fetch_one(pool.get_ref())
        .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": { "pages": pages, "total": total }
    })))
}

#[get("/api/admin/contents/{id}")]
pub async fn get_content(
    pool: web::Data<PgPool>,
    storage: web::Data<S3Storage>,
    _admin: AdminUser,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let page = sqlx::query_as::<_, PageInformation>("SELECT * FROM page_information WHERE id = $1")
        .bind(path.into_inner())
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| AppError::NotFound("Page non trouvée".to_string()))?;

    let contenu = if let Some(ref key) = page.s3_key {
        if let Ok(raw) = storage.download(key).await {
            serde_json::from_str::<serde_json::Value>(&raw)
                .ok()
                .and_then(|v| v.get("contenu").and_then(|c| c.as_str().map(String::from)))
                .unwrap_or(raw)
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": {
            "id": page.id,
            "titre": page.titre,
            "contenu": contenu,
            "resume": page.resume,
            "categorie_id": page.categorie_id,
            "est_publiee": page.est_publiee,
            "s3_key": page.s3_key,
            "date_creation": page.date_creation,
            "date_modification": page.date_modification,
        }
    })))
}

#[post("/api/admin/contents")]
pub async fn create_content(
    pool: web::Data<PgPool>,
    storage: web::Data<S3Storage>,
    admin: AdminUser,
    body: web::Json<CreatePage>,
) -> Result<HttpResponse, AppError> {
    let s3_key = format!("public/articles/{}.json", uuid::Uuid::new_v4());

    let article_json = serde_json::json!({
        "titre": body.titre,
        "resume": body.resume,
        "contenu": body.contenu,
    });

    storage
        .upload(
            &s3_key,
            serde_json::to_vec(&article_json).unwrap(),
            "application/json",
        )
        .await?;

    let page = sqlx::query_as::<_, PageInformation>(
        "INSERT INTO page_information (titre, resume, categorie_id, est_publiee, auteur_id, s3_key)
         VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
    )
    .bind(&body.titre)
    .bind(&body.resume)
    .bind(&body.categorie_id)
    .bind(body.est_publiee.unwrap_or(false))
    .bind(admin.user_id)
    .bind(&s3_key)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "success": true,
        "data": page,
    })))
}

#[put("/api/admin/contents/{id}")]
pub async fn update_content(
    pool: web::Data<PgPool>,
    storage: web::Data<S3Storage>,
    _admin: AdminUser,
    path: web::Path<i32>,
    body: web::Json<CreatePage>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();

    let existing =
        sqlx::query_as::<_, PageInformation>("SELECT * FROM page_information WHERE id = $1")
            .bind(id)
            .fetch_optional(pool.get_ref())
            .await?
            .ok_or_else(|| AppError::NotFound("Page non trouvée".to_string()))?;

    let s3_key = existing
        .s3_key
        .unwrap_or_else(|| format!("public/articles/{}.json", uuid::Uuid::new_v4()));

    let article_json = serde_json::json!({
        "titre": body.titre,
        "resume": body.resume,
        "contenu": body.contenu,
    });

    storage
        .upload(
            &s3_key,
            serde_json::to_vec(&article_json).unwrap(),
            "application/json",
        )
        .await?;

    let page = sqlx::query_as::<_, PageInformation>(
        "UPDATE page_information SET titre = $1, resume = $2, categorie_id = $3,
         est_publiee = $4, s3_key = $5, date_modification = CURRENT_TIMESTAMP
         WHERE id = $6 RETURNING *",
    )
    .bind(&body.titre)
    .bind(&body.resume)
    .bind(&body.categorie_id)
    .bind(body.est_publiee.unwrap_or(false))
    .bind(&s3_key)
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("Page non trouvée".to_string()))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": page,
    })))
}

#[delete("/api/admin/contents/{id}")]
pub async fn delete_content(
    pool: web::Data<PgPool>,
    storage: web::Data<S3Storage>,
    _admin: AdminUser,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();

    let page = sqlx::query_as::<_, PageInformation>("SELECT * FROM page_information WHERE id = $1")
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| AppError::NotFound("Page non trouvée".to_string()))?;

    if let Some(ref key) = page.s3_key {
        let _ = storage.delete(key).await;
    }

    sqlx::query("DELETE FROM page_information WHERE id = $1")
        .bind(id)
        .execute(pool.get_ref())
        .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "Page supprimée",
    })))
}

// --- User Management ---

#[derive(Deserialize)]
pub struct UserQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub search: Option<String>,
    pub est_actif: Option<bool>,
}

const USER_DELETION_GRACE_DAYS: i64 = 30;

#[derive(sqlx::FromRow, serde::Serialize)]
struct AdminUserRow {
    id: i32,
    email: String,
    nom: String,
    prenom: String,
    est_actif: bool,
    role_nom: Option<String>,
    date_creation: chrono::NaiveDateTime,
    date_desactivation: Option<chrono::NaiveDateTime>,
}

#[get("/api/admin/users")]
pub async fn list_users(
    pool: web::Data<PgPool>,
    _admin: AdminUser,
    query: web::Query<UserQuery>,
) -> Result<HttpResponse, AppError> {
    let limit = query.limit.unwrap_or(20).min(50);
    let offset = (query.page.unwrap_or(1) - 1) * limit;

    let users = sqlx::query_as::<_, AdminUserRow>(
        "SELECT u.id, u.email, u.nom, u.prenom, u.est_actif, r.nom as role_nom,
                u.date_creation, u.date_desactivation
         FROM utilisateur u JOIN role r ON u.role_id = r.id
         WHERE ($1::text IS NULL OR u.nom ILIKE '%' || $1 || '%' OR u.prenom ILIKE '%' || $1 || '%' OR u.email ILIKE '%' || $1 || '%')
         AND ($2::bool IS NULL OR u.est_actif = $2)
         ORDER BY u.date_creation DESC LIMIT $3 OFFSET $4",
    )
    .bind(&query.search)
    .bind(query.est_actif)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await?;

    let total = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM utilisateur")
        .fetch_one(pool.get_ref())
        .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": { "users": users, "total": total }
    })))
}

#[get("/api/admin/users/{id}")]
pub async fn get_user(
    pool: web::Data<PgPool>,
    _admin: AdminUser,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let user = sqlx::query_as::<_, AdminUserRow>(
        "SELECT u.id, u.email, u.nom, u.prenom, u.est_actif, r.nom as role_nom,
                u.date_creation, u.date_desactivation
         FROM utilisateur u JOIN role r ON u.role_id = r.id WHERE u.id = $1",
    )
    .bind(path.into_inner())
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("Utilisateur non trouvé".to_string()))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": user,
    })))
}

#[derive(Deserialize)]
pub struct UpdateUser {
    pub email: Option<String>,
    pub nom: Option<String>,
    pub prenom: Option<String>,
    pub role_id: Option<i32>,
}

#[put("/api/admin/users/{id}")]
pub async fn update_user(
    pool: web::Data<PgPool>,
    _admin: AdminUser,
    path: web::Path<i32>,
    body: web::Json<UpdateUser>,
) -> Result<HttpResponse, AppError> {
    sqlx::query(
        "UPDATE utilisateur SET email = COALESCE($1, email), nom = COALESCE($2, nom),
         prenom = COALESCE($3, prenom), role_id = COALESCE($4, role_id),
         date_modification = CURRENT_TIMESTAMP WHERE id = $5",
    )
    .bind(&body.email)
    .bind(&body.nom)
    .bind(&body.prenom)
    .bind(body.role_id)
    .bind(path.into_inner())
    .execute(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "Utilisateur mis à jour",
    })))
}

#[derive(Deserialize)]
pub struct UpdateStatus {
    pub est_actif: bool,
}

#[patch("/api/admin/users/{id}/status")]
pub async fn update_user_status(
    pool: web::Data<PgPool>,
    admin: AdminUser,
    path: web::Path<i32>,
    body: web::Json<UpdateStatus>,
) -> Result<HttpResponse, AppError> {
    let target_id = path.into_inner();

    if !body.est_actif {
        if target_id == admin.user_id {
            return Err(AppError::BadRequest(
                "Vous ne pouvez pas désactiver votre propre compte".to_string(),
            ));
        }

        let other_active_admins = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM utilisateur u JOIN role r ON u.role_id = r.id
             WHERE r.nom = 'administrateur' AND u.id != $1 AND u.est_actif = TRUE",
        )
        .bind(target_id)
        .fetch_one(pool.get_ref())
        .await?;

        let target_is_admin = sqlx::query_scalar::<_, bool>(
            "SELECT r.nom = 'administrateur' FROM utilisateur u JOIN role r ON u.role_id = r.id
             WHERE u.id = $1",
        )
        .bind(target_id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| AppError::NotFound("Utilisateur non trouvé".to_string()))?;

        if target_is_admin && other_active_admins == 0 {
            return Err(AppError::BadRequest(
                "Impossible de désactiver le dernier administrateur".to_string(),
            ));
        }
    }

    let result = sqlx::query(
        "UPDATE utilisateur
         SET est_actif = $1,
             date_desactivation = CASE
                 WHEN $1 = TRUE THEN NULL
                 WHEN est_actif = TRUE AND $1 = FALSE THEN CURRENT_TIMESTAMP
                 ELSE date_desactivation
             END,
             date_modification = CURRENT_TIMESTAMP
         WHERE id = $2",
    )
    .bind(body.est_actif)
    .bind(target_id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Utilisateur non trouvé".to_string()));
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": if body.est_actif { "Utilisateur activé" } else { "Utilisateur désactivé" },
    })))
}

#[delete("/api/admin/users/{id}")]
pub async fn delete_user(
    pool: web::Data<PgPool>,
    admin: AdminUser,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let target_id = path.into_inner();

    if target_id == admin.user_id {
        return Err(AppError::BadRequest(
            "Vous ne pouvez pas supprimer votre propre compte".to_string(),
        ));
    }

    let target = sqlx::query_as::<_, (bool, Option<chrono::NaiveDateTime>, String)>(
        "SELECT u.est_actif, u.date_desactivation, r.nom
         FROM utilisateur u JOIN role r ON u.role_id = r.id WHERE u.id = $1",
    )
    .bind(target_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("Utilisateur non trouvé".to_string()))?;
    let (est_actif, date_desactivation, role_nom) = target;

    if role_nom == "administrateur" {
        let other_admins = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM utilisateur u JOIN role r ON u.role_id = r.id
             WHERE r.nom = 'administrateur' AND u.id != $1",
        )
        .bind(target_id)
        .fetch_one(pool.get_ref())
        .await?;

        if other_admins == 0 {
            return Err(AppError::BadRequest(
                "Impossible de supprimer le dernier administrateur".to_string(),
            ));
        }
    }

    if est_actif {
        return Err(AppError::BadRequest(
            "Le compte doit d'abord être désactivé".to_string(),
        ));
    }

    let deactivated_at = date_desactivation.ok_or_else(|| {
        AppError::BadRequest(format!(
            "Le compte doit être désactivé depuis au moins {USER_DELETION_GRACE_DAYS} jours"
        ))
    })?;
    let elapsed = chrono::Utc::now().naive_utc() - deactivated_at;
    if elapsed < chrono::Duration::days(USER_DELETION_GRACE_DAYS) {
        return Err(AppError::BadRequest(format!(
            "Le compte doit être désactivé depuis au moins {USER_DELETION_GRACE_DAYS} jours"
        )));
    }

    let result = sqlx::query("DELETE FROM utilisateur WHERE id = $1")
        .bind(target_id)
        .execute(pool.get_ref())
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Utilisateur non trouvé".to_string()));
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "Utilisateur supprimé définitivement",
    })))
}

#[derive(Deserialize)]
pub struct CreateAdmin {
    pub email: String,
    pub mot_de_passe: String,
    pub nom: String,
    pub prenom: String,
}

#[post("/api/admin/users")]
pub async fn create_admin_user(
    pool: web::Data<PgPool>,
    _admin: AdminUser,
    body: web::Json<CreateAdmin>,
) -> Result<HttpResponse, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(body.mot_de_passe.as_bytes(), &salt)
        .map_err(|_| AppError::Internal("Erreur de hashage".to_string()))?
        .to_string();

    let user = sqlx::query_as::<_, AdminUserRow>(
        "INSERT INTO utilisateur (email, mot_de_passe_hash, nom, prenom, role_id)
         VALUES ($1, $2, $3, $4, 2) RETURNING id, email, nom, prenom, est_actif, date_creation,
         (SELECT nom FROM role WHERE id = 2) as role_nom",
    )
    .bind(&body.email)
    .bind(&hash)
    .bind(&body.nom)
    .bind(&body.prenom)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "success": true,
        "data": user,
    })))
}

// --- Emotions Management ---

#[get("/api/admin/emotions")]
pub async fn list_admin_emotions(
    pool: web::Data<PgPool>,
    _admin: AdminUser,
) -> Result<HttpResponse, AppError> {
    let emotions = sqlx::query_as::<_, EmotionBase>(
        "SELECT id, nom, emoji, couleur FROM emotion_base ORDER BY id",
    )
    .fetch_all(pool.get_ref())
    .await?;

    let mut result = Vec::new();
    for emotion in emotions {
        let subs = sqlx::query_as::<_, SousEmotion>(
            "SELECT id, nom, emotion_base_id FROM sous_emotion WHERE emotion_base_id = $1 ORDER BY id",
        )
        .bind(emotion.id)
        .fetch_all(pool.get_ref())
        .await?;

        result.push(serde_json::json!({
            "id": emotion.id,
            "nom": emotion.nom,
            "emoji": emotion.emoji,
            "couleur": emotion.couleur,
            "sous_emotions": subs,
        }));
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": result,
    })))
}

#[post("/api/admin/emotions")]
pub async fn create_emotion(
    pool: web::Data<PgPool>,
    _admin: AdminUser,
    body: web::Json<CreateEmotionBase>,
) -> Result<HttpResponse, AppError> {
    let emotion = sqlx::query_as::<_, EmotionBase>(
        "INSERT INTO emotion_base (nom, emoji, couleur) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(&body.nom)
    .bind(&body.emoji)
    .bind(&body.couleur)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "success": true,
        "data": emotion,
    })))
}

#[put("/api/admin/emotions/{id}")]
pub async fn update_emotion(
    pool: web::Data<PgPool>,
    _admin: AdminUser,
    path: web::Path<i32>,
    body: web::Json<CreateEmotionBase>,
) -> Result<HttpResponse, AppError> {
    let emotion = sqlx::query_as::<_, EmotionBase>(
        "UPDATE emotion_base SET nom = $1, emoji = $2, couleur = $3 WHERE id = $4 RETURNING *",
    )
    .bind(&body.nom)
    .bind(&body.emoji)
    .bind(&body.couleur)
    .bind(path.into_inner())
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("Émotion non trouvée".to_string()))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": emotion,
    })))
}

#[delete("/api/admin/emotions/{id}")]
pub async fn delete_emotion(
    pool: web::Data<PgPool>,
    _admin: AdminUser,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();

    let linked = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM entree_tracker et JOIN sous_emotion se ON et.sous_emotion_id = se.id WHERE se.emotion_base_id = $1",
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;

    if linked > 0 {
        return Err(AppError::BadRequest(
            "Impossible de supprimer : des entrées tracker sont liées à cette émotion".to_string(),
        ));
    }

    sqlx::query("DELETE FROM sous_emotion WHERE emotion_base_id = $1")
        .bind(id)
        .execute(pool.get_ref())
        .await?;

    sqlx::query("DELETE FROM emotion_base WHERE id = $1")
        .bind(id)
        .execute(pool.get_ref())
        .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "Émotion supprimée",
    })))
}

#[post("/api/admin/emotions/{id}/sub-emotions")]
pub async fn create_sub_emotion(
    pool: web::Data<PgPool>,
    _admin: AdminUser,
    path: web::Path<i32>,
    body: web::Json<CreateSousEmotion>,
) -> Result<HttpResponse, AppError> {
    let sub = sqlx::query_as::<_, SousEmotion>(
        "INSERT INTO sous_emotion (nom, emotion_base_id) VALUES ($1, $2) RETURNING *",
    )
    .bind(&body.nom)
    .bind(path.into_inner())
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "success": true,
        "data": sub,
    })))
}

#[put("/api/admin/sub-emotions/{id}")]
pub async fn update_sub_emotion(
    pool: web::Data<PgPool>,
    _admin: AdminUser,
    path: web::Path<i32>,
    body: web::Json<CreateSousEmotion>,
) -> Result<HttpResponse, AppError> {
    let sub = sqlx::query_as::<_, SousEmotion>(
        "UPDATE sous_emotion SET nom = $1 WHERE id = $2 RETURNING *",
    )
    .bind(&body.nom)
    .bind(path.into_inner())
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("Sous-émotion non trouvée".to_string()))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": sub,
    })))
}

#[delete("/api/admin/sub-emotions/{id}")]
pub async fn delete_sub_emotion(
    pool: web::Data<PgPool>,
    _admin: AdminUser,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let result = sqlx::query("DELETE FROM sous_emotion WHERE id = $1")
        .bind(path.into_inner())
        .execute(pool.get_ref())
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Sous-émotion non trouvée".to_string()));
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "Sous-émotion supprimée",
    })))
}

// --- Dashboard Stats ---

#[get("/api/admin/stats")]
pub async fn get_admin_stats(
    pool: web::Data<PgPool>,
    _admin: AdminUser,
) -> Result<HttpResponse, AppError> {
    let total_users = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM utilisateur")
        .fetch_one(pool.get_ref())
        .await?;
    let total_entries = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM entree_tracker")
        .fetch_one(pool.get_ref())
        .await?;
    let total_pages = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM page_information")
        .fetch_one(pool.get_ref())
        .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": {
            "total_users": total_users,
            "total_entries": total_entries,
            "total_pages": total_pages,
        }
    })))
}
