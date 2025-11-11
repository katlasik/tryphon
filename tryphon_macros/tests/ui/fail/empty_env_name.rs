use tryphon::Config;

#[derive(Config)]
struct BadConfig {
    #[env("")]
    host: String,
}

fn main() {}
