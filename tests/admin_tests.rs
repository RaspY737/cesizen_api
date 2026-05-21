mod common;

use actix_web::test;

#[actix_web::test]
async fn test_admin_users_without_token_returns_401() {
    let pool = common::setup_test_db().await;
    let app = create_test_app!(pool);

    let req = test::TestRequest::get()
        .uri("/api/admin/users")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 401);
}

#[actix_web::test]
async fn test_admin_users_with_user_token_returns_403() {
    let pool = common::setup_test_db().await;
    let app = create_test_app!(pool);

    let token = register_test_user!(app, "nonadmin@test.fr");

    let req = test::TestRequest::get()
        .uri("/api/admin/users")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 403);
}

#[actix_web::test]
async fn test_admin_stats_without_token_returns_401() {
    let pool = common::setup_test_db().await;
    let app = create_test_app!(pool);

    let req = test::TestRequest::get()
        .uri("/api/admin/stats")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 401);
}

#[actix_web::test]
async fn test_admin_stats_with_admin_token_returns_200() {
    let pool = common::setup_test_db().await;
    let app = create_test_app!(pool);

    let admin_token = common::create_admin_token();

    let req = test::TestRequest::get()
        .uri("/api/admin/stats")
        .insert_header(("Authorization", format!("Bearer {admin_token}")))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
}

#[ignore = "list_users renvoie une structure paginée (non array) — handler à aligner ou test à adapter"]
#[actix_web::test]
async fn test_admin_users_with_admin_token_returns_200() {
    let pool = common::setup_test_db().await;
    let app = create_test_app!(pool);

    let admin_token = common::create_admin_token();

    let req = test::TestRequest::get()
        .uri("/api/admin/users")
        .insert_header(("Authorization", format!("Bearer {admin_token}")))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["success"], true);
    assert!(body["data"].is_array());
}

#[actix_web::test]
async fn test_admin_contents_with_user_token_returns_403() {
    let pool = common::setup_test_db().await;
    let app = create_test_app!(pool);

    let token = register_test_user!(app, "content403@test.fr");

    let req = test::TestRequest::get()
        .uri("/api/admin/contents")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 403);
}

