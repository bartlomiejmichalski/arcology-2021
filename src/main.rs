mod r2d2_pool;
mod handlers; 
mod model;

use r2d2_pool::RedisPool;
use handlers::{ping, retrieve, command};

use actix_web::{web, App, HttpServer};
use actix_web::{middleware::Logger};

pub struct SharedData {
    pub pool: RedisPool
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    
    HttpServer::new(|| {
        let redis_address = format!("redis://{}", std::env::var("REDIS_ADDRESS").unwrap_or("127.0.0.1".to_string()));
        let pool: RedisPool = r2d2_pool::connect(&redis_address).expect("Cannot create r2d2 pool");
        let data = web::Data::new(
            SharedData{
                pool: pool
            });
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(data.clone())
            .service(ping)
            .service(retrieve)
            .service(command)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
