use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct EmotionBase {
    pub id: i32,
    pub nom: String,
    pub emoji: Option<String>,
    pub couleur: Option<String>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct SousEmotion {
    pub id: i32,
    pub nom: String,
    pub emotion_base_id: i32,
}

#[derive(Debug, Serialize)]
pub struct EmotionWithSubs {
    #[serde(flatten)]
    pub base: EmotionBase,
    pub sous_emotions: Vec<SousEmotion>,
}

#[derive(Debug, Deserialize)]
pub struct CreateEmotionBase {
    pub nom: String,
    pub emoji: Option<String>,
    pub couleur: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateSousEmotion {
    pub nom: String,
}
