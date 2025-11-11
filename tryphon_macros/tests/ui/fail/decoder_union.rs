use tryphon::ConfigValueDecoder;

#[derive(ConfigValueDecoder)]
union BadUnion {
    x: u32,
    y: f32,
}

fn main() {}
