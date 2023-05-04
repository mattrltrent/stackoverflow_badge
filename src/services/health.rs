use actix_web::{get, HttpResponse, Responder};

/// Health check route.
#[get("/")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("View the repo @ https://github.com/mattrltrent/stackoverflow_badge")
}