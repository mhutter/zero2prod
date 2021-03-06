#[tokio::main]
#[cfg(not(tarpaulin_include))]
async fn main() -> std::io::Result<()> {
    use std::{net::TcpListener, time::Duration};

    use sqlx::postgres::PgPoolOptions;
    use zero2prod::{
        configuration::Settings,
        startup,
        telemetry::{get_subscriber, init_subscriber},
    };

    // Set up tracing/logging
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Read configuration
    let cfg = Settings::new().expect("read configuration");
    let addr = format!("{}:{}", cfg.application.host, cfg.application.port);

    // Bind listener to port
    let listener = TcpListener::bind(&addr)?;

    // Connect to DB
    let postgres_pool = PgPoolOptions::new()
        .connect_timeout(Duration::from_secs(5))
        .connect_lazy_with(cfg.database.with_db());

    // Start up
    tracing::debug!("Listening on http://{}/", &addr);
    startup::run(listener, postgres_pool)?.await
}
