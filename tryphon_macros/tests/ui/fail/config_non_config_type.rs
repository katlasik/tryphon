use tryphon::Config;

struct NotConfig {
    value: String,
}

#[derive(Config)]
struct BadConfig {
    #[config]
    nested: NotConfig,  // Doesn't implement Config
}

fn main() {}
