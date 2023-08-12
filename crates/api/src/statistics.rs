use std::collections::HashMap;
use std::io::{Error, ErrorKind, Result};

use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use actix_web::web::{Data, Path, Query};
use diesel::data_types::PgInterval;
use diesel::dsl::{now, not};
use diesel::internal::derives::as_expression::Bound;
use diesel::internal::derives::numeric_ops::Sub;
use diesel::prelude::*;
use diesel::sql_types::Interval;

use statty_common::context::Context;
use statty_common::http_utils::http_error;
use statty_common::period::parse_period;
use statty_domain::charge_session::ChargeSession;
use statty_domain::schema::charge_sessions::{date, energy, vehicle_id, id as session_id};
use statty_domain::schema::charge_sessions::dsl::charge_sessions;
use statty_domain::schema::vehicles::dsl::vehicles;
use statty_domain::schema::vehicles::id;
use statty_domain::stats::GlobalStatistics;
use statty_domain::vehicle::Vehicle;

pub async fn get_stats(ctx: Data<Context>, path: Path<i32>, query: Query<HashMap<String, String>>) -> Result<HttpResponse> {
    let path_id = &path.into_inner();

    let all = &"all".to_string();
    let period_str = query.get("period").unwrap_or(all);
    let prev_stats_str = query.get("prev").unwrap_or(all);

    let prev_stats = match prev_stats_str {
        _ if prev_stats_str == "true" => true,
        _ => false,
    };

    let period = match parse_period(period_str.parse().unwrap()) {
        Ok(it) => it,
        Err(e) => return http_error(StatusCode::BAD_REQUEST, &*e.to_string())
    };

    let query_result = match query_data(ctx, path_id, period, prev_stats) {
        Ok(it) => it,
        Err(e) => return http_error(StatusCode::NOT_FOUND, &*e.to_string())
    };

    //just to make the calculations below a bit more readable
    let cap = query_result.capacity;
    let clim = query_result.charge_limit;
    let charged_energy = query_result.charged_energy;
    let odo = query_result.odometer;
    let prev_soc = query_result.prev_soc;
    let prev_odo = query_result.prev_odo;
    let period_start_odo = query_result.period_start_odo;
    let total_energy = query_result.total_energy;

    println!("cap={}; clim={}; charged_energy={}; total_energy={}; odo={}; prev_soc={}; prev_odo={}; period_start_odo={}", cap, clim, charged_energy, total_energy, odo, prev_soc, prev_odo, period_start_odo);

    let avg_consumption = match period_start_odo {
        //assume the car was charged to clim at the start of the first session
        0 => ((total_energy + (cap * clim)) / (odo as f64)) * 100.0,
        //if we're not taking all sessions into account, we need to start the odometer reading at the session before the period
        _ => ((total_energy) / ((odo - period_start_odo) as f64)) * 100.0,
    };

    let stats = GlobalStatistics {
        avg_consumption: ((total_energy - charged_energy) / ((odo - period_start_odo) as f64)) * 100.0,
        //this calculation assumes the battery SoC is reported linearly by the vehicle
        avg_consumption_last_charge: ((charged_energy - (cap * (clim - prev_soc / 100.0))) / (odo - prev_odo) as f64) * 100.0,
        num_sessions: query_result.session_count,
        total_distance: odo,
        distance_last_charge: odo - prev_odo,
        avg_distance_80_percent: cap / avg_consumption * 100.0 * 0.8,
        total_energy,
    };

    return Ok(HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&stats).unwrap()));
}

struct QueryResult {
    //battery capacity in kWh
    capacity: f64,
    //charge limit between 0 and 1
    charge_limit: f64,
    //the energy charged in the last session in kWh
    charged_energy: f64,
    //the odometer reading at the end of the last session
    odometer: i32,
    //the SoC at the end of the last session
    prev_soc: f64,
    //the odometer reading at the end of the second last session
    prev_odo: i32,
    //the odometer reading at the start of the period
    period_start_odo: i32,
    total_energy: f64,
    session_count: i64,
}

fn query_data(ctx: Data<Context>, path_id: &i32, period: Sub<now, Bound<Interval, PgInterval>>, prev: bool) -> Result<QueryResult, > {
    let conn = &mut ctx.clone().get_pool().get().unwrap();

    let vehicle = vehicles
        .filter(id.eq(path_id))
        .select(Vehicle::as_select())
        .load(conn)
        .expect("Failed to retrieve Vehicle data");

    //get all sessions for the given period
    let results = charge_sessions
        .filter(vehicle_id.eq(path_id))
        .filter(date.gt(period))
        .offset(if prev { 1 } else { 0 })
        .select(ChargeSession::as_select())
        .order(date.desc())
        .load(conn)
        .expect("Failed to retrieve sessions");

    //throw an error if less than 2 sessions are available
    if results.len() < 2 {
        //todo this should be mappable to HTTP status code
        return Err(Error::new(ErrorKind::NotFound, "Not enough data available to calculate statistics"));
    }

    //get last session before period
    //only used when period is not "all"
    //tbd if it's worth to skip the query for performance reasons when period is "all"
    let prev_session = charge_sessions
        .filter(vehicle_id.eq(path_id))
        .filter(date.lt(period))
        .select(ChargeSession::as_select())
        .order(date.desc())
        .first::<ChargeSession>(conn)
        .optional()
        .expect("Failed to retrieve sessions");

    //get the total number of sessions
    let mut session_count = charge_sessions
        .filter(vehicle_id.eq(path_id))
        .filter(date.gt(period))
        .count()
        .get_result(conn)
        .expect("Failed to retrieve sessions");

    if prev {
        session_count -= 1;
    }

    //execute query: select COALESCE(SUM(energy), 0) from charge_sessions where vehicle_id=?;
    let total_energy = match prev {
        true => {
            let last_charge_session_id = charge_sessions
                .filter(vehicle_id.eq(path_id))
                .order(date.desc())
                .select(session_id)
                .first::<i32>(conn)
                .expect("Failed to retrieve sessions");

            charge_sessions
            .filter(vehicle_id.eq(path_id))
            .filter(date.gt(period))
            .filter(
                not(
                    session_id.eq(last_charge_session_id)
                )
            )
            .select(diesel::dsl::sum(energy))
            .first::<Option<f64>>(conn)
            .expect("Failed to retrieve sessions")
            .unwrap_or(0.0)
        },
        false => charge_sessions
            .filter(vehicle_id.eq(path_id))
            .filter(date.gt(period))
            .select(diesel::dsl::sum(energy))
            .first::<Option<f64>>(conn)
            .expect("Failed to retrieve sessions")
            .unwrap_or(0.0)
    };

    return Ok(QueryResult {
        capacity: vehicle.get(0).unwrap().battery_capacity as f64,
        charge_limit: vehicle.get(0).unwrap().charge_limit,
        charged_energy: results.get(0).unwrap().energy,
        odometer: results.get(0).unwrap().odometer,
        prev_soc: results.get(1).unwrap().end_soc as f64,
        prev_odo: results.get(1).unwrap().odometer,
        period_start_odo: prev_session.map_or(results.get(results.len() - 1).unwrap().odometer, |s| s.odometer),
        total_energy,
        session_count,
    });
}