use actix_multipart::Multipart;
use actix_web::{HttpResponse, delete, get, post, web};
use futures_util::StreamExt;
use serde::Deserialize;
use sqlx::PgPool;

use crate::errors::AppError;
use crate::middleware::auth::AdminUser;
use crate::models::document::{Document, DocumentResponse};
use crate::storage::S3Storage;

const MAX_FILE_SIZE: usize = 10 * 1024 * 1024; // 10 MB

#[derive(Deserialize)]
pub struct DocumentQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[get("/api/documents")]
pub async fn list_documents(
    pool: web::Data<PgPool>,
    storage: web::Data<S3Storage>,
    query: web::Query<DocumentQuery>,
) -> Result<HttpResponse, AppError> {
    let limit = query.limit.unwrap_or(20).min(50);
    let offset = (query.page.unwrap_or(1) - 1) * limit;

    let documents = sqlx::query_as::<_, Document>(
        "SELECT id, nom, nom_fichier, taille, content_type, s3_key, uploaded_by, date_creation
         FROM document ORDER BY date_creation DESC LIMIT $1 OFFSET $2",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await?;

    let total = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM document")
        .fetch_one(pool.get_ref())
        .await?;

    let responses: Vec<DocumentResponse> = documents
        .into_iter()
        .map(|d| DocumentResponse {
            id: d.id,
            nom: d.nom,
            nom_fichier: d.nom_fichier,
            taille: d.taille,
            content_type: d.content_type,
            url: storage.public_url(&d.s3_key),
            date_creation: d.date_creation,
        })
        .collect();

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": { "documents": responses, "total": total }
    })))
}

#[get("/api/documents/{id}")]
pub async fn get_document(
    pool: web::Data<PgPool>,
    storage: web::Data<S3Storage>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let doc = sqlx::query_as::<_, Document>("SELECT * FROM document WHERE id = $1")
        .bind(path.into_inner())
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| AppError::NotFound("Document non trouvé".to_string()))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": DocumentResponse {
            id: doc.id,
            nom: doc.nom,
            nom_fichier: doc.nom_fichier,
            taille: doc.taille,
            content_type: doc.content_type,
            url: storage.public_url(&doc.s3_key),
            date_creation: doc.date_creation,
        }
    })))
}

#[post("/api/admin/documents")]
pub async fn upload_document(
    pool: web::Data<PgPool>,
    storage: web::Data<S3Storage>,
    admin: AdminUser,
    mut payload: Multipart,
) -> Result<HttpResponse, AppError> {
    let mut file_data: Option<Vec<u8>> = None;
    let mut file_name: Option<String> = None;
    let mut content_type: Option<String> = None;
    let mut nom: Option<String> = None;

    while let Some(Ok(mut field)) = payload.next().await {
        let cd = field.content_disposition().cloned();
        let field_name = cd
            .as_ref()
            .and_then(|cd| cd.get_name().map(|s| s.to_string()))
            .unwrap_or_default();

        match field_name.as_str() {
            "file" => {
                file_name = cd
                    .as_ref()
                    .and_then(|cd| cd.get_filename().map(|s| s.to_string()));
                content_type = field.content_type().map(|ct| ct.to_string());

                let mut data = Vec::new();
                while let Some(Ok(chunk)) = field.next().await {
                    data.extend_from_slice(&chunk);
                    if data.len() > MAX_FILE_SIZE {
                        return Err(AppError::BadRequest(format!(
                            "Fichier trop volumineux (max {} Mo)",
                            MAX_FILE_SIZE / 1024 / 1024
                        )));
                    }
                }
                file_data = Some(data);
            }
            "nom" => {
                let mut data = Vec::new();
                while let Some(Ok(chunk)) = field.next().await {
                    data.extend_from_slice(&chunk);
                }
                nom = Some(String::from_utf8_lossy(&data).to_string());
            }
            _ => {}
        }
    }

    let file_data = file_data.ok_or_else(|| AppError::BadRequest("Fichier requis".to_string()))?;
    let file_name =
        file_name.ok_or_else(|| AppError::BadRequest("Nom de fichier requis".to_string()))?;
    let content_type = content_type.unwrap_or_else(|| "application/octet-stream".to_string());
    let nom = nom.unwrap_or_else(|| file_name.clone());

    let s3_key = format!(
        "public/{}/{}",
        uuid::Uuid::new_v4(),
        sanitize_filename(&file_name)
    );
    let taille = file_data.len() as i64;

    storage.upload(&s3_key, file_data, &content_type).await?;

    let doc = sqlx::query_as::<_, Document>(
        "INSERT INTO document (nom, nom_fichier, taille, content_type, s3_key, uploaded_by)
         VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
    )
    .bind(&nom)
    .bind(&file_name)
    .bind(taille)
    .bind(&content_type)
    .bind(&s3_key)
    .bind(admin.user_id)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "success": true,
        "data": DocumentResponse {
            id: doc.id,
            nom: doc.nom,
            nom_fichier: doc.nom_fichier,
            taille: doc.taille,
            content_type: doc.content_type,
            url: storage.public_url(&doc.s3_key),
            date_creation: doc.date_creation,
        }
    })))
}

#[delete("/api/admin/documents/{id}")]
pub async fn delete_document(
    pool: web::Data<PgPool>,
    storage: web::Data<S3Storage>,
    _admin: AdminUser,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let doc = sqlx::query_as::<_, Document>("SELECT * FROM document WHERE id = $1")
        .bind(path.into_inner())
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| AppError::NotFound("Document non trouvé".to_string()))?;

    storage.delete(&doc.s3_key).await?;

    sqlx::query("DELETE FROM document WHERE id = $1")
        .bind(doc.id)
        .execute(pool.get_ref())
        .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "Document supprimé"
    })))
}

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '.' || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}
