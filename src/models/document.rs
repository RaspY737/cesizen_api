use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Document {
    pub id: i32,
    pub nom: String,
    pub nom_fichier: String,
    pub taille: i64,
    pub content_type: String,
    pub s3_key: String,
    pub uploaded_by: Option<i32>,
    pub date_creation: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct DocumentResponse {
    pub id: i32,
    pub nom: String,
    pub nom_fichier: String,
    pub taille: i64,
    pub content_type: String,
    pub url: String,
    pub date_creation: NaiveDateTime,
}
