use std::time::SystemTime;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Queryable,Selectable)]
#[diesel(table_name = crate::schema::charge_sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ChargeSession {
    pub id: i32,
    pub date: SystemTime,
    pub vehicle_id: i32,
    pub end_soc: i32,
    pub energy: f64,
    pub odometer: i32
}

#[derive(Serialize,Deserialize,Insertable)]
#[diesel(table_name = crate::schema::charge_sessions)]
pub struct NewChargeSession {
    pub date: Option<SystemTime>,
    pub vehicle_id: i32,
    pub end_soc: i32,
    pub energy: f64,
    pub odometer: i32
}