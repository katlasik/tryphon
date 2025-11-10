use crate::common::TEST_MUTEX;
use rand::Rng;
use std::sync::{Arc, Barrier};
use std::thread;
use tryphon::*;

mod common;

#[derive(Config, Debug)]
struct TestConfig {
    #[env("FOO")]
    foo: String,

    #[env("BAZ")]
    baz: String,
}

fn clear_test_env_vars() {
    unsafe {
        clear_test_env_vars!("FOO", "BAZ");
    }
}

fn random_string() -> String {
    let mut rand = rand::rng();
    (0..100).map(|_| rand.random_range('a'..='z')).collect()
}

#[test]
#[env_vars(FOO = "bar", BAZ = "qux")]
fn test_implicit_overrides() {
    let _unused = TEST_MUTEX.lock().unwrap();

    unsafe {
        std::env::set_var("FOO", "bad");
        std::env::set_var("BAZ", "bad");
    }

    let config = TestConfig::load().expect("Failed to load test config");

    clear_test_env_vars();

    assert_eq!(config.foo, "bar");
    assert_eq!(config.baz, "qux");
}

#[test]
fn test_explicit_overrides() {
    let _unused = TEST_MUTEX.lock().unwrap();

    let mut test_overrides = EnvOverrides::init();

    unsafe {
        std::env::set_var("FOO", "bad");
        std::env::set_var("BAZ", "bad");
    }

    test_overrides.set("FOO", "bar");
    test_overrides.set("BAZ", "qux");

    let config = TestConfig::load().expect("Failed to load test config");

    assert_eq!(config.foo, "bar");
    assert_eq!(config.baz, "qux");

    test_overrides.set("FOO", "baz").set("BAZ", "quux");

    let config = TestConfig::load().expect("Failed to load test config");

    assert_eq!(config.foo, "baz");
    assert_eq!(config.baz, "quux");

    clear_test_env_vars();
}

#[test]
fn test_concurrency() {
    let n = 1024;
    let start = Arc::new(Barrier::new(n));

    let mut handles = Vec::new();
    for _ in 0..n {
        let start = Arc::clone(&start);
        handles.push(thread::spawn(move || {
            start.wait();

            let mut test_overrides = EnvOverrides::init();

            let foo = random_string();
            let baz = random_string();

            test_overrides
                .set("FOO", foo.as_str())
                .set("BAZ", baz.as_str());

            let config = TestConfig::load().expect("Failed to load test config");

            assert_eq!(config.foo, foo);
            assert_eq!(config.baz, baz);
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
}

#[should_panic(
    expected = "TestOverrides already initialized. You must not create multiple instances of TestOverrides for single thread."
)]
#[test]
fn test_panic_if_there_are_multiple_env_overrides() {
    let mut overrides = EnvOverrides::init();
    let _ = EnvOverrides::init();

    overrides.set("FOO", "bar");
}
