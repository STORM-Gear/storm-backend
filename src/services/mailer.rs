use lettre::{
    AsyncSmtpTransport, AsyncTransport as _, Message, Tokio1Executor,
    address::AddressError,
    transport::smtp::{Error as SMTPError, authentication::Credentials},
};
use tracing::info;

use crate::{
    stripe::{PaymentInfo, ShippingMethod},
    utils::get_env_var,
};

const EMAIL_SUBJECT: &str = "Votre STORM est en route ✈️";
const EMAIL_SUBJECT_INPERSON: &str = "Votre STORM est prêt, à vous de jouer 🪂";

pub struct Mailer {
    from_email: String,
    client: AsyncSmtpTransport<Tokio1Executor>,
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

        Self {
            from_email: email,
            client,
        }
    }

    pub async fn send_checkout_confirmation(&self, info: &PaymentInfo) -> Result<(), MailerError> {
        let (subject, body) = match info.shipping_method {
            ShippingMethod::France => (EMAIL_SUBJECT, self.render_standard_email(info)),
            ShippingMethod::InPerson => (EMAIL_SUBJECT_INPERSON, self.render_inperson_email(info)),
        };

        let email = Message::builder()
            .from(self.from_email.parse().unwrap())
            .to(info
                .customer_email
                .parse()
                .map_err(|e| MailerError::Parse(e))?)
            .subject(subject)
            .body(body)
            .unwrap();

        self.client
            .send(email)
            .await
            .map_err(|e| MailerError::SMTP(e))?;
        info!("Confirmation email sent.");
        Ok(())
    }

    fn render_standard_email(&self, info: &PaymentInfo) -> String {
        format!(
            r#"
Bonjour {},

Votre commande a bien été reçue et votre paiement confirmé, merci pour votre confiance !

Votre variomètre STORM est en cours de préparation. Vous recevrez un email dès qu'il sera expédié.

───────────────────────
📦 Récapitulatif de commande
───────────────────────
Produit : Vario STORM
Montant : {:.2} €
Mode de livraison : Livraison France
───────────────────────

Si vous avez la moindre question d'ici là, n'hésitez pas à nous écrire à {}.

Bons vols,
L'équipe STORM Gear
        "#,
            info.customer_name, info.revenue, self.from_email
        )
    }

    fn render_inperson_email(&self, info: &PaymentInfo) -> String {
        format!(
            r#"
Bonjour {},

Votre commande a bien été reçue et votre paiement confirmé, merci pour votre confiance !

Vous avez choisi la remise en main propre. Pour convenir d'un rendez-vous, contactez-nous directement à {} en précisant vos disponibilités.

───────────────────────
📦 Récapitulatif de commande
───────────────────────
Produit : Vario STORM
Montant : {:.2} €
Mode de livraison : Remise en main propre
───────────────────────

À très vite sur le terrain,
L'équipe STORM Gear
                "#,
            info.customer_name, self.from_email, info.revenue
        )
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
