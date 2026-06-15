use crate::{stripe::PaymentInfo, utils::get_env_var};

pub struct AnalyticsServer {
    client: reqwest::Client,
    api_url: String,
    website_id: String,
}

impl AnalyticsServer {
    pub fn from_env() -> Self {
        Self {
            client: reqwest::Client::new(),
            api_url: get_env_var("ANALYTICS_API_URL"),
            website_id: get_env_var("ANALYTICS_WEBSITE_ID"),
        }
    }

    pub async fn send_checkout_completed(&self, payment_info: PaymentInfo) {
        const EVENT_NAME: &'static str = "checkout-completed";

        let body = serde_json::json!({
            "type": "event",
            "payload": {
                "website": self.website_id,
                "name": EVENT_NAME,
                "id": payment_info.analytics_id,
                "data": {
                    "revenue": payment_info.revenue,
                    "currency": payment_info.currency,
                    "customer_name": payment_info.customer_name,
                    "customer_email": payment_info.customer_email,
                }
            }
        });

        let res = self.client.post(&self.api_url).json(&body).send().await;

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
}
