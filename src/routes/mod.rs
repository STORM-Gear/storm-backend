use actix_web::*;
use tracing::error;

use crate::{AppState, stripe::PaymentInfo};

#[post("/stripe/webhook")]
pub async fn webhook_handler(
    request: HttpRequest,
    payload: web::Bytes,
    app_data: web::Data<AppState>,
) -> impl Responder {
    match app_data.stripe.get_payment_info(request, payload) {
        Ok(payment_info) => {
            payment_pipeline(payment_info, &app_data).await;
        }
        Err(e) => {
            error!("{}", e);
        }
    };

    HttpResponse::Ok().finish()
}

async fn payment_pipeline(payment_info: PaymentInfo, app_data: &AppState) {
    let (_, mail_res) = tokio::join!(
        app_data.analytics.send_checkout_completed(&payment_info),
        app_data.mailer.send_checkout_confirmation(&payment_info),
    );

    if let Err(e) = mail_res {
        error!("Failed to send checkout confirmation email: {e}");
    };

    let discord_res = app_data
        .discord
        .send_checkout_completed_message(&payment_info)
        .await;

    if let Err(e) = discord_res {
        error!("Failed to send Discord notification: {e}");
    };
}
