use diesel::prelude::*;
use serde::{Serialize,Deserialize};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize,Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String
}


