use std::collections::HashMap;
use std::io::Result;

use actix_web::HttpResponse;
use actix_web::web::{Data, Json, Path, Query};
use diesel::prelude::*;
use time::OffsetDateTime;

use statty_common::context::Context;
use statty_db::util::get_page_params;
use statty_domain::charge_session::{ChargeSession, ChargeSessionDto, from_dto, to_dto};
use statty_domain::meta::{PagedList, PageMeta};
use statty_domain::schema::charge_sessions::{date, vehicle_id};
use statty_domain::schema::charge_sessions::dsl::charge_sessions;

pub async fn list_sessions(ctx: Data<Context>, path: Path<i32>, query: Query<HashMap<String, String>>) -> Result<HttpResponse> {
    //todo check vehicle
    let conn = &mut ctx.clone().get_pool().get().unwrap();

    let page_params = get_page_params(query);
    let path_param = &path.into_inner();


    let total: i64 = charge_sessions
        .filter(vehicle_id.eq(path_param))
        .count()
        .get_result(conn)
        .expect("Failed to retrieve sessions");

    let results: Vec<ChargeSessionDto> = charge_sessions
        .limit(page_params.1)
        .offset(page_params.0 * page_params.1)
        .filter(vehicle_id.eq(path_param))
        .order(date.desc())
        .select(ChargeSession::as_select())
        .load(conn)
        .expect("Failed to retrieve sessions")
        .into_iter()
        .map(|it| to_dto(it))
        .collect();

    println!("Retrieved {} sessions", results.len());

    let list = PagedList {
        items: &results,
        meta: PageMeta {
            page: page_params.0,
            total_items: total,
            total_pages: ((total/page_params.1) as f64).floor() as i32
        }
    };

    return Ok(
        HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&list).unwrap())
    );
}

pub async fn add_session(ctx: Data<Context>, data: Json<ChargeSessionDto>) -> Result<HttpResponse> {
    //todo check vehicle
    let conn = &mut ctx.clone().get_pool().get().unwrap();

    let mut new_session = data.into_inner();
    if new_session.date.is_none() {
        new_session.date = Some(OffsetDateTime::now_utc().date());
    }

    println!("Insert {}", serde_json::to_string(&new_session).unwrap());

    let res = diesel::insert_into(charge_sessions)
        .values(from_dto(new_session))
        .returning(ChargeSession::as_returning())
        .get_result(conn)
        .expect("Failed to save charge session");

    return Ok(
        HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&to_dto(res)).unwrap())
    );
}
