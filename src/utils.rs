pub fn log_init() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
}
