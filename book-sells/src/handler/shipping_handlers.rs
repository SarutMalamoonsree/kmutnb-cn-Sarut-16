use actix_web::{get, Responder, HttpResponse};

#[get("/carted")]
async fn get_cart() -> impl Responder {
HttpResponse::Ok().json("Fuck You")
}