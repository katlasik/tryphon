use tryphon::Config;

#[derive(Config)]
struct ValidConfig {
    #[env("APP_PORT")]
    #[env("PORT")]
    #[env("DEFAULT_PORT")]
    port: u16,
}

fn main() {}
