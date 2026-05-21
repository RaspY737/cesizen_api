use actix_web::{HttpResponse, get, web};
use serde::Deserialize;
use sqlx::PgPool;

use crate::errors::AppError;
use crate::models::information::{CategorieContenu, PageListItem};
use crate::storage::S3Storage;

#[derive(Deserialize)]
pub struct PageQuery {
    pub categorie_id: Option<i32>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[get("/api/information/pages")]
pub async fn list_pages(
    pool: web::Data<PgPool>,
    query: web::Query<PageQuery>,
) -> Result<HttpResponse, AppError> {
    let limit = query.limit.unwrap_or(10).min(50);
    let offset = (query.page.unwrap_or(1) - 1) * limit;

    let pages = if let Some(cat_id) = query.categorie_id {
        sqlx::query_as::<_, PageListItem>(
            "SELECT p.id, p.titre, p.resume, p.categorie_id, c.nom as categorie_nom, p.est_publiee, p.date_creation
             FROM page_information p LEFT JOIN categorie_contenu c ON p.categorie_id = c.id
             WHERE p.est_publiee = TRUE AND p.categorie_id = $1
             ORDER BY p.date_creation DESC LIMIT $2 OFFSET $3",
        )
        .bind(cat_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool.get_ref())
        .await?
    } else {
        sqlx::query_as::<_, PageListItem>(
            "SELECT p.id, p.titre, p.resume, p.categorie_id, c.nom as categorie_nom, p.est_publiee, p.date_creation
             FROM page_information p LEFT JOIN categorie_contenu c ON p.categorie_id = c.id
             WHERE p.est_publiee = TRUE
             ORDER BY p.date_creation DESC LIMIT $1 OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool.get_ref())
        .await?
    };

    let total = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM page_information WHERE est_publiee = TRUE",
    )
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": {
            "pages": pages,
            "total": total,
            "page": query.page.unwrap_or(1),
            "limit": limit,
        }
    })))
}

#[get("/api/information/pages/{id}")]
pub async fn get_page(
    pool: web::Data<PgPool>,
    storage: web::Data<S3Storage>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();

    let page = sqlx::query_as::<_, PageListItem>(
        "SELECT p.id, p.titre, p.resume, p.categorie_id, c.nom as categorie_nom, p.est_publiee, p.date_creation
         FROM page_information p LEFT JOIN categorie_contenu c ON p.categorie_id = c.id
         WHERE p.id = $1 AND p.est_publiee = TRUE",
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("Page non trouvée".to_string()))?;

    let s3_key = sqlx::query_scalar::<_, Option<String>>(
        "SELECT s3_key FROM page_information WHERE id = $1",
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;

    let contenu = if let Some(key) = s3_key {
        if let Ok(raw) = storage.download(&key).await {
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
            "categorie_nom": page.categorie_nom,
            "date_creation": page.date_creation,
        }
    })))
}

#[get("/api/information/categories")]
pub async fn list_categories(pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    let categories = sqlx::query_as::<_, CategorieContenu>(
        "SELECT * FROM categorie_contenu WHERE est_active = TRUE ORDER BY ordre_affichage",
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": categories,
    })))
}
