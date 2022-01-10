#[tokio::main]
#[cfg(not(tarpaulin_include))]
async fn main() -> std::io::Result<()> {
    use std::net::TcpListener;

    use sqlx::PgPool;
    use zero2prod::{
        configuration::get_configuration,
        startup,
        telemetry::{get_subscriber, init_subscriber},
    };

    // Set up tracing/logging
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Read configuration
    let cfg = get_configuration().expect("read configuration");
    let addr = format!("127.0.0.1:{}", cfg.port);

    // Bind listener to port
    let listener = TcpListener::bind(&addr)?;

    // Connect to DB
    let postgres_pool = PgPool::connect(&cfg.database.connection_string())
        .await
        .expect("Connect to PostgreSQL");

    // Start up
    tracing::debug!("Listening on http://{}/", &addr);
    startup::run(listener, postgres_pool)?.await
}
