use std::io::Result;

use actix_web::HttpResponse;
use actix_web::web::{Data, Path};
use diesel::prelude::*;
use log::debug;

use statty_common::context::Context;
use statty_domain::schema::vehicles::dsl::vehicles;
use statty_domain::schema::vehicles::id;
use statty_domain::vehicle::Vehicle;

pub async fn list_vehicles(data: Data<Context>) -> Result<HttpResponse> {
    let connection = &mut data.clone().get_pool().get().unwrap();
    let results = vehicles
        .limit(10)
        .select(Vehicle::as_select())
        .load(connection)
        .expect("Failed to retrieve vehicles");

    debug!("Retrieved {} vehicles", results.len());

    return Ok(HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&results).unwrap()))
}

pub async fn vehicle_details(data: Data<Context>, path: Path<i32>) -> Result<HttpResponse> {
    let conn = &mut data.clone().get_pool().get().unwrap();
    let res = vehicles
        .filter(id.eq(path.into_inner()))
        .select(Vehicle::as_select())
        .load(conn)
        .expect("Failed to retrieve Vehicle data");

    return Ok(
        HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&res).unwrap())
    )
}