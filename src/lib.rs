use std::{io, net::TcpListener};

use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, Responder};

pub fn run(listener: TcpListener) -> Result<Server, io::Error> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .listen(listener)?
        .run();

    Ok(server)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
