use std::io::Result;

use actix_web::HttpResponse;
use actix_web::web::{Data, Path};
use diesel::prelude::*;

use statty_common::context::Context;
use statty_domain::charge_session::ChargeSession;
use statty_domain::schema::charge_sessions::date;
use statty_domain::schema::charge_sessions::dsl::charge_sessions;
use statty_domain::schema::vehicles::dsl::vehicles;
use statty_domain::schema::vehicles::id;
use statty_domain::stats::GlobalStatistics;
use statty_domain::vehicle::Vehicle;

pub async fn get_stats(ctx: Data<Context>, path: Path<i32>) -> Result<HttpResponse> {
    let conn = &mut ctx.clone().get_pool().get().unwrap();

    let vehicle = vehicles
        .filter(id.eq(path.into_inner()))
        .select(Vehicle::as_select())
        .load(conn)
        .expect("Failed to retrieve Vehicle data");

    let results = charge_sessions
        .limit(2)
        .select(ChargeSession::as_select())
        .order(date.desc())
        .load(conn)
        .expect("Failed to retrieve sessions");

    if results.len() != 2 {
        //todo error
    }

    let cap = vehicle.get(0).unwrap().battery_capacity as f64;
    let clim = vehicle.get(0).unwrap().charge_limit;
    let energy = results.get(0).unwrap().energy;
    let odo = results.get(0).unwrap().odometer as f64;
    let prev_soc = results.get(1).unwrap().end_soc as f64;
    let prev_odo = results.get(1).unwrap().odometer as f64;

    println!("cap={}; clim={}; energy={}; odo={}; prev_soc={}; prev_odo={}", cap, clim, energy, odo, prev_soc, prev_odo);

    let stats = GlobalStatistics {
        avg_consumption: 0.0,
        avg_consumption_last_charge: ((energy - (cap * (clim - prev_soc/100.0)))/(odo - prev_odo)) * 100.0,
    };

    return Ok(HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&stats).unwrap()));
}