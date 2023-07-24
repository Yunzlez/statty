use std::io::Result;
use std::time::SystemTime;

use actix_web::HttpResponse;
use actix_web::web::{Data, Json};
use diesel::prelude::*;

use statty_common::context::Context;
use statty_domain::charge_session::{ChargeSession, NewChargeSession};
use statty_domain::schema::charge_sessions::dsl::charge_sessions;

pub async fn list_sessions(ctx: Data<Context>) -> Result<HttpResponse> {
    let conn = &mut ctx.clone().get_pool().get().unwrap();
    let results = charge_sessions
        .limit(100)
        .select(ChargeSession::as_select())
        .load(conn)
        .expect("Failed to retrieve sessions");

    println!("Retrieved {} sessions", results.len());
    return Ok(HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&results).unwrap()));
}

pub async fn add_session(ctx: Data<Context>, data: Json<NewChargeSession>) -> Result<HttpResponse> {
    let conn = &mut ctx.clone().get_pool().get().unwrap();

    let mut new_session = data.into_inner();
    if new_session.date.is_none() {
        new_session.date = Some(SystemTime::now());
    }

    println!("Insert {}", serde_json::to_string(&new_session).unwrap());

    let res = diesel::insert_into(charge_sessions)
        .values(new_session)
        .returning(ChargeSession::as_returning())
        .get_result(conn)
        .expect("Failed to save charge session");

    return Ok(HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&res).unwrap()));
}
