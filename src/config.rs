use std::env;

pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub host: String,
    pub port: u16,
    pub s3: S3Config,
}

pub struct S3Config {
    pub endpoint: String,
    pub bucket: String,
    pub access_key: String,
    pub secret_key: String,
    pub region: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .expect("PORT must be a number"),
            s3: S3Config {
                endpoint: env::var("S3_ENDPOINT").expect("S3_ENDPOINT must be set"),
                bucket: env::var("S3_BUCKET").expect("S3_BUCKET must be set"),
                access_key: env::var("S3_ACCESS_KEY").expect("S3_ACCESS_KEY must be set"),
                secret_key: env::var("S3_SECRET_KEY").expect("S3_SECRET_KEY must be set"),
                region: env::var("S3_REGION").unwrap_or_else(|_| "us-east-1".to_string()),
            },
        }
    }
}
