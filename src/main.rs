use actix_files as fs;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(fs::Files::new("/", "./static"))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}