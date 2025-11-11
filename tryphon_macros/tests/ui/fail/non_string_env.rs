use tryphon::Config;

#[derive(Config)]
struct BadConfig {
    #[env(123)]
    port: u16,
}

fn main() {}
