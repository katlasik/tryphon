use tryphon::Config;

#[derive(Config)]
struct NestedConfig {
    #[env("VALUE")]
    value: String,
}

#[derive(Config)]
struct BadConfig {
    #[config]
    nested: Option<NestedConfig>,  // Invalid
}

fn main() {}
