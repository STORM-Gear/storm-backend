use actix_web::{
    http::{StatusCode, header::ContentType},
    *,
};
use tracing::error;

use crate::{
    AppState,
    db::InsertPaymentError,
    services::mailer::MailerError,
    stripe::{PaymentInfo, errors::WebhookProcessingError},
};

#[derive(Debug, thiserror::Error)]
enum PaymentPipelineError {
    #[error("stripe error: {0}")]
    Stripe(#[from] WebhookProcessingError),
    #[error("database error: {0}")]
    Database(#[from] InsertPaymentError),
    #[error("mailer error: {0}")]
    Mailer(#[from] MailerError),
    #[error("discord error: {0}")]
    Discord(#[from] reqwest::Error),
}

#[post("/stripe/webhook")]
pub async fn webhook_handler(
    request: HttpRequest,
    payload: web::Bytes,
    app_data: web::Data<AppState>,
) -> Result<(), PaymentPipelineError> {
    let payment_info = app_data.stripe.get_payment_info(request, payload).await?;

    if let Err(e) = payment_pipeline(payment_info, &app_data).await {
        error!("payment pipeline error: {e}");
        if let Err(discord_error) = app_data
            .discord
            .send_internal_error_message(e.to_string())
            .await
        {
            error!("failed to report error message through discord: {discord_error}");
        }

        Err(e)
    } else {
        Ok(())
    }
}

async fn payment_pipeline(
    payment_info: PaymentInfo,
    app_data: &AppState,
) -> Result<(), PaymentPipelineError> {
    {
        let mut db = app_data.db.lock().await;
        db.insert_payment(payment_info.clone()).await?;
    }

    app_data
        .mailer
        .send_checkout_confirmation(&payment_info)
        .await?;

    app_data
        .analytics
        .send_checkout_completed(&payment_info)
        .await;

    app_data
        .discord
        .send_checkout_completed_message(&payment_info)
        .await?;

    Ok(())
}

impl ResponseError for PaymentPipelineError {
    fn error_response(&self) -> HttpResponse<body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> http::StatusCode {
        match self {
            PaymentPipelineError::Stripe(e) => match e {
                WebhookProcessingError::MissingSignatureHeader
                | WebhookProcessingError::InvalidPayload
                | WebhookProcessingError::ParseError(_) => StatusCode::BAD_REQUEST,
                WebhookProcessingError::InvalidSignature => StatusCode::UNAUTHORIZED,
                WebhookProcessingError::UnhandledEvent(_) => StatusCode::NOT_FOUND,
                WebhookProcessingError::Stripe(_) => StatusCode::INTERNAL_SERVER_ERROR,
            },
            PaymentPipelineError::Database(e) => match e {
                InsertPaymentError::AlreadyExists(_) => StatusCode::OK,
                InsertPaymentError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
                InsertPaymentError::ProductNotFound(_)
                | InsertPaymentError::ShippingMethodNotFound(_) => StatusCode::NOT_FOUND,
            },
            // Send OK as these are non-critical
            PaymentPipelineError::Mailer(_) | PaymentPipelineError::Discord(_) => StatusCode::OK,
        }
    }
}
