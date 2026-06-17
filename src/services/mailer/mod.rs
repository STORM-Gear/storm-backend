use lettre::{
    AsyncSmtpTransport, AsyncTransport as _, Message, Tokio1Executor,
    address::AddressError,
    message::SinglePart,
    transport::smtp::{Error as SMTPError, authentication::Credentials},
};
use minijinja::{Environment, context};
use tracing::info;

use crate::{
    stripe::{PaymentInfo, ShippingMethod},
    utils::get_env_var,
};

pub struct Mailer {
    from_email: String,
    client: AsyncSmtpTransport<Tokio1Executor>,
    templates: Environment<'static>,
}

pub enum MailerError {
    Parse(AddressError),
    SMTP(SMTPError),
}

impl Mailer {
    pub fn from_env() -> Self {
        let email = get_env_var("SMTP_USER");
        let credentials = Credentials::new(email.clone(), get_env_var("SMTP_PASS"));
        let smtp_host = get_env_var("SMTP_HOST");

        let client = AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_host)
            .unwrap()
            .credentials(credentials)
            .build();

        let mut templates = Environment::new();

        templates
            .add_template(
                "confirmation_fr",
                include_str!("templates/confirmation_fr.html.jinja"),
            )
            .unwrap();

        Self {
            from_email: email,
            client,
            templates,
        }
    }

    pub async fn send_checkout_confirmation(&self, info: &PaymentInfo) -> Result<(), MailerError> {
        let subject = match info.shipping_method {
            ShippingMethod::InPerson => "Votre STORM est prêt, à vous de jouer 🪂",
            ShippingMethod::FranceStandard | ShippingMethod::International => {
                "Votre STORM est en route ✈️"
            }
            ShippingMethod::FranceTracking
            | ShippingMethod::InternationalTracking
            | ShippingMethod::FranceExpressTracking => "Votre STORM est en route ✈️",
        };

        let template = self.templates.get_template("confirmation_fr").unwrap();
        let body = template
            .render(context! {
                logo_url => "https://stormvario.fr/logo.svg",
                customer_name => info.customer_name,
                amount => format!("{:.2}", info.revenue),
                currency => info.currency.to_uppercase(),
                payment_id => info.payment_id,
                shipping_method => format!("{:?}", info.shipping_method),
                from_email => self.from_email,
            })
            .unwrap();

        let email = Message::builder()
            .from(self.from_email.parse().unwrap())
            .to(info
                .customer_email
                .parse()
                .map_err(|e| MailerError::Parse(e))?)
            .subject(subject)
            .singlepart(SinglePart::html(body))
            .unwrap();

        self.client
            .send(email)
            .await
            .map_err(|e| MailerError::SMTP(e))?;
        info!("Confirmation email sent.");
        Ok(())
    }
}

impl std::fmt::Display for MailerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MailerError::Parse(e) => write!(f, "Failed to parse customer email address: {e}"),
            MailerError::SMTP(e) => write!(f, "Failed to send email over SMTP: {e}"),
        }
    }
}
