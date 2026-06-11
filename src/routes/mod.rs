use std::borrow::Borrow as _;

use actix_web::*;
use stripe_shared::CheckoutSession;
use stripe_webhook::{EventObject, Webhook};

use crate::AppState;

#[post("/stripe/webhook")]
pub async fn webhook_handler(
    req: HttpRequest,
    payload: web::Bytes,
    app_data: web::Data<AppState>,
) -> impl Responder {
    let payload_str = std::str::from_utf8(payload.borrow()).unwrap();

    let stripe_signature = req
        .headers()
        .get("Stripe-Signature")
        .unwrap()
        .to_str()
        .unwrap();

    if let Ok(event) =
        Webhook::construct_event(payload_str, stripe_signature, &app_data.stripe_secret)
    {
        match event.data.object {
            EventObject::CheckoutSessionCompleted(session) => {
                handle_checkout_session_success(*session);
            }
            _ => {
                println!("Unknown event encountered in webhook: {:?}", event.type_);
            }
        }
    } else {
        println!("Failed to construct webhook event, ensure your webhook secret is correct.");
    }

    HttpResponse::Ok().finish()
}

fn handle_checkout_session_success(session: CheckoutSession) {
    println!("{:#?}", session);
}
