use std::time::Duration;

use actix_web::{App, HttpServer};
mod services;
mod utils;
use actix_extensible_rate_limit::{
    backend::{memory::InMemoryBackend, SimpleInputFunctionBuilder},
    RateLimiter,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let backend = InMemoryBackend::builder().build();
    HttpServer::new(move || {
        let input = SimpleInputFunctionBuilder::new(Duration::from_secs(60), 15) // 10 requests per minute per ip
            .real_ip_key()
            .build();
        let rate_limiter_middleware = RateLimiter::builder(backend.clone(), input)
            .add_headers()
            .build();
        App::new()
            .wrap(rate_limiter_middleware)
            .service(services::stack_overflow::gen_image)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}