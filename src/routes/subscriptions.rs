use actix_web::{
    web::{Data, Form},
    HttpResponse,
};
use chrono::Utc;
use serde::Deserialize;
use sqlx::{query, PgPool};
use uuid::Uuid;

use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(db, form),
    fields(
        subscriber.name = %form.name,
        subscriber.email = %form.email,
    )
)]
#[allow(clippy::async_yields_async)]
pub async fn subscribe(db: Data<PgPool>, form: Form<FormData>) -> HttpResponse {
    let new_subscriber = match form.0.try_into() {
        Ok(s) => s,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    match insert_subscriber(&db, &new_subscriber).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

impl TryFrom<FormData> for NewSubscriber {
    type Error = String;

    fn try_from(form: FormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(form.name)?;
        let email = SubscriberEmail::parse(form.email)?;
        Ok(Self { email, name })
    }
}

#[tracing::instrument(name = "Saving subscriber details into DB", skip(db, new_subscriber))]
pub async fn insert_subscriber(
    db: &PgPool,
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
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
