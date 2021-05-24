#[macro_use]
extern crate serde_derive;

mod mods;

use actix_web::{middleware, App, HttpServer};
use env_logger::Env;
use mods::{config_controllers, settings::Settings};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Settings::new();
    env_logger::Builder::from_env(Env::default().default_filter_or(config.log.get_level())).init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .service(config_controllers::get_config)
            .service(config_controllers::ping)
    })
    .bind(config.server.get_addr())?
    .run()
    .await
}
