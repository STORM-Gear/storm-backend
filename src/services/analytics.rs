use tracing::{error, info};

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

    pub async fn send_checkout_completed(&self, info: &PaymentInfo) {
        const EVENT_NAME: &str = "checkout-completed";

        let mut payload = serde_json::json!({
            "website": self.website_id,
            "name": EVENT_NAME,
            "data": {
                "revenue": info.revenue,
                "currency": info.currency,
                "customer_name": info.customer_name,
                "customer_email": info.customer_email,
            }
        });

        if let Some(id) = &info.analytics_id {
            payload["id"] = serde_json::json!(id);
        }

        let body = serde_json::json!({ "type": "event", "payload": payload });

        let res = self.client.post(&self.api_url).json(&body).send().await;

        match res {
            Ok(response) => {
                if response.status().is_success() {
                    info!("Successfully sent '{EVENT_NAME}' event to analytics server")
                } else {
                    error!(
                        "Error in response when sending '{EVENT_NAME}' event to analytics server. Status: {}",
                        response.status()
                    )
                }
            }

            Err(e) => {
                error!("Error when sending '{EVENT_NAME}' event to analytics server: {e}")
            }
        }
    }
}
