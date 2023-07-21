use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

#[derive(Clone)]
pub struct Context {
    db_pool: Pool<ConnectionManager<PgConnection>>
}

impl Context {
    pub fn new_context(
        db_pool: Pool<ConnectionManager<PgConnection>>
    ) -> Context {
        Context {
            db_pool
        }
    }

    pub fn get_pool(&self) -> &Pool<ConnectionManager<PgConnection>> {
        return &self.db_pool
    }
}