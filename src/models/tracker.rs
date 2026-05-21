use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct EntreeTracker {
    pub id: i32,
    pub utilisateur_id: i32,
    pub sous_emotion_id: i32,
    pub intensite: i32,
    pub note: Option<String>,
    pub date_entree: NaiveDateTime,
    pub date_modification: NaiveDateTime,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct EntreeListItem {
    pub id: i32,
    pub sous_emotion_id: i32,
    pub sous_emotion_nom: Option<String>,
    pub emotion_base_nom: Option<String>,
    pub emotion_base_emoji: Option<String>,
    pub intensite: i32,
    pub note: Option<String>,
    pub date_entree: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateEntree {
    pub sous_emotion_id: i32,
    pub intensite: i32,
    pub note: Option<String>,
    pub date_entree: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct EmotionStat {
    pub emotion_base_nom: Option<String>,
    pub emotion_base_emoji: Option<String>,
    pub count: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct TrackerStats {
    pub total_entrees: i64,
    pub entrees_semaine: i64,
    pub intensite_moyenne: f64,
    pub emotion_dominante: Option<String>,
}
