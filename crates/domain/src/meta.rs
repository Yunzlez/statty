use serde::{Serialize};

#[derive(Serialize)]
pub struct PageMeta {
    pub page: i64,
    pub total_items: i64,
    pub total_pages: i32,
}

#[derive(Serialize)]
pub struct PagedList<'a, T> {
    pub items: &'a Vec<T>,
    pub meta: PageMeta
}