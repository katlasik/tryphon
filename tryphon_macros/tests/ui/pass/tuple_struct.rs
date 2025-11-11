use tryphon::Config;

#[derive(Config)]
struct TupleConfig(
    #[env("FIRST")] String,
    #[env("SECOND")] i32,
    #[env("THIRD")] bool,
);

fn main() {}
