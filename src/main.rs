use std::{time::Duration, env};

use actix_web::{App, HttpServer};
mod services;
mod utils;
use actix_extensible_rate_limit::{
    backend::{memory::InMemoryBackend, SimpleInputFunctionBuilder},
    RateLimiter,
};
extern crate dotenv;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let backend = InMemoryBackend::builder().build();
    dotenv().ok();
    let port = env::var("PORT").expect("Port not set");
    HttpServer::new(move || {
        let input = SimpleInputFunctionBuilder::new(Duration::from_secs(60), 15) // 15 requests per minute per ip
            .real_ip_key()
            .build();
        let rate_limiter_middleware = RateLimiter::builder(backend.clone(), input)
            .add_headers()
            .build();
        App::new()
            .wrap(rate_limiter_middleware)
            .service(services::stack_overflow::gen_image)
            .service(services::health::hello_world)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
