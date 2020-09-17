use std::env;

pub struct Environment {
    pub database_url: String
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            database_url: env::var("NAIS_DATABASE_RUST_POC_RUSTPOC_URL")
                .unwrap_or("postgres://postgres:hunter2@127.0.0.1:5432/postgres".to_string())
        }
    }
}