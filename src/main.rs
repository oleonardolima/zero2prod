use std::net::TcpListener;

use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Port 0 is special-cased at the OS level:
    // trying to bind port 0 will trigger an OS scan for an available port which will then be bound to the application.
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind random port.");
    run(listener)?.await
}
