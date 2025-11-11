use tryphon::Config;

#[derive(Config)]
struct ValidConfig {
    #[env("REQUIRED")]
    required: String,

    #[env("OPTIONAL")]
    optional: Option<String>,
}

fn main() {}
