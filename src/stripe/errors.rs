use stripe::StripeError;
use stripe_webhook::EventType;

#[derive(Debug, thiserror::Error)]
pub enum WebhookProcessingError {
    #[error("missing signature header")]
    MissingSignatureHeader,
    #[error("invalid payload")]
    InvalidPayload,
    #[error("invalid signature")]
    InvalidSignature,
    #[error("unhandled event type: {0}")]
    UnhandledEvent(EventType),
    #[error("failed to build PaymentInfo from CheckoutSession: {0}")]
    ParseError(PaymentInfoParsingError),
    #[error("stripe api error: {0}")]
    Stripe(StripeError),
}

#[derive(Debug, thiserror::Error)]
pub enum PaymentInfoParsingError {
    #[error("missing field: {0}")]
    MissingField(&'static str),
    #[error("unhandled currency: {0}")]
    UnhandledCurrency(String),
    #[error("unknown shipping rate id: {0}")]
    UnknownShippingRate(String),
    #[error("unknown product id: {0}")]
    UnknownProduct(String),
}
