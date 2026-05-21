use chrono::NaiveDate;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    #[serde(skip_serializing)]
    pub mot_de_passe_hash: String,
    pub nom: String,
    pub prenom: String,
    pub date_naissance: Option<NaiveDate>,
    pub est_actif: bool,
    pub role_id: i32,
    pub date_creation: NaiveDateTime,
    pub date_modification: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub email: String,
    pub nom: String,
    pub prenom: String,
    pub date_naissance: Option<NaiveDate>,
    pub est_actif: bool,
    pub role: String,
    pub date_creation: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub mot_de_passe: String,
    pub nom: String,
    pub prenom: String,
    pub date_naissance: Option<NaiveDate>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub mot_de_passe: String,
}
