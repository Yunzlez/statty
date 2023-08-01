use std::collections::HashMap;
use std::io::Result;

use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use actix_web::web::{Data, Path, Query};
use diesel::prelude::*;

use statty_common::context::Context;
use statty_common::http_utils::http_error;
use statty_common::period::parse_period;
use statty_domain::charge_session::ChargeSession;
use statty_domain::schema::charge_sessions::{date, energy, vehicle_id};
use statty_domain::schema::charge_sessions::dsl::charge_sessions;
use statty_domain::schema::vehicles::dsl::vehicles;
use statty_domain::schema::vehicles::id;
use statty_domain::stats::GlobalStatistics;
use statty_domain::vehicle::Vehicle;

pub async fn get_stats(ctx: Data<Context>, path: Path<i32>, query: Query<HashMap<String, String>>) -> Result<HttpResponse> {
    let conn = &mut ctx.clone().get_pool().get().unwrap();

    let path_id = &path.into_inner();

    let all = &"all".to_string();
    let period_str = query.get("period").unwrap_or(all);

    let period = match parse_period(period_str.parse().unwrap()) {
        Ok(it) => it,
        Err(e) => return http_error(StatusCode::BAD_REQUEST, &*e.to_string())
    };

    let vehicle = vehicles
        .filter(id.eq(path_id))
        .select(Vehicle::as_select())
        .load(conn)
        .expect("Failed to retrieve Vehicle data");

    //get all sessions for the given period
    let results = charge_sessions
        .filter(vehicle_id.eq(path_id))
        .filter(date.gt(period))
        .limit(2)
        .select(ChargeSession::as_select())
        .order(date.desc())
        .load(conn)
        .expect("Failed to retrieve sessions");

    //get last session before period
    //only used when period is not "all"
    //tbd if it's better to skip the query for performance reasons when period is "all"
    let prev_session = charge_sessions
        .filter(vehicle_id.eq(path_id))
        .filter(date.lt(period))
        .select(ChargeSession::as_select())
        .order(date.desc())
        .first::<ChargeSession>(conn)
        .optional()
        .expect("Failed to retrieve sessions");

    //get the total number of sessions
    let session_count = charge_sessions
        .filter(vehicle_id.eq(path_id))
        .filter(date.gt(period))
        .count()
        .get_result(conn)
        .expect("Failed to retrieve sessions");

    //execute query: select COALESCE(SUM(energy), 0) from charge_sessions where vehicle_id=?;
    let total_energy = charge_sessions
        .filter(vehicle_id.eq(path_id))
        .filter(date.gt(period))
        .select(diesel::dsl::sum(energy))
        .first::<Option<f64>>(conn)
        .expect("Failed to retrieve sessions")
        .unwrap_or(0.0);

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
    //the odometer reading at the start of the period
    let period_start_odo = prev_session.map_or(0, |s| s.odometer);

    println!("cap={}; clim={}; charged_energy={}; total_energy={}; odo={}; prev_soc={}; prev_odo={}; period_start_odo={}", cap, clim, charged_energy, total_energy, odo, prev_soc, prev_odo, period_start_odo);
    
    let avg_consumption = match period_start_odo {
        //assume the car was charged to clim at the start of the first session
        0 => ((total_energy + (cap * clim)) / (odo as f64)) * 100.0,
        //if we're not taking all sessions into account, we need to start the odometer reading at the session before the period
        _ => ((total_energy) / ((odo - period_start_odo) as f64)) * 100.0,
    };

    let stats = GlobalStatistics {
        avg_consumption,
        //this calculation assumes the battery SoC is reported linearly by the vehicle
        avg_consumption_last_charge: ((charged_energy - (cap * (clim - prev_soc/100.0)))/(odo as f64 - prev_odo)) * 100.0,
        num_sessions: session_count,
        total_distance: odo
    };

    return Ok(HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&stats).unwrap()));
}