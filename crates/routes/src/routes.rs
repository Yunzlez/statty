use std::io;
use io::Result;
use actix_web::{HttpRequest, HttpResponse, web};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(
                web::scope("/ascope")
                    .route("", web::get().to(todo))
            )
    );
}

async fn todo(_req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::NoContent().finish())
}