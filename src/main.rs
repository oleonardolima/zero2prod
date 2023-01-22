use std::net::TcpListener;

use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", configuration.application_port))
        .expect("Failed to bind random port.");
    run(listener)?.await
}
