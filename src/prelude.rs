#[macro_export]
macro_rules! env {
    ($name:literal) => {{
        std::env::var($name)
    }};
}
