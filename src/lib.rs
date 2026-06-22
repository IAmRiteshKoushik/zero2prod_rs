use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, web};

async fn health_check(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn subscribe() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    // The HTTP server handles all transport level concerns
    // The App handles all the routing logic, middlewares, handlers etc.
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::get().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
