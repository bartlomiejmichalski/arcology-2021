use actix_web::{web, App, HttpServer, Responder};

async fn greet() -> impl Responder {
    format!("Hello World")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}