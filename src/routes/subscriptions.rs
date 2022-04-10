use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    log::info!(
        "Request_id {} - Adding '{}' '{}' as a new subscrber.",
        request_id,
        form.email,
        form.name
    );
    log::info!(
        "Request_id {} - Saving new subscriber details in the database.",
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
    // Use get_ref() to to get an immutable reference to the PgConnection.
    .execute(pool.as_ref())
    .await
    {
        Ok(_) => {
            log::info!(
                "Request_id {} - New subscriber details have been saved.",
                request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::error!(
                "Request_id {} - Failed to execute the query: {:?}",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
