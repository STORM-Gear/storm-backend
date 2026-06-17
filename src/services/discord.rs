use rand::seq::IndexedRandom;
use serde_json::json;

use crate::{stripe::PaymentInfo, utils::get_env_var};

const MEME_URLS: [&str; 8] = [
    // Stonks
    "https://media.giphy.com/media/v1.Y2lkPTc5MGI3NjExd214Z3p5bm1ycDVidjJzMTJidTUzYnc0MHFybWR0ZHJjeWJ6OWx4dSZlcD12MV9naWZzX3NlYXJjaCZjdD1n/YnkMcHgNIMW4Yfmjxr/giphy.gif",
    // Cat vibe stocks
    "https://media.giphy.com/media/v1.Y2lkPTc5MGI3NjExd214Z3p5bm1ycDVidjJzMTJidTUzYnc0MHFybWR0ZHJjeWJ6OWx4dSZlcD12MV9naWZzX3NlYXJjaCZjdD1n/Opgs8NUosTAnRSFYzc/giphy.gif",
    // Stock prices going up
    "https://media.giphy.com/media/v1.Y2lkPTc5MGI3NjExd214Z3p5bm1ycDVidjJzMTJidTUzYnc0MHFybWR0ZHJjeWJ6OWx4dSZlcD12MV9naWZzX3NlYXJjaCZjdD1n/JtBZm3Getg3dqxK0zP/giphy.gif",
    // Checking daily revenue Wall Street wolf
    "https://media.giphy.com/media/v1.Y2lkPWVjZjA1ZTQ3dGN5bGFuNGpsaGtzaGFpdWFodzVsa3Y3b2pubWpzc2YzYWhkOTdqbiZlcD12MV9naWZzX3NlYXJjaCZjdD1n/w6NLkHuoWlcdXIHktr/giphy.gif",
    // Breaking bad money pile
    "https://media.giphy.com/media/v1.Y2lkPTc5MGI3NjExbTZ2OGs4aW1lMGI3Y3gzdWw0cXNrNHZ3a3Zhb2xzdGU4MWF6a25iMiZlcD12MV9naWZzX3NlYXJjaCZjdD1n/3oKIPm3BynUpUysTHW/giphy.gif",
    // Vince McMahon
    "https://media.giphy.com/media/v1.Y2lkPTc5MGI3NjExbTZ2OGs4aW1lMGI3Y3gzdWw0cXNrNHZ3a3Zhb2xzdGU4MWF6a25iMiZlcD12MV9naWZzX3NlYXJjaCZjdD1n/kj41Ti8GLVs1STX0bH/giphy.gif",
    // Wall Street Wolf dropping money
    "https://media.giphy.com/media/v1.Y2lkPTc5MGI3NjExbTZ2OGs4aW1lMGI3Y3gzdWw0cXNrNHZ3a3Zhb2xzdGU4MWF6a25iMiZlcD12MV9naWZzX3NlYXJjaCZjdD1n/gTURHJs4e2Ies/giphy.gif",
    // Donald Duck counting bills
    "https://media.giphy.com/media/v1.Y2lkPTc5MGI3NjExM2dwa2Q1cTkxajdxd2sycjh3bTNmaHZlNHhsb2IzYXg4bGpmMjF0cyZlcD12MV9naWZzX3NlYXJjaCZjdD1n/xTiTnqUxyWbsAXq7Ju/giphy.gif",
];

pub struct DiscordWebhook {
    url: String,
    client: reqwest::Client,
}

impl DiscordWebhook {
    pub fn from_env() -> Self {
        Self {
            url: get_env_var("DISCORD_WEBHOOK_URL"),
            client: reqwest::Client::new(),
        }
    }

    pub async fn send_checkout_completed_message(
        &self,
        info: &PaymentInfo,
    ) -> Result<(), reqwest::Error> {
        self.client
            .post(&self.url)
            // Thanks to https://phantombot.gg/tools/discord-embed-creator !
            .json(&json!({
              "embeds": [
                {
                  "color": 0x451bea,
                  "title": "🪂 Nouvelle commande !",
                  "url": format!("https://dashboard.stripe.com/payments/{}", info.payment_id),
                  "image": {
                    "url": MEME_URLS.choose(&mut rand::rng())
                  },
                  "fields": [
                    {
                      "name": "💶  Montant",
                      "value": format!("{:.2} {}", info.revenue, info.currency),
                      "inline": false
                    },
                    {
                      "name": "📦  Mode de livraison",
                      "value": info.shipping_method.to_string(),
                      "inline": false
                    }
                  ]
                }
              ]

            }))
            .send()
            .await?;

        Ok(())
    }
}
