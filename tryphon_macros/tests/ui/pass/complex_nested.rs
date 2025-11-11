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
struct CacheConfig {
    #[env("CACHE_HOST")]
    host: String,

    #[env("CACHE_TTL")]
    #[default(300)]
    ttl: u32,
}

#[derive(Config)]
struct ServerConfig {
    #[config]
    database: DatabaseConfig,

    #[config]
    cache: CacheConfig,

    #[env("SERVER_PORT")]
    #[default(8080)]
    port: u16,

    #[env("DEBUG")]
    debug: Option<bool>,
}

fn main() {}
