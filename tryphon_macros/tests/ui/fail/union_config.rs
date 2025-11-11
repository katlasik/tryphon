use tryphon::Config;

#[derive(Config)]
union BadConfig {
    x: u32,
    y: f32,
}

fn main() {}
