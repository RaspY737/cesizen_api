use actix_web::{HttpResponse, post, web};
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use serde::Serialize;
use sqlx::PgPool;

use crate::errors::AppError;
use crate::middleware::auth::create_token;
use crate::models::user::{LoginRequest, RegisterRequest, User};

#[derive(Serialize)]
struct AuthResponse {
    success: bool,
    data: AuthData,
}

#[derive(Serialize)]
struct AuthData {
    token: String,
    user: UserInfo,
}

#[derive(Serialize)]
struct UserInfo {
    id: i32,
    email: String,
    nom: String,
    prenom: String,
    role: String,
}

#[post("/register")]
pub async fn register(
    pool: web::Data<PgPool>,
    jwt_secret: web::Data<String>,
    body: web::Json<RegisterRequest>,
) -> Result<HttpResponse, AppError> {
    if body.email.is_empty()
        || body.mot_de_passe.is_empty()
        || body.nom.is_empty()
        || body.prenom.is_empty()
    {
        return Err(AppError::BadRequest(
            "Tous les champs obligatoires doivent être remplis".to_string(),
        ));
    }

    if !body.email.contains('@') || !body.email.contains('.') || body.email.len() < 5 {
        return Err(AppError::BadRequest("Format d'email invalide".to_string()));
    }

    if body.mot_de_passe.len() < 8 {
        return Err(AppError::BadRequest(
            "Le mot de passe doit contenir au moins 8 caractères".to_string(),
        ));
    }

    if !body.mot_de_passe.chars().any(|c| c.is_uppercase())
        || !body.mot_de_passe.chars().any(|c| c.is_lowercase())
        || !body.mot_de_passe.chars().any(|c| c.is_numeric())
    {
        return Err(AppError::BadRequest(
            "Le mot de passe doit contenir au moins une majuscule, une minuscule et un chiffre"
                .to_string(),
        ));
    }

    let existing = sqlx::query_scalar::<_, i32>("SELECT id FROM utilisateur WHERE email = $1")
        .bind(&body.email)
        .fetch_optional(pool.get_ref())
        .await?;

    if existing.is_some() {
        return Err(AppError::BadRequest(
            "Un compte avec cet email existe déjà".to_string(),
        ));
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(body.mot_de_passe.as_bytes(), &salt)
        .map_err(|_| AppError::Internal("Erreur de hashage du mot de passe".to_string()))?
        .to_string();

    let user = sqlx::query_as::<_, User>(
        "INSERT INTO utilisateur (email, mot_de_passe_hash, nom, prenom, date_naissance) VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(&body.email)
    .bind(&password_hash)
    .bind(&body.nom)
    .bind(&body.prenom)
    .bind(&body.date_naissance)
    .fetch_one(pool.get_ref())
    .await?;

    let role_nom = sqlx::query_scalar::<_, String>("SELECT nom FROM role WHERE id = $1")
        .bind(user.role_id)
        .fetch_one(pool.get_ref())
        .await?;

    let token = create_token(user.id, &role_nom, &jwt_secret)?;

    Ok(HttpResponse::Created().json(AuthResponse {
        success: true,
        data: AuthData {
            token,
            user: UserInfo {
                id: user.id,
                email: user.email,
                nom: user.nom,
                prenom: user.prenom,
                role: role_nom,
            },
        },
    }))
}

#[post("/login")]
pub async fn login(
    pool: web::Data<PgPool>,
    jwt_secret: web::Data<String>,
    body: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM utilisateur WHERE email = $1 AND est_actif = TRUE",
    )
    .bind(&body.email)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| AppError::Unauthorized("Email ou mot de passe incorrect".to_string()))?;

    let parsed_hash = PasswordHash::new(&user.mot_de_passe_hash)
        .map_err(|_| AppError::Internal("Erreur de vérification du mot de passe".to_string()))?;

    Argon2::default()
        .verify_password(body.mot_de_passe.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::Unauthorized("Email ou mot de passe incorrect".to_string()))?;

    let role_nom = sqlx::query_scalar::<_, String>("SELECT nom FROM role WHERE id = $1")
        .bind(user.role_id)
        .fetch_one(pool.get_ref())
        .await?;

    let token = create_token(user.id, &role_nom, &jwt_secret)?;

    Ok(HttpResponse::Ok().json(AuthResponse {
        success: true,
        data: AuthData {
            token,
            user: UserInfo {
                id: user.id,
                email: user.email,
                nom: user.nom,
                prenom: user.prenom,
                role: role_nom,
            },
        },
    }))
}

#[cfg(test)]
mod tests {
    use argon2::password_hash::SaltString;
    use argon2::password_hash::rand_core::OsRng;
    use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

    #[test]
    fn test_argon2_hash_and_verify_succeeds() {
        let password = "MonMotDePasse123";
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        // Verify with correct password
        let parsed = PasswordHash::new(&hash).unwrap();
        assert!(argon2.verify_password(password.as_bytes(), &parsed).is_ok());
    }

    #[test]
    fn test_argon2_verify_wrong_password_fails() {
        let password = "CorrectPassword";
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        let parsed = PasswordHash::new(&hash).unwrap();
        assert!(argon2.verify_password(b"WrongPassword", &parsed).is_err());
    }

    #[test]
    fn test_argon2_hash_format_is_valid() {
        let salt = SaltString::generate(&mut OsRng);
        let hash = Argon2::default()
            .hash_password(b"test", &salt)
            .unwrap()
            .to_string();

        // Must start with argon2 identifier
        assert!(hash.starts_with("$argon2"));
        // Must be parseable
        assert!(PasswordHash::new(&hash).is_ok());
    }

    #[test]
    fn test_seed_admin_hash_is_invalid_for_admin123() {
        // This test documents that the seed hash does NOT match "Admin123!"
        // It was written manually and is not a real hash for that password.
        let seed_hash = "$argon2id$v=19$m=19456,t=2,p=1$dGVzdHNhbHQ$VnTRSJMSBE9Cnl/TgJo8GdPp3VPJklGBkPHSO1cAews";
        let parsed = PasswordHash::new(seed_hash).unwrap();
        let result = Argon2::default().verify_password(b"Admin123!", &parsed);
        // This SHOULD fail — the hash was not generated from "Admin123!"
        assert!(result.is_err(), "Seed hash unexpectedly matched Admin123!");
    }

    #[test]
    fn test_generate_valid_admin_hash() {
        // Generate a real hash for "Admin123!" and verify it works
        let password = "Admin123!";
        let salt = SaltString::generate(&mut OsRng);
        let hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        let parsed = PasswordHash::new(&hash).unwrap();
        assert!(
            Argon2::default()
                .verify_password(password.as_bytes(), &parsed)
                .is_ok()
        );

        // Print hash so we can use it in seed data
        println!("Valid Admin123! hash: {hash}");
    }
}
