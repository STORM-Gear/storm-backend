use std::borrow::Borrow as _;

use actix_web::{HttpRequest, web};
use stripe_shared::CheckoutSession;
use stripe_webhook::{EventObject, Webhook};

pub mod errors;

use crate::utils::get_env_var;
use errors::{PaymentInfoParsingError as ParseError, WebhookProcessingError as HookError};

pub struct StripeWebhookHandler {
    secret: String,
}

#[derive(Debug, Clone)]
pub struct PaymentInfo {
    pub revenue: f64,
    pub currency: String,
    pub customer_name: String,
    pub customer_email: String,
    pub analytics_id: Option<String>,
}

impl StripeWebhookHandler {
    pub fn from_env() -> Self {
        Self {
            secret: get_env_var("STRIPE_SECRET"),
        }
    }

    pub fn get_payment_info(
        &self,
        request: HttpRequest,
        payload: web::Bytes,
    ) -> Result<PaymentInfo, HookError> {
        let payload_str = std::str::from_utf8(payload.borrow()).unwrap();

        let stripe_signature = request
            .headers()
            .get("Stripe-Signature")
            .ok_or(HookError::MissingSignatureHeader)?
            .to_str()
            .unwrap();

        if let Ok(event) = Webhook::construct_event(payload_str, stripe_signature, &self.secret) {
            match event.data.object {
                EventObject::CheckoutSessionCompleted(session) => {
                    PaymentInfo::try_from(*session).map_err(|e| HookError::ParseError(e))
                }
                _ => Err(HookError::UnhandledEvent(event.type_)),
            }
        } else {
            Err(HookError::InvalidSignature)
        }
    }
}

impl TryFrom<CheckoutSession> for PaymentInfo {
    type Error = ParseError;

    fn try_from(session: CheckoutSession) -> Result<Self, Self::Error> {
        let currency = session
            .currency
            .ok_or(ParseError::MissingField("currency"))?
            .to_string()
            .to_ascii_uppercase();

        let revenue = session
            .amount_total
            .ok_or(ParseError::MissingField("amount_total"))
            .and_then(|revenue_raw| match currency.as_str() {
                "USD" | "EUR" => Ok(revenue_raw as f64 / 100.0),
                _ => Err(ParseError::UnhandledCurrency(currency.clone())),
            })?;

        let customer_details = session
            .customer_details
            .ok_or(ParseError::MissingField("customer_details"))?;
        let name = customer_details
            .name
            .ok_or(ParseError::MissingField("customer_details.name"))?;
        let email = customer_details
            .email
            .ok_or(ParseError::MissingField("customer_details.email"))?;

        let id = session.client_reference_id;

        Ok(Self {
            revenue,
            currency,
            customer_name: name,
            customer_email: email,
            analytics_id: id,
        })
    }
}
