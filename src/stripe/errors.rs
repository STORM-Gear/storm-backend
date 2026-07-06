use stripe::StripeError;
use stripe_webhook::EventType;

pub enum WebhookProcessingError {
    MissingSignatureHeader,
    InvalidPayload,
    InvalidSignature,
    UnhandledEvent(EventType),
    ParseError(PaymentInfoParsingError),
    Stripe(StripeError),
}

pub enum PaymentInfoParsingError {
    MissingField(&'static str),
    UnhandledCurrency(String),
    UnknownShippingRate(String),
    UnknownProduct(String),
}

impl std::fmt::Display for WebhookProcessingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebhookProcessingError::MissingSignatureHeader => write!(f, "Missing signature header"),
            WebhookProcessingError::InvalidPayload => write!(f, "Invalid payload"),
            WebhookProcessingError::InvalidSignature => write!(f, "Invalid webhook signature"),
            WebhookProcessingError::UnhandledEvent(event_type) => {
                write!(f, "Unhandled event type: {}", event_type)
            }
            WebhookProcessingError::ParseError(e) => write!(
                f,
                "Failed to build `PaymentInfo` from `CheckoutSession: {e}`"
            ),
            WebhookProcessingError::Stripe(e) => writeln!(f, "Failed to call Stripe API: {e}"),
        }
    }
}

impl std::fmt::Display for PaymentInfoParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaymentInfoParsingError::MissingField(field) => write!(f, "Missing '{field}' field"),
            PaymentInfoParsingError::UnhandledCurrency(currency) => {
                write!(f, "Unhandled currency '{currency}'")
            }
            PaymentInfoParsingError::UnknownShippingRate(id) => {
                write!(f, "Unknown shipping rate ID: {id}")
            }
            PaymentInfoParsingError::UnknownProduct(id) => {
                write!(f, "Unknown product ID: {id}")
            }
        }
    }
}
