use tryphon::Config;

#[derive(Config)]
struct DatabaseConfig {
    #[env("DB_HOST")]
    host: String,

    #[env("DB_PORT")]
    #[default(5432)]
    port: u16,
}

#[derive(Config)]
struct AppConfig {
    #[config]
    database: DatabaseConfig,

    #[env("APP_NAME")]
    app_name: String,
}

fn main() {}
