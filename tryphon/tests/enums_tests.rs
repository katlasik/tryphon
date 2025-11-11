mod common;

use common::TEST_MUTEX;
use std::env;
use std::net::Ipv4Addr;
use tryphon::Config;

#[derive(Debug, PartialEq, Config)]
enum MessagingConfig {
    Kafka {
        #[env("KAFKA_BROKER")]
        broker: Ipv4Addr,
    },
    Pulsar {
        #[env("PULSAR_BROKER")]
        broker: Ipv4Addr,
    },
    Mock {
        #[env("MOCK_BROKER")]
        #[default(Ipv4Addr::new(1, 1, 1, 1))]
        broker: Ipv4Addr,
    },
}

fn clear_test_env_vars() {
    unsafe {
        clear_test_env_vars!("KAFKA_BROKER", "PULSAR_BROKER");
    }
}

#[test]
fn test_enum() {
    let _unused = TEST_MUTEX.lock().unwrap();

    clear_test_env_vars();

    unsafe {
        env::set_var("KAFKA_BROKER", "127.0.0.1");
    }

    matches!(MessagingConfig::load(), Ok(MessagingConfig::Kafka { broker }) if broker == Ipv4Addr::new(127, 0, 0, 1));

    clear_test_env_vars();

    unsafe {
        env::set_var("PULSAR_BROKER", "192.168.1.1");
    }

    matches!(MessagingConfig::load(), Ok(MessagingConfig::Pulsar { broker }) if broker == Ipv4Addr::new(192, 168, 1, 1));

    clear_test_env_vars();

    assert_eq!(
        MessagingConfig::load().unwrap(),
        MessagingConfig::Mock {
            broker: Ipv4Addr::new(1, 1, 1, 1)
        }
    );
}
