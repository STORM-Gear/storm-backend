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
                handle_checkout_session_success(*session, &app_data).await;
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

async fn handle_checkout_session_success(session: CheckoutSession, app_data: &AppState) {
    const EVENT_NAME: &'static str = "checkout-completed";

    let currency = session
        .currency
        .as_ref()
        .expect("Chechout session should provide 'currency'")
        .to_string()
        .to_ascii_uppercase();

    let revenue = {
        let revenue_raw = session
            .amount_total
            .expect("Chechout session should provide 'amount_total'");

        match currency.as_str() {
            "USD" | "EUR" => revenue_raw as f64 / 100.0,
            _ => panic!("Currency {currency} is not supported"),
        }
    };

    let (name, email) = session
        .customer_details
        .map(|details| (details.name, details.email))
        .unzip();

    let body = serde_json::json!({
        "type": "event",
        "payload": {
            "website": app_data.analytics_website_id,
            "name": EVENT_NAME,
            "id": email, // Uniquely identify the user
            "data": {
                "revenue": revenue,
                "currency": currency,
                "customer_name": name,
            }
        }
    });

    println!("Sending '{EVENT_NAME}' event to analytics server...");

    let res = app_data
        .http_client
        .post(&app_data.analytics_api_url)
        .json(&body)
        .send()
        .await;

    match res {
        Ok(response) => {
            if response.status().is_success() {
                println!("Successfully sent '{EVENT_NAME}' event to analytics server")
            } else {
                println!(
                    "Error in response when sending '{EVENT_NAME}' event to analytics server. Status: {}",
                    response.status()
                )
            }
        }

        Err(e) => {
            println!("Error when sending '{EVENT_NAME}' event to analytics server: {e}")
        }
    }
}
