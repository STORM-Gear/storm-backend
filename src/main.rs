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
    stripe_secret: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let Args { port, bind_address } = Args::parse();

    let stripe_secret = std::env::var("STRIPE_SECRET").expect("'STRIPE_SECRET' env var required");
    let app_data = web::Data::new(AppState { stripe_secret });

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
