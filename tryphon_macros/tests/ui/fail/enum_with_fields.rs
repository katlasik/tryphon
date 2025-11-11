use tryphon::ConfigValueDecoder;

#[derive(ConfigValueDecoder)]
enum BadLevel {
    Low,
    High { value: u32 },  // Has fields
}

fn main() {}
