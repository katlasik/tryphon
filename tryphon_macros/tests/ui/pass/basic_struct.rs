use tryphon::Config;

#[derive(Config)]
struct ValidConfig {
    #[env("HOST")]
    host: String,
}

fn main() {}
