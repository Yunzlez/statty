use actix_web::{App, HttpServer};
use statty_routes::routes::config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(|cfg| config(cfg))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}