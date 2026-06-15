use lettre::{
    AsyncSmtpTransport, AsyncTransport as _, Message, Tokio1Executor,
    transport::smtp::authentication::Credentials,
};

use crate::{stripe::PaymentInfo, utils::get_env_var};

pub struct Mailer {
    client: AsyncSmtpTransport<Tokio1Executor>,
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

    pub async fn send_checkout_confirmation(&self, payment_info: PaymentInfo) {
        let email = Message::builder()
            .from("variostorm@gmail.com".parse().unwrap())
            .to(payment_info.customer_email.parse().unwrap())
            .subject("Commande confirmée")
            .body(format!("Bonjour {}, merci pour votre commande ! Elle vous sera livrée très prochainement.\nCdt\n\nSTORM Gear", payment_info.customer_name))
            .unwrap();

        self.client.send(email).await.unwrap();
        println!("Email sent.");
    }
}