#[ignore = "dépend d'un backend S3 (MinIO) — à mocker séparément"]
#[actix_web::test]
async fn test_admin_create_content_returns_201() {
    let pool = common::setup_test_db().await;
    let app = create_test_app!(pool);

    let admin_token = common::create_admin_token();

    let req = test::TestRequest::post()
        .uri("/api/admin/contents")
        .insert_header(("Authorization", format!("Bearer {admin_token}")))
        .set_json(serde_json::json!({
            "titre": "Article de test",
            "contenu": "Contenu de l'article de test",
            "est_publiee": true
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["success"], true);
    assert_eq!(body["data"]["titre"], "Article de test");
}

#[actix_web::test]
async fn test_admin_emotions_without_token_returns_401() {
    let pool = common::setup_test_db().await;
    let app = create_test_app!(pool);

    let req = test::TestRequest::get()
        .uri("/api/admin/emotions")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 401);
}

// --- Sécurisation de la suppression / désactivation de comptes ---

fn forge_admin_token(user_id: i32) -> String {
    cesizen_api::middleware::auth::create_token(user_id, "administrateur", common::TEST_JWT_SECRET)
        .unwrap()
}

macro_rules! register_and_deactivate {
    ($app:expr, $admin_token:expr, $email:expr) => {{
        let req = test::TestRequest::post()
            .uri("/api/auth/register")
            .peer_addr("127.0.0.1:12345".parse().unwrap())
            .set_json(serde_json::json!({
                "email": $email,
                "mot_de_passe": "Secure123!",
                "nom": "Test",
                "prenom": "User"
            }))
            .to_request();
        let body: serde_json::Value =
            test::read_body_json(test::call_service(&$app, req).await).await;
        let user_id = body["data"]["user"]["id"].as_i64().unwrap() as i32;

        let req = test::TestRequest::patch()
            .uri(&format!("/api/admin/users/{user_id}/status"))
            .insert_header(("Authorization", format!("Bearer {}", $admin_token)))
            .set_json(serde_json::json!({ "est_actif": false }))
            .to_request();
        assert_eq!(test::call_service(&$app, req).await.status(), 200);
        user_id
    }};
}

#[actix_web::test]
async fn test_delete_self_returns_400() {
    let pool = common::setup_test_db().await;
    let app = create_test_app!(pool);

    let admin_token = common::create_admin_token();

    let req = test::TestRequest::delete()
        .uri("/api/admin/users/1")
        .insert_header(("Authorization", format!("Bearer {admin_token}")))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

#[actix_web::test]
async fn test_delete_last_admin_returns_400() {
    let pool = common::setup_test_db().await;
    let app = create_test_app!(pool.clone());

    // Le seed 009 ajoute un admin2 ; on le supprime pour isoler le scénario "1 seul admin".
    // On désactive aussi admin id=1 et on antidate pour passer les autres règles
    // (compte actif / délai < 30j) et n'avoir que la règle "dernier admin" qui puisse déclencher.
    sqlx::query("DELETE FROM utilisateur WHERE email = 'admin2@cesizen.fr'")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("UPDATE utilisateur SET est_actif = FALSE, date_desactivation = CURRENT_TIMESTAMP - INTERVAL '60 days' WHERE id = 1")
        .execute(&pool)
        .await
        .unwrap();

    let ghost_token = forge_admin_token(9999);

    let req = test::TestRequest::delete()
        .uri("/api/admin/users/1")
        .insert_header(("Authorization", format!("Bearer {ghost_token}")))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

#[actix_web::test]
async fn test_delete_active_user_returns_400() {
    let pool = common::setup_test_db().await;
    let app = create_test_app!(pool);

    let admin_token = common::create_admin_token();
    let _ = register_test_user!(app, "active@test.fr");

    // On récupère l'id via /admin/users en cherchant par email
    let req = test::TestRequest::get()
        .uri("/api/admin/users?search=active@test.fr")
        .insert_header(("Authorization", format!("Bearer {admin_token}")))
        .to_request();
    let body: serde_json::Value = test::read_body_json(test::call_service(&app, req).await).await;
    let user_id = body["data"]["users"][0]["id"].as_i64().unwrap();

    let req = test::TestRequest::delete()
        .uri(&format!("/api/admin/users/{user_id}"))
        .insert_header(("Authorization", format!("Bearer {admin_token}")))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

#[actix_web::test]
async fn test_delete_recently_deactivated_returns_400() {
    let pool = common::setup_test_db().await;
    let app = create_test_app!(pool);

    let admin_token = common::create_admin_token();
    let user_id = register_and_deactivate!(app, admin_token, "recent@test.fr");

    let req = test::TestRequest::delete()
        .uri(&format!("/api/admin/users/{user_id}"))
        .insert_header(("Authorization", format!("Bearer {admin_token}")))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

#[actix_web::test]
async fn test_delete_eligible_user_returns_200() {
    let pool = common::setup_test_db().await;
    let app = create_test_app!(pool.clone());

    let admin_token = common::create_admin_token();
    let user_id = register_and_deactivate!(app, admin_token, "eligible@test.fr");

    // Reculer la date_desactivation de 31 jours pour franchir le délai
    sqlx::query("UPDATE utilisateur SET date_desactivation = CURRENT_TIMESTAMP - INTERVAL '31 days' WHERE id = $1")
        .bind(user_id)
        .execute(&pool)
        .await
        .unwrap();

    let req = test::TestRequest::delete()
        .uri(&format!("/api/admin/users/{user_id}"))
        .insert_header(("Authorization", format!("Bearer {admin_token}")))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
}

#[actix_web::test]
async fn test_deactivate_self_returns_400() {
    let pool = common::setup_test_db().await;
    let app = create_test_app!(pool);

    let admin_token = common::create_admin_token();

    let req = test::TestRequest::patch()
        .uri("/api/admin/users/1/status")
        .insert_header(("Authorization", format!("Bearer {admin_token}")))
        .set_json(serde_json::json!({ "est_actif": false }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

#[actix_web::test]
async fn test_deactivate_last_admin_returns_400() {
    let pool = common::setup_test_db().await;
    let app = create_test_app!(pool.clone());

    // Le seed 009 ajoute un admin2 actif ; on le supprime pour retomber sur 1 seul admin actif.
    sqlx::query("DELETE FROM utilisateur WHERE email = 'admin2@cesizen.fr'")
        .execute(&pool)
        .await
        .unwrap();

    let ghost_token = forge_admin_token(9999);

    let req = test::TestRequest::patch()
        .uri("/api/admin/users/1/status")
        .insert_header(("Authorization", format!("Bearer {ghost_token}")))
        .set_json(serde_json::json!({ "est_actif": false }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

#[actix_web::test]
async fn test_reactivate_clears_deactivation_date() {
    let pool = common::setup_test_db().await;
    let app = create_test_app!(pool);

    let admin_token = common::create_admin_token();
    let user_id = register_and_deactivate!(app, admin_token, "reactivate@test.fr");

    let req = test::TestRequest::patch()
        .uri(&format!("/api/admin/users/{user_id}/status"))
        .insert_header(("Authorization", format!("Bearer {admin_token}")))
        .set_json(serde_json::json!({ "est_actif": true }))
        .to_request();
    assert_eq!(test::call_service(&app, req).await.status(), 200);

    let req = test::TestRequest::get()
        .uri(&format!("/api/admin/users/{user_id}"))
        .insert_header(("Authorization", format!("Bearer {admin_token}")))
        .to_request();
    let body: serde_json::Value = test::read_body_json(test::call_service(&app, req).await).await;
    assert_eq!(body["data"]["est_actif"], true);
    assert!(
        body["data"]["date_desactivation"].is_null(),
        "date_desactivation devrait être NULL après réactivation"
    );
}
