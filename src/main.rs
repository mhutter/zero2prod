use std::{io, net::TcpListener};

use zero2prod::startup;

#[tokio::main]
#[cfg(not(tarpaulin_include))]
async fn main() -> io::Result<()> {
    use sqlx::PgPool;
    use zero2prod::configuration::get_configuration;

    let cfg = get_configuration().expect("read configuration");
    let addr = format!("127.0.0.1:{}", cfg.port);
    let listener = TcpListener::bind(&addr)?;
    let postgres_pool = PgPool::connect(&cfg.database.connection_string())
        .await
        .expect("Connect to PostgreSQL");

    println!("Listening to http://{}/", &addr);
    startup::run(listener, postgres_pool)?.await
}
