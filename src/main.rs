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
use actix_web::http::header;
use actix_web::web::Data;
use dotenvy::dotenv;
use log::info;
use statty_common::context::Context;
use statty_db::db_conn::{get_db_ctx, run_migrations};
use statty_routes::routes::config;
use statty_config::config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let app_config = Config::from_env().expect("Unable to load config");
    env_logger::init();

    info!("Starting statty");
    run_migrations().expect("Unable to run migrations");
    info!("Finished DB migrations successfully");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![header::CONTENT_TYPE, header::ACCEPT, header::AUTHORIZATION])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .configure(|cfg| config(cfg))
            .app_data(Data::new(Context::new_context(get_db_ctx(app_config.db_url.clone()))))
    })
        .bind(("0.0.0.0", app_config.port))?
        .run()
        .await
}