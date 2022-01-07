use std::net::TcpListener;

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
    let server = zero2prod::run(listener).expect("start server");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
