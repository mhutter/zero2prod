#[tokio::main]
#[cfg(not(tarpaulin_include))]
async fn main() -> std::io::Result<()> {
    use std::{net::TcpListener, time::Duration};

    use secrecy::ExposeSecret;
    use sqlx::postgres::PgPoolOptions;
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
    let addr = format!("{}:{}", cfg.application.host, cfg.application.port);

    // Bind listener to port
    let listener = TcpListener::bind(&addr)?;

    // Connect to DB
    let postgres_pool = PgPoolOptions::new()
        .connect_timeout(Duration::from_secs(5))
        .connect_lazy(cfg.database.connection_string().expose_secret())
        .expect("Connect to PostgreSQL");

    // Start up
    tracing::debug!("Listening on http://{}/", &addr);
    startup::run(listener, postgres_pool)?.await
}
