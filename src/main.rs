use std::net::TcpListener;

use zero2prod_rs::{configuration::get_configuration, startup::run};

// Asynchronous programming in Rust is built on top of the Future trait. A
// future is basically a value that is not there yet. It is like a Promise in JS.
// Each future exposes a `Poll()` method which has to be called so that the
// future can make progress and eventually resolve. Unlike promises which are
// resolved eagerly upon creation. Hence, futures they are lazy by default.

// The Rust standard library by design does not contain an async runtime, and
// the developer is expected to bring one into the project as a depdendency.

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Panic if we cant read configuration
    let configuration =
        get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    // As futures are lazy loaded, the server does nto run unless you supply
    // the await keyword
    run(listener)?.await
}
