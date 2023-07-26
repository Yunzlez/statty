/*
   Copyright 2023 Frederik Van Herbruggen

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
 */
use actix_cors::Cors;
use actix_web::{App, HttpServer};
use actix_web::web::Data;
use dotenvy::dotenv;
use statty_common::context::Context;
use statty_db::db_conn::get_db_pool;
use statty_routes::routes::config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin();

        App::new()
            .wrap(cors)
            .configure(|cfg| config(cfg))
            .app_data(Data::new(Context::new_context(get_db_pool())))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}