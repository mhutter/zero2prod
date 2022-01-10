use actix_web::{
    web::{Data, Form},
    HttpResponse,
};
use chrono::Utc;
use serde::Deserialize;
use sqlx::{query, PgPool};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(db: Data<PgPool>, form: Form<FormData>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    log::info!(
        "Request<{}> Adding '{} <{}>' as a new subscriber",
        request_id,
        form.name,
        form.email
    );
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
    .await
    {
        Ok(_) => {
            log::info!("Request<{}> New subscriber has been saved", request_id);
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::error!(
                "Request<{}> Failed to create subscription: {:?}",
                request_id,
                e,
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
