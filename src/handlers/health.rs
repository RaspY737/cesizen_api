use actix_web::{HttpResponse, get};
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    success: bool,
    message: String,
}

#[get("/api/health")]
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse {
        success: true,
        message: "API CESIZen opérationnelle".to_string(),
    })
}
