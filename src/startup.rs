use std::{io, net::TcpListener};

use actix_web::{dev::Server, middleware::Logger, web, App, HttpServer};
use sqlx::PgPool;

use crate::routes::{health_check, subscribe};

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            // Middleware
            .wrap(Logger::default())
            // Data
            .app_data(db_pool.clone())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
