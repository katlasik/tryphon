use tryphon::Config;

#[derive(Config)]
struct ValidConfig {
    #[env("PORT")]
    #[default(8080)]
    port: u16,

    #[env("HOST")]
    #[default("localhost")]
    host: String,
}

fn main() {}
