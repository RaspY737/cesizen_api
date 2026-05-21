use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use cesizen_api::{config, configure_app, db, storage};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();

    let cfg = config::Config::from_env();
    let pool = db::create_pool(&cfg.database_url).await;
    let s3 = storage::S3Storage::new(&cfg.s3).await;

    log::info!("Starting server on {}:{}", cfg.host, cfg.port);

    let jwt_secret = cfg.jwt_secret.clone();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::CONTENT_TYPE,
            ])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(jwt_secret.clone()))
            .app_data(web::Data::new(s3.clone()))
            .configure(configure_app)
    })
    .bind((cfg.host, cfg.port))?
    .run()
    .await
}
