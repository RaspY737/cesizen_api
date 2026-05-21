use sqlx::{Executor, PgPool};
use testcontainers_modules::postgres::Postgres;
use testcontainers_modules::testcontainers::ContainerAsync;
use testcontainers_modules::testcontainers::runners::AsyncRunner;
use tokio::sync::OnceCell;
use uuid::Uuid;

pub const TEST_JWT_SECRET: &str = "test-secret-for-integration-tests";

struct SharedPostgres {
    _container: ContainerAsync<Postgres>,
    admin_url: String,
    host: String,
    port: u16,
}

static SHARED: OnceCell<SharedPostgres> = OnceCell::const_new();

async fn shared_postgres() -> &'static SharedPostgres {
    SHARED
        .get_or_init(|| async {
            let container = Postgres::default()
                .start()
                .await
                .expect("Impossible de démarrer le conteneur Postgres de test");
            let host = container
                .get_host()
                .await
                .expect("Impossible de récupérer l'hôte du conteneur")
                .to_string();
            let port = container
                .get_host_port_ipv4(5432)
                .await
                .expect("Impossible de récupérer le port mappé du conteneur");
            let admin_url = format!("postgres://postgres:postgres@{host}:{port}/postgres");
            SharedPostgres {
                _container: container,
                admin_url,
                host,
                port,
            }
        })
        .await
}

/// Crée une base de test isolée (une par appel), applique toutes les migrations,
/// et retourne un pool connecté.
pub async fn setup_test_db() -> PgPool {
    let shared = shared_postgres().await;

    let db_name = format!("test_{}", Uuid::new_v4().simple());

    let admin_pool = PgPool::connect(&shared.admin_url)
        .await
        .expect("Impossible de se connecter à la base admin du conteneur");
    admin_pool
        .execute(format!("CREATE DATABASE \"{db_name}\"").as_str())
        .await
        .expect("Impossible de créer la base de test");
    admin_pool.close().await;

    let db_url = format!(
        "postgres://postgres:postgres@{}:{}/{}",
        shared.host, shared.port, db_name
    );
    let pool = PgPool::connect(&db_url)
        .await
        .expect("Impossible de se connecter à la base de test");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Erreur lors de l'exécution des migrations");

    pool
}

/// Macro pour créer l'app de test.
#[macro_export]
macro_rules! create_test_app {
    ($pool:expr) => {
        actix_web::test::init_service(
            actix_web::App::new()
                .app_data(actix_web::web::Data::new($pool))
                .app_data(actix_web::web::Data::new(
                    common::TEST_JWT_SECRET.to_string(),
                ))
                .configure(cesizen_api::configure_app),
        )
        .await
    };
}

/// Macro pour enregistrer un utilisateur et obtenir le token.
#[macro_export]
macro_rules! register_test_user {
    ($app:expr, $email:expr) => {{
        let req = actix_web::test::TestRequest::post()
            .uri("/api/auth/register")
            .peer_addr("127.0.0.1:12345".parse().unwrap())
            .set_json(serde_json::json!({
                "email": $email,
                "mot_de_passe": "Secure123!",
                "nom": "Test",
                "prenom": "User"
            }))
            .to_request();
        let resp = actix_web::test::call_service(&$app, req).await;
        let body: serde_json::Value = actix_web::test::read_body_json(resp).await;
        body["data"]["token"].as_str().unwrap().to_string()
    }};
}

/// Crée un token JWT admin directement (sans passer par la DB).
pub fn create_admin_token() -> String {
    cesizen_api::middleware::auth::create_token(1, "administrateur", TEST_JWT_SECRET).unwrap()
}

/// Crée un token JWT utilisateur directement.
#[allow(dead_code)]
pub fn create_user_token(user_id: i32) -> String {
    cesizen_api::middleware::auth::create_token(user_id, "utilisateur", TEST_JWT_SECRET).unwrap()
}
