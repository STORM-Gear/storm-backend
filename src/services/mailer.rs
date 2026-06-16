use lettre::{
    AsyncSmtpTransport, AsyncTransport as _, Message, Tokio1Executor,
    address::AddressError,
    transport::smtp::{Error as SMTPError, authentication::Credentials},
};
use tracing::info;

use crate::{stripe::PaymentInfo, utils::get_env_var};

const EMAIL_SUBJECT: &str = "Votre commande a bien été prise en compte !";
const EMAIL_BODY: &str = r#"
Bonjour,

Nous vous informons que votre commande a bien été prise en compte.

Le délai de livraison estimé est de 3 jours ouvrés.

Nous espérons que votre STORM vous accompagnera dans de nombreux beaux vols.

N'hésitez pas à nous faire un retour après réception. Vos avis sont précieux pour nous aider à améliorer le produit.

Si STORM vous plaît, parler du produit autour de vous est le meilleur moyen de soutenir la marque.

Merci pour votre confiance et bons vols.

L'équipe STORM
"#;

pub struct Mailer {
    client: AsyncSmtpTransport<Tokio1Executor>,
}

pub enum MailerError {
    Parse(AddressError),
    SMTP(SMTPError),
}

impl Mailer {
    pub fn from_env() -> Self {
        let credentials = Credentials::new(get_env_var("SMTP_USER"), get_env_var("SMTP_PASS"));
        let smtp_host = get_env_var("SMTP_HOST");

        let client = AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_host)
            .unwrap()
            .credentials(credentials)
            .build();

        Self { client }
    }

    pub async fn send_checkout_confirmation(
        &self,
        payment_info: PaymentInfo,
    ) -> Result<(), MailerError> {
        let email = Message::builder()
            .from("variostorm@gmail.com".parse().unwrap())
            .to(payment_info
                .customer_email
                .parse()
                .map_err(|e| MailerError::Parse(e))?)
            .subject(EMAIL_SUBJECT)
            .body(EMAIL_BODY.to_string())
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
