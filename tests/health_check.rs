use std::net::TcpListener;

use sqlx::{Connection, PgConnection};
use zero2prod::configuration::get_configuration;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", address))
        .send()
        .await
        .expect("execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    // Bind to a random free port
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind random port");
    let port = listener.local_addr().unwrap().port();

    // Bind the server to the port & start it
    let server = zero2prod::startup::run(listener).expect("start server");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form() {
    let address = spawn_app();
    let configuration = get_configuration().expect("read configuration");
    let db_url = configuration.database.connection_string();
    let mut connection = PgConnection::connect(&db_url).await.expect("connect to DB");
    let client = reqwest::Client::new();

    let body = "name=hans%20wurst&email=hans.wurst%40example.com";
    let response = client
        .post(format!("{}/subscriptions", address))
        .header("content-type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("send request");

    assert_eq!(200, response.status().as_u16());
    let saved = sqlx::query!("select email, name from subscriptions")
        .fetch_one(&mut connection)
        .await
        .expect("fetch saved subscriptions");
    assert_eq!("hans.wurst@example.com", saved.email);
    assert_eq!("hans wurst", saved.name);
}

#[tokio::test]
async fn subscribe_returns_400_when_data_is_missing() {
    let url = &format!("{}/subscriptions", spawn_app());
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
