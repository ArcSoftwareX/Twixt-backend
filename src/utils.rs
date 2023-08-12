pub fn log_init() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
}

pub fn storage_init() -> std::io::Result<()> {
    std::fs::create_dir_all("./storage/image")?;
    std::fs::create_dir_all("./storage/video")?;

    Ok(())
}
