use tryphon::Config;

#[derive(Config)]
struct BadConfig {
    #[env("PORT")]
    #[default(8080)]
    #[default(3000)]
    port: u16,
}

fn main() {}
