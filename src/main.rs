use actix_web::{App, HttpServer};
mod services;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(services::stack_overflow::gen_image)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}