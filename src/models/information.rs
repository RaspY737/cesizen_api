use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PageInformation {
    pub id: i32,
    pub titre: String,
    pub resume: Option<String>,
    pub categorie_id: Option<i32>,
    pub est_publiee: bool,
    pub auteur_id: i32,
    pub s3_key: Option<String>,
    pub date_creation: NaiveDateTime,
    pub date_modification: NaiveDateTime,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PageListItem {
    pub id: i32,
    pub titre: String,
    pub resume: Option<String>,
    pub categorie_id: Option<i32>,
    pub categorie_nom: Option<String>,
    pub est_publiee: bool,
    pub date_creation: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreatePage {
    pub titre: String,
    pub contenu: String,
    pub resume: Option<String>,
    pub categorie_id: Option<i32>,
    pub est_publiee: Option<bool>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct CategorieContenu {
    pub id: i32,
    pub nom: String,
    pub description: Option<String>,
    pub ordre_affichage: i32,
    pub est_active: bool,
}
