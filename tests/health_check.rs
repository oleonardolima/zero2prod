use sqlx::{Connection, PgConnection};
use std::{net::TcpListener, vec};
use zero2prod::configuration::get_configuration;

// launch our application in the background
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port.");
    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::startup::run(listener).expect("Failed to bind address.");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    // arrange
    let addr = spawn_app();

    // need an HTTP client to perform HTTP requests against our application
    let client = reqwest::Client::new();

    // act
    let response = client
        .get(format!("{}/health_check", addr))
        .send()
        .await
        .expect("Failed to execute request.");

    // assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // arrange
    let app_address = spawn_app();
    let configuration = get_configuration().expect("Failed to read configuration file.");
    let connection_string = configuration.database.connection_string();

    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres");

    let client = reqwest::Client::new();

    // act
    let body = "name=aliocha%20karamazov&email=aliocha_karamazov%40email.com";
    let response = client
        .post(format!("{}/subscriptions", app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute the request.");

    // assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "aliocha_karamazov@email.com");
    assert_eq!(saved.name, "aliocha karamazov");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    // this is an example of table driven tests (https://github.com/golang/go/wiki/TableDrivenTests)
    let test_cases = vec![
        ("name=fyodor%karamazov", "missing the email value"),
        (
            "email=dimitri_karamazov@email.com",
            "missing the name value",
        ),
        ("smerdiakov", "missing both name and email values"),
    ];

    // act
    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{}/subscriptions", app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute the request.");

        // assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // aditional customized error message on test failure only (https://doc.rust-lang.org/std/macro.assert.html#custom-messages)
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
