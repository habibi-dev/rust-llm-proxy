use crate::seed::run_all;
use anyhow::{Context, Result};
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::{env, fs};

#[derive(Clone)]
pub struct Config {
    pub domain: String,
    pub final_domain: String,
    pub host: String,
    pub https: bool,
    pub port: u16,
    pub database_url: String,
    pub hmac: String,
}

impl Config {
    pub fn create_env_file() {
        let env_path = ".env";
        const ENV_SAMPLE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/.env.example"));

        if !Path::new(env_path).exists() {
            match fs::write(env_path, ENV_SAMPLE) {
                Ok(_) => {
                    println!(".env file created from sample");

                    #[cfg(unix)]
                    {
                        if let Ok(meta) = fs::metadata(env_path) {
                            let mut perms = meta.permissions();
                            perms.set_mode(0o600);
                            if let Err(e) = fs::set_permissions(env_path, perms) {
                                eprintln!("Failed to set permissions: {}", e);
                            }
                        }
                    }
                }
                Err(err) => eprintln!("Failed to create .env file: {}", err),
            }
        }

        if let Err(e) = dotenvy::dotenv() {
            eprintln!("Error loading .env: {}", e);
        }
    }

    pub async fn setup_database() -> Result<sea_orm::DatabaseConnection> {
        let db_url = Self::database_url();

        let db = Database::connect(&db_url)
            .await
            .context("Failed to connect to database")?;

        Migrator::up(&db, None)
            .await
            .context("Database migration failed")?;

        run_all(&db).await?;

        Ok(db)
    }

    pub fn load() -> Self {
        Self::create_env_file();
        Self {
            host: Self::app_host(),
            https: Self::app_https(),
            port: Self::app_port(),
            database_url: Self::database_url(),
            domain: Self::domain(),
            final_domain: Self::final_domain(),
            hmac: Self::app_hmac(),
        }
    }

    fn app_https() -> bool {
        env::var("APP_HTTPS")
            .unwrap_or_else(|_| "false".into())
            .parse()
            .unwrap_or(false)
    }

    fn app_hmac() -> String {
        env::var("HMAC_KEY").unwrap_or_default()
    }

    fn app_host() -> String {
        env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".into())
    }

    fn final_domain() -> String {
        env::var("APP_FINAL_DOMAIN").unwrap_or_else(|_| "localhost".into())
    }

    fn domain() -> String {
        env::var("APP_DOMAIN").unwrap_or_else(|_| "localhost".into())
    }

    fn database_url() -> String {
        env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://database.db?mode=rwc".into())
    }

    fn app_port() -> u16 {
        env::var("APP_PORT")
            .unwrap_or_else(|_| "8080".into())
            .parse()
            .unwrap_or(8080)
    }
}
