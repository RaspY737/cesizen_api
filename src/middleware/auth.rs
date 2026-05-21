use actix_web::{FromRequest, HttpRequest, dev::Payload, web};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::future::{Future, Ready, ready};

use crate::errors::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub role: String,
    pub exp: usize,
}

pub struct AuthUser {
    pub user_id: i32,
    pub role: String,
}

impl FromRequest for AuthUser {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let result = extract_auth(req);
        ready(result)
    }
}

fn extract_auth(req: &HttpRequest) -> Result<AuthUser, AppError> {
    let jwt_secret = req
        .app_data::<web::Data<String>>()
        .ok_or_else(|| AppError::Internal("JWT secret not configured".to_string()))?;

    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| AppError::Unauthorized("Token manquant".to_string()))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| AppError::Unauthorized("Format de token invalide".to_string()))?;

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| AppError::Unauthorized("Token invalide ou expiré".to_string()))?;

    Ok(AuthUser {
        user_id: token_data.claims.sub,
        role: token_data.claims.role,
    })
}

pub fn create_token(user_id: i32, role: &str, secret: &str) -> Result<String, AppError> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .ok_or_else(|| AppError::Internal("Erreur de calcul de date".to_string()))?
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        role: role.to_string(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| AppError::Internal("Erreur de création du token".to_string()))
}

pub struct AdminUser {
    pub user_id: i32,
}

impl FromRequest for AdminUser {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let auth_result = extract_auth(req);
        ready(match auth_result {
            Ok(auth) if auth.role == "administrateur" => Ok(AdminUser {
                user_id: auth.user_id,
            }),
            Ok(_) => Err(AppError::Forbidden(
                "Accès réservé aux administrateurs".to_string(),
            )),
            Err(e) => Err(e),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::{DecodingKey, Validation, decode};

    const TEST_SECRET: &str = "test-secret-key-for-unit-tests";

    #[test]
    fn test_create_token_returns_valid_jwt() {
        let token = create_token(42, "utilisateur", TEST_SECRET).unwrap();
        assert!(!token.is_empty());

        // Decode and verify claims
        let token_data = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(TEST_SECRET.as_bytes()),
            &Validation::default(),
        )
        .unwrap();

        assert_eq!(token_data.claims.sub, 42);
        assert_eq!(token_data.claims.role, "utilisateur");
    }

    #[test]
    fn test_create_token_admin_role() {
        let token = create_token(1, "administrateur", TEST_SECRET).unwrap();
        let token_data = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(TEST_SECRET.as_bytes()),
            &Validation::default(),
        )
        .unwrap();

        assert_eq!(token_data.claims.sub, 1);
        assert_eq!(token_data.claims.role, "administrateur");
    }

    #[test]
    fn test_create_token_has_future_expiration() {
        let token = create_token(1, "utilisateur", TEST_SECRET).unwrap();
        let token_data = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(TEST_SECRET.as_bytes()),
            &Validation::default(),
        )
        .unwrap();

        let now = chrono::Utc::now().timestamp() as usize;
        // Token should expire at least 23 hours from now
        assert!(token_data.claims.exp > now + 23 * 3600);
    }

    #[test]
    fn test_decode_token_with_wrong_secret_fails() {
        let token = create_token(1, "utilisateur", TEST_SECRET).unwrap();
        let result = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(b"wrong-secret"),
            &Validation::default(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_decode_garbage_token_fails() {
        let result = decode::<Claims>(
            "not.a.valid.jwt",
            &DecodingKey::from_secret(TEST_SECRET.as_bytes()),
            &Validation::default(),
        );
        assert!(result.is_err());
    }
}
