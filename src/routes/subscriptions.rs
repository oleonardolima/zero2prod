use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> impl Responder {
    // generate an id to correlate all request logs
    let request_id = Uuid::new_v4();

    let request_span = tracing::info_span!(
        "adding a new subscriber.",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name);

    let _request_span_guard = request_span.enter();

    // there is no need to call .enter on the query_span!, the .instrument handles it.
    let query_span =
        tracing::info_span!("saving new subscriber details in the database.", %request_id);

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
    .instrument(query_span)
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
