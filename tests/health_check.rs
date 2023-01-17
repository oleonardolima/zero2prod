#[tokio::test]
async fn health_check_works() {
    // arrange
    spawn_app();

    // need an HTTP client to perform HTTP requests against our application
    let client = reqwest::Client::new();

    // act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    // assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// launch our application in the background
fn spawn_app() {
    // todo!()
    let server = zero2prod::run().expect("Failed to bind address.");
    let _ = tokio::spawn(server);
}