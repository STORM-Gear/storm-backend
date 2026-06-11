use actix_web::*;
use clap::Parser;

mod routes;

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
    http_client: reqwest::Client,
    analytics_website_id: String,
    analytics_api_url: String,
    stripe_secret: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let Args { port, bind_address } = Args::parse();

    let http_client = reqwest::Client::new();

    fn get_env_var(name: &'static str) -> String {
        std::env::var(name).expect(format!("'{name}' env var required").as_str())
    }
    let stripe_secret = get_env_var("STRIPE_SECRET");
    let analytics_website_id = get_env_var("ANALYTICS_WEBSITE_ID");
    let analytics_api_url = get_env_var("ANALYTICS_API_URL");

    let app_data = web::Data::new(AppState {
        http_client,
        analytics_website_id,
        analytics_api_url,
        stripe_secret,
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
