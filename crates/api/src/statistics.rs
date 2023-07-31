use std::io::Result;
use actix_web::http::StatusCode;

use actix_web::HttpResponse;
use actix_web::web::{Data, Path};
use diesel::prelude::*;

use statty_common::context::Context;
use statty_domain::charge_session::ChargeSession;
use statty_domain::schema::charge_sessions::{date, vehicle_id, energy};
use statty_domain::schema::charge_sessions::dsl::charge_sessions;
use statty_domain::schema::vehicles::dsl::vehicles;
use statty_domain::schema::vehicles::id;
use statty_domain::stats::GlobalStatistics;
use statty_domain::vehicle::Vehicle;
use statty_common::http_utils::http_error;

pub async fn get_stats(ctx: Data<Context>, path: Path<i32>) -> Result<HttpResponse> {
    let conn = &mut ctx.clone().get_pool().get().unwrap();

    let path_id = &path.into_inner();

    let vehicle = vehicles
        .filter(id.eq(path_id))
        .select(Vehicle::as_select())
        .load(conn)
        .expect("Failed to retrieve Vehicle data");

    let results = charge_sessions
        .filter(vehicle_id.eq(path_id))
        .limit(2)
        .select(ChargeSession::as_select())
        .order(date.desc())
        .load(conn)
        .expect("Failed to retrieve sessions");

    let session_count = charge_sessions
        .filter(vehicle_id.eq(path_id))
        .count()
        .get_result(conn)
        .expect("Failed to retrieve sessions");

    //execute query: select COALESCE(SUM(energy), 0) from charge_sessions where vehicle_id=?;
    let total_energy = charge_sessions
        .filter(vehicle_id.eq(path_id))
        .select(diesel::dsl::sum(energy))
        .first::<Option<f64>>(conn)
        .expect("Failed to retrieve sessions")
        .unwrap();

    //throw an error if less than 2 sessions are available
    if results.len() < 2 {
        return http_error(StatusCode::NOT_FOUND, "Not enough data available to calculate statistics");
    }

    //battery capacity in kWh
    let cap = vehicle.get(0).unwrap().battery_capacity as f64;
    //charge limit between 0 and 1
    let clim = vehicle.get(0).unwrap().charge_limit;
    //the energy charged in the last session in kWh
    let charged_energy = results.get(0).unwrap().energy;
    //the odometer reading at the end of the last session
    let odo = results.get(0).unwrap().odometer;
    //the SoC at the end of the last session
    let prev_soc = results.get(1).unwrap().end_soc as f64;
    //the odometer reading at the end of the second last session
    let prev_odo = results.get(1).unwrap().odometer as f64;

    println!("cap={}; clim={}; energy={}; odo={}; prev_soc={}; prev_odo={}", cap, clim, charged_energy, odo, prev_soc, prev_odo);

    let stats = GlobalStatistics {
        //assume the car was charged to clim at the start of the first session
        avg_consumption: ( (total_energy + (cap * clim)) / (odo as f64)) * 100.0,
        //this calculation assumes the battery SoC is reported linearly by the vehicle
        avg_consumption_last_charge: ((charged_energy - (cap * (clim - prev_soc/100.0)))/(odo as f64 - prev_odo)) * 100.0,
        num_sessions: session_count,
        total_distance: odo
    };

    return Ok(HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&stats).unwrap()));
}