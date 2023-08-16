use actix::{Actor, Addr};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::auth::AuthState;

#[derive(Clone)]
pub struct Context {
    db_pool: Pool<ConnectionManager<PgConnection>>,
    auth_state: Addr<AuthState>
}

impl Context {
    pub fn new_context(
        db_pool: Pool<ConnectionManager<PgConnection>>
    ) -> Context {
        Context {
            db_pool,
            auth_state: AuthState::preconfigured().start()
        }
    }

    pub fn get_pool(&self) -> &Pool<ConnectionManager<PgConnection>> {
        return &self.db_pool
    }

    pub fn get_auth_state(&self) -> &Addr<AuthState> {
        return &self.auth_state
    }
}