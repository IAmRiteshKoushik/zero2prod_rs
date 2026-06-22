use std::net::TcpListener;

use zero2prod_rs::run;

// Asynchronous programming in Rust is built on top of the Future trait. A
// future is basically a value that is not there yet. It is like a Promise in JS.
// Each future exposes a `Poll()` method which has to be called so that the
// future can make progress and eventually resolve. Unlike promises which are
// resolved eagerly upon creation. Hence, futures they are lazy by default.

// The Rust standard library by design does not contain an async runtime, and
// the developer is expected to bring one into the project as a depdendency.

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")?;
    // As futures are lazy loaded, the server does nto run unless you supply
    // the await keyword
    run(listener)?.await
}
