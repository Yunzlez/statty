use std::io;
use io::Result;
use actix_web::{HttpRequest, HttpResponse, web};
use statty_api::vehicle_service::list_vehicles;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(
                web::scope("/vehicles")
                    .route("", web::get().to(list_vehicles))
            )
    );
}

async fn todo(_req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::NoContent().finish())
}