use std::error::Error;

use crate::config::Config;
use tracing_subscriber::{
    filter::{FromEnvError, ParseError},
    fmt,
    prelude::*,
    EnvFilter,
};

pub fn tracing_subscriber_init(config: &Config) {
    let default_filter = { format!("{},{}", config.log.level.level(), config.log.filter) };
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|from_env_error| {
            _ = from_env_error
                .source()
                .and_then(|source| source.downcast_ref::<ParseError>())
                .map(|parse_err| {
                    eprintln!("log failed to parse filter from env: {}", parse_err);
                });

            Ok::<EnvFilter, FromEnvError>(EnvFilter::builder().parse_lossy(&default_filter))
        })
        .unwrap();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter_layer)
        .init()
}
