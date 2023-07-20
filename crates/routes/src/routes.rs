use io::Result;
use std::io;
use actix_web::{HttpRequest, HttpResponse, web};
use statty_api::vehicle_service::list_vehicles;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(
                web::scope("/ev_stats/vehicles")
                    .route("", web::get().to(list_vehicles))
                    .service(
                        web::scope("/chargesessions")
                            .route("", web::get().to(todo))
                            .route("", web::post().to(todo))
                    )
            )
    );
}

async fn todo(_req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::NoContent().finish())
}