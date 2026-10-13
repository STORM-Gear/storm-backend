use actix_web::*;

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("bonk")
}
