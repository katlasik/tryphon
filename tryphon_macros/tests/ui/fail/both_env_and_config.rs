use tryphon::Config;

#[derive(Config)]
struct BadConfig {
    #[env("DB_HOST")]
    #[config]
    database: String,
}

fn main() {}
