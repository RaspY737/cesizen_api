pub mod config;
pub mod db;
pub mod errors;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod storage;

use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::web;

pub fn configure_app(cfg: &mut web::ServiceConfig) {
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(2)
        .burst_size(5)
        .finish()
        .unwrap();

    cfg
        // Auth (rate limited: 2 req/s, burst 5)
        .service(
            web::scope("/api/auth")
                .wrap(Governor::new(&governor_conf))
                .service(handlers::auth::register)
                .service(handlers::auth::login),
        )
        .service(handlers::health::health_check)
        .service(handlers::user::get_profile)
        .service(handlers::user::update_profile)
        .service(handlers::user::change_password)
        // Information (public)
        .service(handlers::information::list_pages)
        .service(handlers::information::get_page)
        .service(handlers::information::list_categories)
        // Admin - Contents
        .service(handlers::admin::list_contents)
        .service(handlers::admin::get_content)
        .service(handlers::admin::create_content)
        .service(handlers::admin::update_content)
        .service(handlers::admin::delete_content)
        // Emotions (public)
        .service(handlers::emotion::list_emotions)
        .service(handlers::emotion::list_sub_emotions)
        // Tracker (authenticated)
        .service(handlers::tracker::list_entries)
        .service(handlers::tracker::get_entry)
        .service(handlers::tracker::create_entry)
        .service(handlers::tracker::update_entry)
        .service(handlers::tracker::delete_entry)
        .service(handlers::tracker::get_stats)
        .service(handlers::tracker::get_distribution)
        // Admin - Users
        .service(handlers::admin::list_users)
        .service(handlers::admin::get_user)
        .service(handlers::admin::update_user)
        .service(handlers::admin::update_user_status)
        .service(handlers::admin::delete_user)
        .service(handlers::admin::create_admin_user)
        // Admin - Emotions
        .service(handlers::admin::list_admin_emotions)
        .service(handlers::admin::create_emotion)
        .service(handlers::admin::update_emotion)
        .service(handlers::admin::delete_emotion)
        .service(handlers::admin::create_sub_emotion)
        .service(handlers::admin::update_sub_emotion)
        .service(handlers::admin::delete_sub_emotion)
        // Admin - Stats
        .service(handlers::admin::get_admin_stats)
        // Documents (public read)
        .service(handlers::document::list_documents)
        .service(handlers::document::get_document)
        // Admin - Documents
        .service(handlers::document::upload_document)
        .service(handlers::document::delete_document);
}
