use tryphon::ConfigValueDecoder;

#[derive(ConfigValueDecoder)]
enum BadLevel {
    Low,
    High(u32),  // Tuple variant
}

fn main() {}
