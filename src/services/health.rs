use actix_web::{get, HttpResponse, Responder};

/// Health check route / default homepage redirecting to the GitHub repo.
#[get("/")]
async fn health_check() -> impl Responder {
    let html_content = include_str!("../../assets/index.html");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_content)
}
