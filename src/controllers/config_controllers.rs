use std::{collections::hash_map::DefaultHasher, fmt::Debug};
use std::hash::{Hash, Hasher};

extern crate sha1;

use actix_web::{get, web, HttpResponse};
use config::{Config, File};
use serde_yaml::Value;

#[derive(Deserialize, Debug)]
struct GetConfigParams {
    client_configuration_version: Option<String>,
    client_id: Option<String>,
}

#[get("/applications/{application}/environments/{environment}/configurations/{configuration}")]
async fn get_config(
    web::Path((application, environment, configuration)): web::Path<(String, String, String)>,
    _params: web::Query<GetConfigParams>,
) -> HttpResponse {

    let config_path = format!(
        "resurce/{}/{}/{}.yml",
        application, environment, configuration
    );

    let config = match read_config_file(&config_path){
        Ok(value) => value,
        Err(e) => {
            return HttpResponse::InternalServerError().body(e);
        }
    };

    let string_config = match serde_yaml::to_string(&config) {
        Ok(config) => config,
        Err(e) => {
            let msg = e.to_string();
            log::info!("{}", e);
            return HttpResponse::InternalServerError().body(msg);
        }
    };

    log::info!("Hash config: {}", calculate_hash(&string_config));
    HttpResponse::Ok().header("Configuration-Version", "v1").body(string_config)
}

#[get("/ping")]
async fn ping() -> HttpResponse {
    HttpResponse::Ok().body("pong - AppConfig service")
}

fn read_config_file(config_path: &str) -> Result<Value, String>{
    let mut settings = Config::default();
    let config_file = File::from(std::path::Path::new(&config_path));

    if let Err(e) = settings.merge(config_file) {
        log::info!("{}", e);
        return Err("ConfigError!".to_string());
    };

    let configs_value = match settings.try_into::<Value>() {
        Ok(value) => value,
        Err(e) => {
            log::info!("{}", e);
            return Err("Error parse Yaml!!".to_string());
        }
    };

    Ok(configs_value)
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}