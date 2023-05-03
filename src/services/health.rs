use actix_web::{get, HttpResponse, Responder};

/// Health check route.
#[get("/")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}