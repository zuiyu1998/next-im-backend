use crate::config::Config;

pub fn tracing_subscriber_init(config: &Config) {
    tracing_subscriber::fmt()
        .with_max_level(config.log.level.level())
        .init();
}
