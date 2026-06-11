use actix_web::*;
use stripe_core::Event as StripeEvent;

#[post("/stripe/webhook")]
async fn stripe_webhook(event: web::Json<StripeEvent>) -> impl Responder {
    println!("Event: {event:?}");

    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(stripe_webhook))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
