use std::collections::HashMap;
use actix_web::web::Query;

pub fn get_page_params(query: Query<HashMap<String, String>>) -> (i64, i64) {
    let page = (query.get("page").unwrap_or(&"0".to_string())).parse::<i64>().unwrap();
    let mut limit = query.get("limit").unwrap_or(&"10".to_string()).parse::<i64>().unwrap();
    if limit > 1000 {
        limit = 1000;
    }

    return (page, limit);
}