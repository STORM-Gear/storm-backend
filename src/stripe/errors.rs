use stripe_shared::EventType;

pub enum WebhookProcessingError {
    MissingSignatureHeader,
    InvalidPayload,
    InvalidSignature,
    UnhandledEvent(EventType),
    ParseError(PaymentInfoParsingError),
}

pub enum PaymentInfoParsingError {
    MissingField(&'static str),
    UnhandledCurrency(String),
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
        }
    }
}
