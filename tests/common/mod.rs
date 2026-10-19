use std::net::TcpListener;

pub async fn spawn_app() -> String {
    let listener =
        TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    // Retrieve the port assigned by the operating system
    let port = listener.local_addr().unwrap().port();
    let server =
        zero2prod_rs::startup::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
