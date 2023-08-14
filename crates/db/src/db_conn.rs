use std::env;
use std::error::Error;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../../migrations");

pub fn get_db_conn() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Failed to connect to database"))
}

pub fn get_db_pool() -> Pool<ConnectionManager<PgConnection>> {

    let mgr = ConnectionManager::<PgConnection>::new(env::var("DATABASE_URL").expect("DATABASE_URL must be set"));

    Pool::builder()
        .test_on_check_out(true)
        .max_size(5)
        .build(mgr)
        .expect("Failed to connect to database")
}

pub fn run_migrations() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let conn = &mut get_db_conn();
    conn.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}