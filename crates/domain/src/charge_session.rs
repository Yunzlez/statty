use time::Date;
use diesel::prelude::*;
use serde::{Serialize,Deserialize};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::charge_sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize,Deserialize)]
pub struct ChargeSession {
    pub id: i32,
    pub date: Date,
    pub vehicle_id: i32,
    pub end_soc: i32,
    pub energy: i32,
    pub odometer: i32
}