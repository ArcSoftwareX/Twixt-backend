use crate::env;

#[derive(Clone)]
pub struct Config {
    pub db_url: String,
    pub jwt_secret: String,
    // pub jwt_expires_in: String,
    // pub jwt_maxage: i32,
}

impl Config {
    pub fn init() -> anyhow::Result<Self> {
        let db_url = env!("DATABASE_URL")?;
        let jwt_secret = env!("JWT_SECRET")?;
        // let jwt_expires_in = env!("JWT_EXPIRES_IN")?;
        // let jwt_maxage = env!("JWT_MAXAGE")?.parse::<i32>()?;

        Ok(Self {
            db_url,
            // jwt_expires_in,
            // jwt_maxage,
            jwt_secret,
        })
    }
}
