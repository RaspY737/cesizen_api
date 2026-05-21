mod common;

use actix_web::test;

#[actix_web::test]
async fn test_health_check_returns_200() {
    let pool = common::setup_test_db().await;
    let app = create_test_app!(pool);

    let req = test::TestRequest::get().uri("/api/health").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);
}
