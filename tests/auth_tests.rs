mod common;

use actix_web::test;

#[actix_web::test]
async fn test_register_valid_returns_201() {
    let pool = common::setup_test_db().await;
    let app = create_test_app!(pool);

    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .peer_addr("127.0.0.1:12345".parse().unwrap())
        .set_json(serde_json::json!({
            "email": "newuser@test.fr",
            "mot_de_passe": "Secure123!",
            "nom": "Test",
            "prenom": "User"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["success"], true);
    assert!(body["data"]["token"].is_string());
    assert_eq!(body["data"]["user"]["email"], "newuser@test.fr");
    assert_eq!(body["data"]["user"]["role"], "utilisateur");
}

#[actix_web::test]
async fn test_register_empty_email_returns_400() {
    let pool = common::setup_test_db().await;
    let app = create_test_app!(pool);

    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .peer_addr("127.0.0.1:12345".parse().unwrap())
        .set_json(serde_json::json!({
            "email": "",
            "mot_de_passe": "Secure123!",
            "nom": "Test",
            "prenom": "User"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

#[actix_web::test]
async fn test_register_invalid_email_format_returns_400() {
    let pool = common::setup_test_db().await;
    let app = create_test_app!(pool);

    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .peer_addr("127.0.0.1:12345".parse().unwrap())
        .set_json(serde_json::json!({
            "email": "abc",
            "mot_de_passe": "Secure123!",
            "nom": "Test",
            "prenom": "User"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

#[actix_web::test]
async fn test_register_weak_password_returns_400() {
    let pool = common::setup_test_db().await;
    let app = create_test_app!(pool);

    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .peer_addr("127.0.0.1:12345".parse().unwrap())
        .set_json(serde_json::json!({
            "email": "weak@test.fr",
            "mot_de_passe": "Ab1",
            "nom": "Test",
            "prenom": "User"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

#[actix_web::test]
async fn test_register_password_no_uppercase_returns_400() {
    let pool = common::setup_test_db().await;
    let app = create_test_app!(pool);

    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .peer_addr("127.0.0.1:12345".parse().unwrap())
        .set_json(serde_json::json!({
            "email": "noup@test.fr",
            "mot_de_passe": "secure123!",
            "nom": "Test",
            "prenom": "User"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

#[actix_web::test]
async fn test_register_duplicate_email_returns_400() {
    let pool = common::setup_test_db().await;
    let app = create_test_app!(pool);

    // Premier enregistrement
    register_test_user!(app, "dup@test.fr");

    // Doublon
    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .peer_addr("127.0.0.1:12345".parse().unwrap())
        .set_json(serde_json::json!({
            "email": "dup@test.fr",
            "mot_de_passe": "Secure123!",
            "nom": "Test2",
            "prenom": "User2"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

#[actix_web::test]
async fn test_login_valid_returns_200() {
    let pool = common::setup_test_db().await;
    let app = create_test_app!(pool);

    register_test_user!(app, "login@test.fr");

    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .peer_addr("127.0.0.1:12345".parse().unwrap())
        .set_json(serde_json::json!({
            "email": "login@test.fr",
            "mot_de_passe": "Secure123!"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["success"], true);
    assert!(body["data"]["token"].is_string());
}

#[actix_web::test]
async fn test_login_wrong_password_returns_401() {
    let pool = common::setup_test_db().await;
    let app = create_test_app!(pool);

    register_test_user!(app, "wrong@test.fr");

    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .peer_addr("127.0.0.1:12345".parse().unwrap())
        .set_json(serde_json::json!({
            "email": "wrong@test.fr",
            "mot_de_passe": "WrongPass1!"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 401);
}

#[actix_web::test]
async fn test_login_nonexistent_email_returns_401() {
    let pool = common::setup_test_db().await;
    let app = create_test_app!(pool);

    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .peer_addr("127.0.0.1:12345".parse().unwrap())
        .set_json(serde_json::json!({
            "email": "nobody@test.fr",
            "mot_de_passe": "Secure123!"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 401);
}
