use actix_web::*;
use clap::Parser;
use tracing::info;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    services::{analytics::AnalyticsServer, discord::DiscordWebhook, mailer::Mailer},
    stripe::StripeWebhookHandler,
};

mod routes;
mod services;
mod stripe;
mod utils;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The port to start the server on
    #[arg(short, long, default_value_t = 8080)]
    port: u16,

    /// The address to bind the server on
    #[arg(short, long, default_value = "127.0.0.1")]
    bind_address: String,
}

struct AppState {
    stripe: StripeWebhookHandler,
    analytics: AnalyticsServer,
    mailer: Mailer,
    discord: DiscordWebhook,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info,actix_web=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let Args { port, bind_address } = Args::parse();

    let stripe = StripeWebhookHandler::from_env();
    let analytics = AnalyticsServer::from_env();
    let mailer = Mailer::from_env();
    let discord = DiscordWebhook::from_env();

    let app_data = web::Data::new(AppState {
        stripe,
        analytics,
        mailer,
        discord,
    });

    info!("Starting server on http://{bind_address}:{port}");

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(TracingLogger::default())
            .service(routes::webhook_handler)
    })
    .bind((bind_address, port))?
    .run()
    .await
}
