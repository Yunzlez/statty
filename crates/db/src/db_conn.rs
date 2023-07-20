use std::env;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;

pub fn get_db_conn() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Failed to connect to database"))
}