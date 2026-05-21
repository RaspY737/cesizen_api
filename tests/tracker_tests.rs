mod common;

use actix_web::test;

#[actix_web::test]
async fn test_create_entry_valid_returns_201() {
    let pool = common::setup_test_db().await;
    let app = create_test_app!(pool);

    let token = register_test_user!(app, "tracker@test.fr");

    let req = test::TestRequest::post()
        .uri("/api/tracker/entries")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(serde_json::json!({
            "sous_emotion_id": 1,
            "intensite": 7,
            "note": "Test entry"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["success"], true);
    assert_eq!(body["data"]["intensite"], 7);
}

#[actix_web::test]
async fn test_create_entry_intensity_too_high_returns_400() {
    let pool = common::setup_test_db().await;
    let app = create_test_app!(pool);

    let token = register_test_user!(app, "high@test.fr");

    let req = test::TestRequest::post()
        .uri("/api/tracker/entries")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(serde_json::json!({
            "sous_emotion_id": 1,
            "intensite": 999
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

#[actix_web::test]
async fn test_create_entry_intensity_too_low_returns_400() {
    let pool = common::setup_test_db().await;
    let app = create_test_app!(pool);

    let token = register_test_user!(app, "low@test.fr");

    let req = test::TestRequest::post()
        .uri("/api/tracker/entries")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(serde_json::json!({
            "sous_emotion_id": 1,
            "intensite": 0
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

#[actix_web::test]
async fn test_update_entry_intensity_too_high_returns_400() {
    let pool = common::setup_test_db().await;
    let app = create_test_app!(pool);

    let token = register_test_user!(app, "updhigh@test.fr");

    // Créer d'abord une entrée valide
    let req = test::TestRequest::post()
        .uri("/api/tracker/entries")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(serde_json::json!({
            "sous_emotion_id": 1,
            "intensite": 5
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    let entry_id = body["data"]["id"].as_i64().unwrap();

    // Tenter de mettre à jour avec une intensité invalide
    let req = test::TestRequest::put()
        .uri(&format!("/api/tracker/entries/{entry_id}"))
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(serde_json::json!({
            "sous_emotion_id": 1,
            "intensite": 99
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

#[actix_web::test]
async fn test_get_entry_returns_correct_data() {
    let pool = common::setup_test_db().await;
    let app = create_test_app!(pool);

    let token = register_test_user!(app, "getentry@test.fr");

    // Créer une entrée
    let req = test::TestRequest::post()
        .uri("/api/tracker/entries")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(serde_json::json!({
            "sous_emotion_id": 1,
            "intensite": 8,
            "note": "Ma note"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    let entry_id = body["data"]["id"].as_i64().unwrap();

    // Récupérer l'entrée
    let req = test::TestRequest::get()
        .uri(&format!("/api/tracker/entries/{entry_id}"))
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["data"]["intensite"], 8);
}

#[actix_web::test]
async fn test_idor_get_other_user_entry_returns_404() {
    let pool = common::setup_test_db().await;
    let app = create_test_app!(pool);

    // User A crée une entrée
    let token_a = register_test_user!(app, "usera@test.fr");
    let req = test::TestRequest::post()
        .uri("/api/tracker/entries")
        .insert_header(("Authorization", format!("Bearer {token_a}")))
        .set_json(serde_json::json!({
            "sous_emotion_id": 1,
            "intensite": 5
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    let entry_id = body["data"]["id"].as_i64().unwrap();

    // User B tente de lire l'entrée de A
    let token_b = register_test_user!(app, "userb@test.fr");
    let req = test::TestRequest::get()
        .uri(&format!("/api/tracker/entries/{entry_id}"))
        .insert_header(("Authorization", format!("Bearer {token_b}")))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

#[actix_web::test]
async fn test_idor_delete_other_user_entry_returns_404() {
    let pool = common::setup_test_db().await;
    let app = create_test_app!(pool);

    let token_a = register_test_user!(app, "dela@test.fr");
    let req = test::TestRequest::post()
        .uri("/api/tracker/entries")
        .insert_header(("Authorization", format!("Bearer {token_a}")))
        .set_json(serde_json::json!({
            "sous_emotion_id": 1,
            "intensite": 5
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    let entry_id = body["data"]["id"].as_i64().unwrap();

    let token_b = register_test_user!(app, "delb@test.fr");
    let req = test::TestRequest::delete()
        .uri(&format!("/api/tracker/entries/{entry_id}"))
        .insert_header(("Authorization", format!("Bearer {token_b}")))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

#[actix_web::test]
async fn test_delete_entry_returns_200() {
    let pool = common::setup_test_db().await;
    let app = create_test_app!(pool);

    let token = register_test_user!(app, "delete@test.fr");

    let req = test::TestRequest::post()
        .uri("/api/tracker/entries")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(serde_json::json!({
            "sous_emotion_id": 1,
            "intensite": 5
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    let entry_id = body["data"]["id"].as_i64().unwrap();

    let req = test::TestRequest::delete()
        .uri(&format!("/api/tracker/entries/{entry_id}"))
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
}

#[actix_web::test]
async fn test_list_entries_without_token_returns_401() {
    let pool = common::setup_test_db().await;
    let app = create_test_app!(pool);

    let req = test::TestRequest::get()
        .uri("/api/tracker/entries")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 401);
}

#[actix_web::test]
async fn test_get_stats_returns_200() {
    let pool = common::setup_test_db().await;
    let app = create_test_app!(pool);

    let token = register_test_user!(app, "stats@test.fr");

    let req = test::TestRequest::get()
        .uri("/api/tracker/stats")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["success"], true);
    assert!(body["data"]["total_entrees"].is_number());
}

#[actix_web::test]
async fn test_list_entries_filter_by_emotion_base_id_returns_only_matching() {
    let pool = common::setup_test_db().await;
    let app = create_test_app!(pool);

    let token = register_test_user!(app, "filter@test.fr");

    // Entrée Joie (emotion_base_id = 1, sous_emotion 1..=6)
    let req = test::TestRequest::post()
        .uri("/api/tracker/entries")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(serde_json::json!({ "sous_emotion_id": 1, "intensite": 5 }))
        .to_request();
    assert_eq!(test::call_service(&app, req).await.status(), 201);

    // Entrée Colère (emotion_base_id = 2, sous_emotion 7..=12)
    let req = test::TestRequest::post()
        .uri("/api/tracker/entries")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(serde_json::json!({ "sous_emotion_id": 7, "intensite": 6 }))
        .to_request();
    assert_eq!(test::call_service(&app, req).await.status(), 201);

    let req = test::TestRequest::get()
        .uri("/api/tracker/entries?emotion_base_id=1")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    let entries = body["data"].as_array().expect("data doit être un tableau");
    assert_eq!(entries.len(), 1, "seule l'entrée Joie doit être retournée");
    assert_eq!(entries[0]["emotion_base_nom"], "Joie");
}
