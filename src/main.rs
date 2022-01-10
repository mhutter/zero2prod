use std::{io, net::TcpListener};

use zero2prod::startup;

#[tokio::main]
#[cfg(not(tarpaulin_include))]
async fn main() -> io::Result<()> {
    use env_logger::Env;
    use sqlx::PgPool;
    use zero2prod::configuration::get_configuration;

    // Set up logging
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

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
    log::debug!("Listening on http://{}/", &addr);
    startup::run(listener, postgres_pool)?.await
}
