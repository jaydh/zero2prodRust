use crate::authentication::UserId;
use crate::domain::SubscriberEmail;
use crate::email_client::EmailClient;
use crate::idempotency::{get_saved_response, save_response, IdempotencyKey};
use crate::utils::{e400, e500, see_other};
use actix_web::{web, HttpResponse};
use actix_web_flash_messages::FlashMessage;
use anyhow::Context;
use sqlx::PgPool;

struct ConfirmedSubscriber {
    email: SubscriberEmail,
}

#[derive(serde::Deserialize)]
pub struct FormData {
    title: String,
    html_content: String,
    text_content: String,
    idempotency_key: String,
}

#[tracing::instrument(
    name = "Publish a newsletter issue",
    skip(form, pool, email_client),
    fields(user_id=%*user_id)
)]
pub async fn publish_newsletter(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
    user_id: web::ReqData<UserId>,
) -> Result<HttpResponse, actix_web::Error> {
    let FormData {
        title,
        html_content,
        text_content,
        idempotency_key,
    } = form.0;
    let user_id = user_id.into_inner();
    let idempotency_key: IdempotencyKey = idempotency_key.try_into().map_err(e400)?;
    if let Some(saved_response) = get_saved_response(&pool, &idempotency_key, *user_id)
        .await
        .map_err(e500)?
    {
        FlashMessage::info("The newsletter issue has been published!").send();
        return Ok(saved_response);
    }

    let subscribers = get_confirmed_subscribers(&pool).await.map_err(e500)?;
    for subscriber in subscribers {
        match subscriber {
            Ok(subscriber) => {
                email_client
                    .send_email(&subscriber.email, &title, &html_content, &text_content)
                    .await
                    .with_context(|| {
                        format!("Failed ot send newsletter issue to {}", subscriber.email)
                    })
                    .map_err(e500)?;
            }
            Err(e) => {
                tracing::warn!(error.cause_chain = ?e, "Skipping a confirmed subscriber, their stored contact details are invalid");
            }
        }
    }
    FlashMessage::info("The newsletter issue has been published!").send();
    let response = see_other("/admin/newsletters");
    let response = save_response(&pool, &idempotency_key, *user_id, response)
        .await
        .map_err(e500)?;
    Ok(response)
}

#[tracing::instrument(name = "Get confirmed subscribers", skip(pool))]
async fn get_confirmed_subscribers(
    pool: &PgPool,
) -> Result<Vec<Result<ConfirmedSubscriber, anyhow::Error>>, anyhow::Error> {
    Ok(
        sqlx::query!(r#"SELECT email FROM SUBSCRIPTIONS WHERE status = 'confirmed'"#)
            .fetch_all(pool)
            .await?
            .into_iter()
            .map(|r| match SubscriberEmail::parse(r.email) {
                Ok(email) => Ok(ConfirmedSubscriber { email }),
                Err(error) => Err(anyhow::anyhow!(error)),
            })
            .collect(),
    )
}
