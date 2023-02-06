use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> impl Responder {
    // generate an id to correlate all request logs
    let request_id = Uuid::new_v4();
    tracing::info!(
        "request_id {} - adding '{}' '{}' as a new subscriber",
        request_id,
        form.email,
        form.name
    );
    tracing::info!(
        "request_id {} - saving new subscriber details in the database",
        request_id
    );
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            tracing::info!(
                "request_id {} - successfully saved new subscriber details to database",
                request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!(
                "request_id {} - failed to execute query to insert new subscriber: {:?}",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
