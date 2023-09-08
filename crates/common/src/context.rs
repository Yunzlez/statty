use actix::{Actor, Addr};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use statty_db::db_conn::DatabaseContext;
use crate::auth::auth::AuthState;
use crate::auth::auth::DbRegistrar;

#[derive(Clone)]
pub struct Context {
    db: DatabaseContext,
    auth_state: Addr<AuthState>
}

impl Context {
    pub fn new_context(
        db: DatabaseContext
    ) -> Context {
        Context {
            db: db.clone(),
            auth_state: AuthState::preconfigured(DbRegistrar{
                db
            }).start()
        }
    }

    pub fn get_pool(&self) -> &Pool<ConnectionManager<PgConnection>> {
        return &self.db.pool
    }

    pub fn get_auth_state(&self) -> &Addr<AuthState> {
        return &self.auth_state
    }
}