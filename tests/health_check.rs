use sqlx::{Connection, PgConnection, PgPool};
use std::{net::TcpListener, vec};
use zero2prod::configuration::get_configuration;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

// launch our application in the background
async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port.");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let server = zero2prod::startup::run(listener, connection_pool.clone())
        .expect("Failed to bind address.");
    let _ = tokio::spawn(server);

    TestApp {
        address,
        db_pool: connection_pool,
    }
}

#[tokio::test]
async fn health_check_works() {
    // arrange
    let app = spawn_app().await;

    // need an HTTP client to perform HTTP requests against our application
    let client = reqwest::Client::new();

    // act
    let response = client
        .get(format!("{}/health_check", app.address))
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
    let app: TestApp = spawn_app().await;
    // let configuration = get_configuration().expect("Failed to read configuration file.");
    // let connection_string = configuration.database.connection_string();

    // let mut connection = PgConnection::connect(&connection_string)
    //     .await
    //     .expect("Failed to connect to Postgres");

    let client = reqwest::Client::new();

    // act
    let body = "name=aliocha%20karamazov&email=aliocha_karamazov%40email.com";
    let response = client
        .post(format!("{}/subscriptions", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute the request.");

    // assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "aliocha_karamazov@email.com");
    assert_eq!(saved.name, "aliocha karamazov");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // arrange
    let app = spawn_app().await;
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
            .post(format!("{}/subscriptions", app.address))
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
