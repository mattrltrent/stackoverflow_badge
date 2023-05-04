use std::{env, time::Duration};

use actix_treblle::Treblle;
use actix_web::{App, HttpServer, middleware::Logger};
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
    let port = env::var("PORT").expect("Error getting port");
    let treblle_project_id =
        env::var("TREBLLE_PROJECT_ID").expect("Error getting Treblle project id");
    let treblle_api_key = env::var("TREBLLE_API_KEY").expect("Error getting Treblle api key");
    HttpServer::new(move || {
        let input = SimpleInputFunctionBuilder::new(Duration::from_secs(60), 15) // 15 requests per minute per ip
            .real_ip_key()
            .build();
        let rate_limiter_middleware = RateLimiter::builder(backend.clone(), input)
            .add_headers()
            .build();
        App::new()
            // Treblle project id and api key for endpoint logging
            .wrap(Treblle::new(
                treblle_project_id.clone(),
                treblle_api_key.clone(),
            ))
            // Middleware for basic IP-based rate limiting
            .wrap(rate_limiter_middleware)
            // Adds logging (output level = info!() level)
            .wrap(Logger::default())
            .service(services::stack_overflow::handler_1)
            .service(services::stack_overflow::handler_2)
            .service(services::health::health_check)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
