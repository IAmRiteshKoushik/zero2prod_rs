use std::net::TcpListener;

async fn spawn_app() -> String {
    let listener =
        TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    // Retrieve the port assigned by the operating system
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod_rs::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

// `tokio::test` is used for tests. Using this, you don't have to specify the
// test attribute and it makes the function async too.
//
// For inspection: cargo expand --test health_check (<- name of test file)
#[tokio::test]
async fn health_check_works() {
    let app_address = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &app_address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Asserts
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // let app_address = spawn_app().await;
    // let client = reqwest::Client::new();
    todo!()
}
