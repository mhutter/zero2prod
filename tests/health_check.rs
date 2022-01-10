use std::net::TcpListener;

use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use zero2prod::configuration::{get_configuration, DatabaseSettings};

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", app.address))
        .send()
        .await
        .expect("execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

pub struct TestApp {
    pub address: String,
    pub db: PgPool,
}

async fn spawn_app() -> TestApp {
    // Bind to a random free port
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind random port");
    let port = listener.local_addr().unwrap().port();

    // Configure DB
    let mut cfg = get_configuration().expect("read configuration");
    cfg.database.name = Uuid::new_v4().to_string();
    let db = configure_database(&cfg.database).await;

    // Bind the server to the port & start it
    let server = zero2prod::startup::run(listener, db.clone()).expect("start server");
    let _ = tokio::spawn(server);

    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        db,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create Database
    let mut conn = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Connect to PostgreSQL");
    conn.execute(format!(r#"CREATE DATABASE "{}";"#, config.name).as_str())
        .await
        .expect("Create Database");

    // Run DB migrations
    let conn = PgPool::connect(&config.connection_string())
        .await
        .expect("Connect to PostgreSQL");
    sqlx::migrate!("./migrations")
        .run(&conn)
        .await
        .expect("Applying migrations");

    conn
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=hans%20wurst&email=hans.wurst%40example.com";
    let response = client
        .post(format!("{}/subscriptions", app.address))
        .header("content-type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("send request");

    assert_eq!(200, response.status().as_u16());
    let saved = sqlx::query!("select email, name from subscriptions")
        .fetch_one(&app.db)
        .await
        .expect("fetch saved subscriptions");
    assert_eq!("hans.wurst@example.com", saved.email);
    assert_eq!("hans wurst", saved.name);
}

#[tokio::test]
async fn subscribe_returns_400_when_data_is_missing() {
    let app = spawn_app().await;
    let url = &format!("{}/subscriptions", app.address);
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=hans%20wurst", "missing the email"),
        ("email=hans.wurst%40example.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (body, error_message) in test_cases {
        let response = client
            .post(url)
            .header("content-type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("send request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "API did not fail with 400 when body was {}",
            error_message,
        );
    }
}
