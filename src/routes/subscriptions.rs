use actix_web::{
    web::{Data, Form},
    HttpResponse,
};
use chrono::Utc;
use serde::Deserialize;
use sqlx::{query, PgPool};
use tracing::Instrument;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(db: Data<PgPool>, form: Form<FormData>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new subscriber",
        %request_id,
        subscriber_name=%form.name,
        subscriber_email=%form.email,
    );
    let _request_span_guard = request_span.enter();

    let query_span = tracing::info_span!("Saving subscriber details into DB");
    match query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now(),
    )
    // `get_ref` returns an immutable reference to the `PgConnection` within
    // `web::Data`.
    .execute(db.get_ref())
    // Attach instrumentation
    .instrument(query_span)
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            tracing::error!("Failed to create subscription: {:?}", e,);
            HttpResponse::InternalServerError().finish()
        }
    }
}
