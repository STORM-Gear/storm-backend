use actix_web::*;
use clap::Parser;

use crate::{
    services::{analytics::AnalyticsServer, mailer::Mailer},
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
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let Args { port, bind_address } = Args::parse();

    let stripe = StripeWebhookHandler::from_env();
    let analytics = AnalyticsServer::from_env();
    let mailer = Mailer::from_env();

    let app_data = web::Data::new(AppState {
        stripe,
        analytics,
        mailer,
    });

    println!("Starting server on http://{bind_address}:{port}");

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(routes::webhook_handler)
    })
    .bind((bind_address, port))?
    .run()
    .await
}
