use crate::routes::{health_check, subscribe};
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::{PgPool};
use std::net::TcpListener;

pub fn run(lst: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // wraps the db_pool in a smart pointer (Arc)
    // in order to make it cloneable
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
    .listen(lst)?
    .run();
    Ok(server)
}
