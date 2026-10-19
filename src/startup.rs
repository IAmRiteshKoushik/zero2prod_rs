use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use std::net::TcpListener;

use super::routes;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    // The HTTP server handles all transport level concerns
    // The App handles all the routing logic, middlewares, handlers etc.
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(routes::health_check))
            .route("/subscriptions", web::post().to(routes::subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
