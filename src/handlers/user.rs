use actix_web::{HttpResponse, get, put, web};
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::errors::AppError;
use crate::middleware::auth::AuthUser;

#[derive(Serialize)]
struct ApiResponse<T: Serialize> {
    success: bool,
    data: T,
}

#[derive(Serialize, sqlx::FromRow)]
struct UserProfile {
    id: i32,
    email: String,
    nom: String,
    prenom: String,
    date_naissance: Option<chrono::NaiveDate>,
    role_nom: Option<String>,
    date_creation: chrono::NaiveDateTime,
}

#[get("/api/users/me")]
pub async fn get_profile(
    pool: web::Data<PgPool>,
    auth: AuthUser,
) -> Result<HttpResponse, AppError> {
    let profile = sqlx::query_as::<_, UserProfile>(
        "SELECT u.id, u.email, u.nom, u.prenom, u.date_naissance, r.nom as role_nom, u.date_creation
         FROM utilisateur u JOIN role r ON u.role_id = r.id WHERE u.id = $1"
    )
    .bind(auth.user_id)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: serde_json::json!({
            "id": profile.id,
            "email": profile.email,
            "nom": profile.nom,
            "prenom": profile.prenom,
            "date_naissance": profile.date_naissance,
            "role": profile.role_nom,
            "date_creation": profile.date_creation,
        }),
    }))
}

#[derive(Deserialize)]
pub struct UpdateProfile {
    pub nom: Option<String>,
    pub prenom: Option<String>,
    pub date_naissance: Option<chrono::NaiveDate>,
}

#[put("/api/users/me")]
pub async fn update_profile(
    pool: web::Data<PgPool>,
    auth: AuthUser,
    body: web::Json<UpdateProfile>,
) -> Result<HttpResponse, AppError> {
    sqlx::query(
        "UPDATE utilisateur SET nom = COALESCE($1, nom), prenom = COALESCE($2, prenom),
         date_naissance = COALESCE($3, date_naissance), date_modification = CURRENT_TIMESTAMP WHERE id = $4"
    )
    .bind(&body.nom)
    .bind(&body.prenom)
    .bind(&body.date_naissance)
    .bind(auth.user_id)
    .execute(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "Profil mis à jour"
    })))
}

#[derive(Deserialize)]
pub struct ChangePassword {
    pub ancien_mot_de_passe: String,
    pub nouveau_mot_de_passe: String,
}

#[put("/api/users/me/password")]
pub async fn change_password(
    pool: web::Data<PgPool>,
    auth: AuthUser,
    body: web::Json<ChangePassword>,
) -> Result<HttpResponse, AppError> {
    let current_hash =
        sqlx::query_scalar::<_, String>("SELECT mot_de_passe_hash FROM utilisateur WHERE id = $1")
            .bind(auth.user_id)
            .fetch_one(pool.get_ref())
            .await?;

    let parsed_hash = PasswordHash::new(&current_hash)
        .map_err(|_| AppError::Internal("Erreur de vérification".to_string()))?;

    Argon2::default()
        .verify_password(body.ancien_mot_de_passe.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::BadRequest("Ancien mot de passe incorrect".to_string()))?;

    let salt = SaltString::generate(&mut OsRng);
    let new_hash = Argon2::default()
        .hash_password(body.nouveau_mot_de_passe.as_bytes(), &salt)
        .map_err(|_| AppError::Internal("Erreur de hashage".to_string()))?
        .to_string();

    sqlx::query("UPDATE utilisateur SET mot_de_passe_hash = $1, date_modification = CURRENT_TIMESTAMP WHERE id = $2")
        .bind(&new_hash)
        .bind(auth.user_id)
        .execute(pool.get_ref())
        .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "Mot de passe modifié"
    })))
}
