use actix_web::*;
use clap::Parser;
use stripe_core::Event as StripeEvent;

#[post("/stripe/webhook")]
async fn stripe_webhook(event: web::Json<StripeEvent>) -> impl Responder {
    println!("Event: {event:?}");

    HttpResponse::Ok().body("Hello world!")
}

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let Args { port, bind_address } = Args::parse();

    println!("Starting server on http://{bind_address}:{port}");

    HttpServer::new(|| App::new().service(stripe_webhook))
        .bind((bind_address, port))?
        .run()
        .await
}
