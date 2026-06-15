use actix_web::*;

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
            println!("{}", e);
        }
    };

    HttpResponse::Ok().finish()
}

async fn payment_pipeline(payment_info: PaymentInfo, app_data: &AppState) {
    app_data
        .analytics
        .send_checkout_completed(payment_info.clone())
        .await;

    app_data
        .mailer
        .send_checkout_confirmation(payment_info)
        .await;
}
