use tryphon::Config;

#[derive(Config)]
struct BadConfig {
    #[env("PORT")]
    #[default(some_function())]
    port: u16,
}

fn main() {}
