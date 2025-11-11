use tryphon::Config;

#[derive(Config)]
struct BadConfig {
    // Missing both #[env] and #[config]
    host: String,
}

fn main() {}
