use diesel::prelude::*;
use serde::{Serialize,Deserialize};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::vehicles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize,Deserialize)]
pub struct Vehicle {
    pub id: i32,
    pub name: String,
    pub battery_capacity: i32,
    pub charge_limit: f64
}


