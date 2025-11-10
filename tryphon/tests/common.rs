use std::sync::Mutex;

pub static TEST_MUTEX: Mutex<()> = Mutex::new(());

#[macro_export]
macro_rules! clear_test_env_vars {
  ($($var:expr),* $(,)?) => {
      $(
          std::env::remove_var($var);
      )*
  };
}
