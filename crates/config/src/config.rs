use std::{env, fmt::Debug};

use crate::error::{ConfigError, ConfigErrorType};

#[derive(Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub db_url: String,
}

impl Config {
    pub fn from_env() -> Result<Config, ConfigError> {
        return Ok(Config {
            port: parse_var("PORT", |it| it.parse::<u16>(), Some(8080))?,
            db_url: parse_var::<String, ConfigError>("DATABASE_URL", |it| Ok(it), None)?
        });
    }
}

fn parse_var<OUT, ERR>(name: &str, parser: fn(String) -> Result<OUT, ERR>, default: Option<OUT>) -> Result<OUT, ConfigError> {
    let res = env::var(name)
        .map_err(|_| ConfigError {
            name: String::from(name),
            message: String::from("Value is required"),
            reason: ConfigErrorType::NotPresent, 
        })
        .and_then(|it| parser(it).map_err(|_| ConfigError {
            name: String::from(name),
            message: String::from("value is invalid"),
            reason: ConfigErrorType::Invalid,
        }));

    return match res {
        Ok(it) => Ok(it),
        Err(err) => match default {
            Some(it) => Ok(it),
            None => Err(err),
        }
    }
}
