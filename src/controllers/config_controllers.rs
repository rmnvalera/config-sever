use std::{fmt::Debug, fs::File};

use actix_web::{get, http::StatusCode, web, HttpResponse};
use serde_yaml::Value;

use crate::mods::utils;

#[derive(Deserialize, Debug)]
struct GetConfigParams {
    client_configuration_version: Option<String>,
    client_id: Option<String>,
}

struct ResponseOfMessageError {
    ststus: StatusCode,
    message: Option<String>,
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

    let config = match read_config_file(&config_path) {
        Ok(value) => value,
        Err(e) => {
            return HttpResponse::Ok().status(e.ststus).body(e.message.unwrap());
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

    let version_hash = utils::calculate_hash(&string_config);
    log::info!("Hash config: {}", version_hash);
    HttpResponse::Ok()
        .header("Configuration-Version", version_hash)
        .body(string_config)
}

fn read_config_file(config_path: &str) -> Result<Value, ResponseOfMessageError> {
    let file = match File::open(config_path) {
        Ok(file) => file,
        Err(_) => {
            return Err(ResponseOfMessageError {
                ststus: StatusCode::NOT_FOUND,
                message: Some(String::from("Config not found!!")),
            });
        }
    };

    let configs_value = match serde_yaml::from_reader(&file) {
        Ok(value) => value,
        Err(e) => {
            log::info!("{}", e);
            return Err(ResponseOfMessageError {
                ststus: StatusCode::INTERNAL_SERVER_ERROR,
                message: Some(String::from("Error parse Yaml!!")),
            });
        }
    };

    Ok(configs_value)
}
