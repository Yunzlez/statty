use actix_web::{HttpRequest, HttpResponse};
use statty_domain::vehicle::Vehicle;
use statty_domain::schema::vehicles::dsl::vehicles;
use statty_db::db_conn::get_db_conn;
use diesel::prelude::*;
use std::io::Result;

pub async fn list_vehicles(_req: HttpRequest) -> Result<HttpResponse> {
    let connection = &mut get_db_conn();

    let results = vehicles
        .limit(10)
        .select(Vehicle::as_select())
        .load(connection)
        .expect("Failed to retrieve vehicles");

    println!("Retrieved {} vehicles", results.len());

    return Ok(HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&results).unwrap()))
}