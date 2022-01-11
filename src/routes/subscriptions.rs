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

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(db, form),
    fields(
        request_id = %Uuid::new_v4(),
        subscriber_name = %form.name,
        subscriber_email = %form.email,
    )
)]
#[allow(clippy::async_yields_async)]
pub async fn subscribe(db: Data<PgPool>, form: Form<FormData>) -> HttpResponse {
    match insert_subscriber(&db, &form).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(name = "Saving subscriber details into DB", skip(db, form))]
pub async fn insert_subscriber(db: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now(),
    )
    .execute(db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create subscriptions: {:?}", e);
        e
    })?;

    Ok(())
}
