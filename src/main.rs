use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer};
use env_logger;
use log::*;

mod api;
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    openssl_probe::init_ssl_cert_env_vars();

    let config = config::load();
    info!("HTTP Server is listening for {}", config.http_addr);

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET"])
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .service(api::index::get_index)
            .service(api::watching::get_watching)
    })
    .bind(config.http_addr)?
    .run()
    .await
}
