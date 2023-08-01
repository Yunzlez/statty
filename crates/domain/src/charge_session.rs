use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use diesel::prelude::*;
use diesel::{QueryId};
use serde::{Deserialize, Serialize};
use statty_common::date_format;

#[derive(Queryable,Selectable,QueryId)]
#[diesel(table_name = crate::schema::charge_sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ChargeSession {
    pub id: i32,
    pub date: NaiveDateTime,
    pub vehicle_id: i32,
    pub end_soc: i32,
    pub energy: f64,
    pub odometer: i32
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::charge_sessions)]
pub struct NewChargeSession {
    pub date: Option<NaiveDateTime>,
    pub vehicle_id: i32,
    pub end_soc: i32,
    pub energy: f64,
    pub odometer: i32
}

#[derive(Serialize,Deserialize)]
pub struct ChargeSessionDto {
    pub id: Option<i32>,
    #[serde(default)]
    #[serde(with = "date_format")]
    pub date: Option<NaiveDate>,
    pub vehicle_id: i32,
    pub end_soc: i32,
    pub energy: f64,
    pub odometer: i32
}

pub fn from_dto(dto: ChargeSessionDto) -> NewChargeSession {
    NewChargeSession {
        date: dto.date.map(|it| it.and_time(NaiveTime::MIN)),
        vehicle_id: dto.vehicle_id,
        end_soc: dto.end_soc,
        energy: dto.energy,
        odometer: dto.odometer
    }
}

pub fn to_dto(session: ChargeSession) -> ChargeSessionDto {
    ChargeSessionDto {
        id: Some(session.id),
        //date: Some(session.date.with_time(Time::MIDNIGHT).assume_offset(offset!(UTC))),
        date: Some(session.date.date()),
        vehicle_id: session.vehicle_id,
        end_soc: session.end_soc,
        energy: session.energy,
        odometer: session.odometer,
    }
}