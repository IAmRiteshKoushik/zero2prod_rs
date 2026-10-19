mod common;

// `tokio::test` is used for tests. Using this, you don't have to specify the
// test attribute and it makes the function async too.
//
// For inspection: cargo expand --test health_check (<- name of test file)
#[tokio::test]
async fn health_check_works() {
    let app_address = common::spawn_app().await;
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
